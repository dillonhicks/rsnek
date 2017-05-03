use std::borrow::Borrow;

use rsnek_compile::{
    Ast, Module, Stmt, Expr, Op, Lexer,
    LexResult, Parser, ParserResult,
    OwnedTk, Id};

use rsnek_compile::fmt;
use ::error::Error;
use ::opcode::OpCode;
use ::typedef::native::{self, Instr, Native};

pub type CompilerResult = Result<Box<[Instr]>, Error>;


#[derive(Debug, Copy, Clone, Serialize)]
pub enum Context {
    Load,
    Store
}


#[derive(Debug, Clone, Serialize)]
pub struct Compiler<'a>{
    lexer: Lexer,
    parser: Parser<'a>
}




impl<'a> Compiler<'a> {
    pub fn new() -> Self {
        Compiler {
            lexer: Lexer::new(),
            parser: Parser::new(),
        }
    }

    pub fn compile_str<'b>(&mut self, input: &'b str) -> CompilerResult {
        let mut parser = Parser::new();

        let tokens = match self.lexer.tokenize2(input.as_bytes()) {
            LexResult::Done(left, ref tokens) if left.len() == 0 => tokens.clone(),
            _ => return Err(Error::syntax("Could not tokenize input"))
        };

        match parser.parse_tokens(&tokens) {
            ParserResult::Ok(ref result) if result.remaining_tokens.len() == 0 => {
                self.compile_ast(&result.ast.borrow())
            },
            other => {
                trace!("Parser"; "Result" => fmt::json(&other));
                Err(Error::syntax("Could not parse input"))
            }
        }
    }

    // Ast Compiler Methods

    pub fn compile_ast(&self, ast: &Ast) -> CompilerResult{
        //println!("CompileAST({:?})", ast);
        let mut instructions: Vec<Instr> = vec![];

        let result = match *ast {
            Ast::Module(ref module) => {
                self.compile_module(module)
            },
            _ => Ok(Box::default())
        };

        instructions.append(&mut result?.to_vec());
        Ok(instructions.into_boxed_slice())
    }

    pub fn compile_module(&self, module: &Module) -> CompilerResult {
        //println!("CompileModule({:?})", module);

        let mut instructions: Vec<Instr> = vec![];

        match *module {
            Module::Body(ref stmts) => {

                for stmt in stmts {
                    let ins = self.compile_stmt(&stmt);
                    instructions.append(&mut ins?.to_vec());
                }
            }
        }

        Ok(instructions.into_boxed_slice())
    }

    #[allow(unused_variables)]
    fn compile_stmt(&self, stmt: &'a Stmt) -> CompilerResult {
        let mut instructions: Vec<Instr> = vec![];

        let ins: Box<[Instr]> = match *stmt {
            Stmt::FunctionDef {fntype: _, ref name, ref arguments, ref body } => {
                let mut argnames: Vec<String> = Vec::new();
                for arg in arguments {
                    match arg {
                        &Expr::Constant(ref owned_tk) => argnames.push(owned_tk.as_string()),
                        _ => return Err(Error::system(&format!(
                            "Unreachable code executed at line: {}", line!())))
                    }
                };

                let stmt =  self.compile_stmt(body)?;

                let code = native::Code {
                    co_name: name.as_string(),
                    co_names: argnames.iter().cloned().collect::<Vec<_>>(),
                    co_varnames: Vec::new(),
                    co_code: stmt.to_vec(),
                };

                let func_ins: Vec<Instr> = vec![
                    Instr(OpCode::LoadConst, Some(Native::Code(code))),
                    Instr(OpCode::LoadConst, Some(Native::from(name))),
                    Instr(OpCode::MakeFunction, None),
                    Instr(OpCode::StoreName, Some(Native::from(name)))
                ];

                func_ins.into_boxed_slice()
            },
            Stmt::Block(ref stmts) => {
                let mut block_ins: Vec<Instr> = vec![];

                for stmt in stmts.iter().as_ref() {
                    block_ins.append(&mut self.compile_stmt(&stmt)?.to_vec());
                }

                block_ins.into_boxed_slice()
            },
            Stmt::Return(Some(ref value)) => {
                let mut return_ins: Vec<Instr> = vec![];
                return_ins.append(&mut self.compile_expr(&value, Context::Load)?.to_vec());
                return_ins.push(Instr(OpCode::ReturnValue, None));
                return_ins.into_boxed_slice()
            },
            Stmt::Return(None) => {
                let return_ins: Vec<Instr> = vec![
                    Instr(OpCode::LoadName, Some(Native::from("None"))),
                    Instr(OpCode::ReturnValue, None)
                ];
                return_ins.into_boxed_slice()
            }
            Stmt::Assign { ref target, ref value } => self.compile_stmt_assign(target, value)?,
            Stmt::Expr(ref expr) => {
                let mut ins = self.compile_expr(expr, Context::Load)?.to_vec();
                ins.push(Instr(OpCode::PopTop, None));
                ins.into_boxed_slice()
            },
            Stmt::Assert { ref test, ref message } => {
                let mut ins: Vec<Instr> = Vec::new();
                ins.append(&mut self.compile_expr(test, Context::Load)?.to_vec());
                let args = match *message {
                    Some(ref expr) => {
                        ins.append(&mut self.compile_expr(expr, Context::Load)?.to_vec());
                        Native::Count(2)
                    },
                    None => Native::Count(1)
                };

                ins.push(Instr(OpCode::AssertCondition, Some(args)));
                ins.into_boxed_slice()
            },
            Stmt::Delete(_)                     => {Box::default()},
            Stmt::AugAssign {ref target, ref op, ref value} => {Box::default()},
            Stmt::ClassDef {ref name, ref bases, ref body}  => {Box::default()},
            Stmt::Newline                       => {Box::default()},
            Stmt::Import                        => {Box::default()},
            Stmt::ImportFrom                    => {Box::default()},
            Stmt::Global(_)                     => {Box::default()},
            Stmt::Nonlocal(_)                   => {Box::default()},
            Stmt::Pass                          => {Box::default()},
            Stmt::Break                         => {Box::default()},
            Stmt::Continue                      => {Box::default()},
        };

        instructions.append(&mut ins.to_vec());
        Ok(instructions.into_boxed_slice())
    }

    fn compile_stmt_assign(&self, target: &'a Expr, value: &'a Expr) -> CompilerResult {
        // println!("CompileAssignment(target={:?}, value={:?})", target, value);
        let mut instructions: Vec<Instr> = vec![];

        let ins: Box<[Instr]> = self.compile_expr(value, Context::Load)?;
        instructions.append(&mut ins.to_vec());

        let ins: Box<[Instr]> = self.compile_expr(target, Context::Store)?;
        instructions.append(&mut ins.to_vec());

        Ok(instructions.into_boxed_slice())
    }


    #[allow(unused_variables)]
    fn compile_expr(&self, expr: &'a Expr, ctx: Context) -> CompilerResult {
        let mut instructions: Vec<Instr> = vec![];

        let ins: Box<[Instr]> = match *expr {
            Expr::NameConstant(ref tk)  |
            Expr::Constant(ref tk)      => {
                self.compile_expr_constant(ctx, tk)?
            },
            Expr::BinOp {ref op, ref left, ref right} => {
                self.compile_expr_binop(op, left, right)?
            },
            Expr::Call {ref func, ref args, keywords: _} => {
                self.compile_expr_call(func, args)?
            },
            Expr::UnaryOp {ref op, ref operand} => {
                return Err(Error::system(&format!(
                    "Compiler does not implement Unary operator expressions; file: {}, line: {}",
                    file!(), line!())))
            },
            Expr::Lambda {ref arguments, ref body } => {
                return Err(Error::system(&format!(
                    "Compiler does not implement Lambda expressions; file: {}, line: {}",
                    file!(), line!())))
            },
            Expr::Conditional {ref condition, ref consequent, ref alternative} => {
                return Err(Error::system(&format!(
                    "Compiler does not implement Conditional expressions; file: {}, line: {}",
                    file!(), line!())))
            },
            Expr::Attribute {ref value, ref attr} => {
                return Err(Error::system(&format!(
                    "Compiler does not implement Attribute expressions; file: {}, line: {}",
                    file!(), line!())))
            },
            Expr::List {ref elems} => {
                self.compile_expr_list(elems)?
            },
            Expr::None => return Err(Error::system(&format!(
                "Unreachable code executed at line: {}", line!())))
        };

        instructions.append(&mut ins.to_vec());
        Ok(instructions.into_boxed_slice())
    }

    fn compile_expr_call(&self, func: &'a OwnedTk, arg_exprs: &'a[Expr]) -> CompilerResult {
        let mut instructions: Vec<Instr> = vec![
            Instr(OpCode::LoadName, Some(Native::from(func)))
        ];

        for expr in arg_exprs.iter().as_ref() {
            instructions.append(&mut self.compile_expr(&expr, Context::Load)?.to_vec());
        }

        instructions.push(
                Instr(OpCode::CallFunction, Some(Native::Count(arg_exprs.len())))
        );

        Ok(instructions.into_boxed_slice())
    }

    fn compile_expr_binop(&self, op: &'a Op, left: &'a Expr, right: &'a Expr) -> CompilerResult {
        let mut instructions: Vec<Instr> = vec![];

        instructions.append(&mut self.compile_expr(left, Context::Load)?.to_vec());
        instructions.append(&mut self.compile_expr(right, Context::Load)?.to_vec());

        let code = match op.0.id() {
            Id::Is              => Instr(OpCode::CompareIs, None),
            Id::IsNot           => Instr(OpCode::CompareIsNot, None),
            Id::DoubleEqual     => Instr(OpCode::CompareEqual, None),
            Id::In              => Instr(OpCode::CompareIn, None),
            Id::NotIn           => Instr(OpCode::CompareNotIn, None),
            Id::NotEqual        => Instr(OpCode::CompareNotEqual, None),
            Id::LeftAngle       => Instr(OpCode::CompareLess, None),
            Id::LessOrEqual     => Instr(OpCode::CompareLessOrEqual, None),
            Id::RightAngle      => Instr(OpCode::CompareGreater, None),
            Id::GreaterOrEqual  => Instr(OpCode::CompareGreaterOrEqual, None),
            Id::And             => Instr(OpCode::LogicalAnd, None),
            Id::Or              => Instr(OpCode::LogicalOr, None),
            Id::Plus            => Instr(OpCode::BinaryAdd, None),
            Id::Minus           => Instr(OpCode::BinarySubtract, None),
            Id::Star            => Instr(OpCode::BinaryMultiply, None),
            Id::DoubleStar      => Instr(OpCode::BinaryPower, None),
            Id::Slash           => Instr(OpCode::BinaryTrueDivide, None),
            Id::DoubleSlash     => Instr(OpCode::BinaryTrueDivide, None),
            Id::Pipe            => Instr(OpCode::BinaryOr, None),
            Id::Percent         => Instr(OpCode::BinaryModulo, None),
            Id::Amp             => Instr(OpCode::BinaryAnd, None),
            Id::At              => Instr(OpCode::BinaryMatrixMultiply, None),
            Id::Caret           => Instr(OpCode::BinaryXor, None),
            Id::LeftShift       => Instr(OpCode::BinaryLshift, None),
            Id::RightShift      => Instr(OpCode::BinaryRshift, None),
            _ =>  {
                return Err(Error::system(&format!(
                    "Compiler encountered unhandled binary operator {:?}; file: {}, line: {}",
                    op, file!(), line!())))
            }
        };

        instructions.push(code);
        Ok(instructions.into_boxed_slice())
    }

    fn compile_expr_constant(&self, ctx: Context, tk: &'a OwnedTk) -> CompilerResult {
        let instr = match ctx {
            Context::Store => {
                Instr(OpCode::StoreName, Some(Native::from(tk)))
            },
            Context::Load => {
                let code = match tk.id() {
                    Id::Name => OpCode::LoadName,
                    _ => OpCode::LoadConst
                };
                Instr(code, Some(Native::from(tk)))
            }
        };

        Ok(vec![instr].into_boxed_slice())
    }

    fn compile_expr_list(&self, elem_exprs: &'a[Expr]) -> CompilerResult {
        let mut instructions: Vec<Instr> = Vec::new();

        for expr in elem_exprs.iter().as_ref() {
            instructions.append(&mut self.compile_expr(&expr, Context::Load)?.to_vec());
        }

        instructions.push(Instr(OpCode::BuildList, Some(Native::Count(elem_exprs.len()))));
        Ok(instructions.into_boxed_slice())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rsnek_compile::fmt;

    /// Use to create a named test case of a single line snippet of code.
    /// This `basic_test!(print_function, "print('hello world!')`
    /// will create a test function named `print_function` that will try to compile the
    /// string.
    macro_rules! basic_test {
        ($name:ident, $code:expr) => {
            #[test]
            fn $name() {
               assert_compile($code);
            }
        };
    }


    fn assert_compile<'a>(text: &'a str) {
        println!("<Input>\n\n{}\n\n</Input>", text);

        let compiler = Compiler::new();
        let lexer = Lexer::new();
        let mut parser = Parser::new();

        let tokens = match lexer.tokenize2(text.as_bytes()) {
            LexResult::Done(left, ref tokens) if left.len() == 0 => tokens.clone(),
            _ => unreachable!()
        };

        println!("Tokens({}):\n----------------------------------------\n{}\n",
                 tokens.len(), fmt::tokens(&tokens, true));

        match parser.parse_tokens(&tokens) {
            ParserResult::Ok(ref result) if result.remaining_tokens.len() == 0 => {
                println!("Ast(tokens: {:?})\n{}", tokens.len(), fmt::json(result.ast.borrow()));
                let ins = compiler.compile_ast(result.ast.borrow()).unwrap();

                println!();
                println!("Compiled Instructions ({}):", ins.len());
                println!("--------------------------------");
                println!("{:#?}", ins);
                println!("{}", fmt::json(&ins))
            },
            result => panic!("\n\nERROR: {}\n\n", fmt::json(&result))
        }
    }

    #[test]
    fn compile_multiple_simple_expr() {
        assert_compile(
r#"
x = 123
y = 45
z = x + y
"#)
    }


    // Stmt::Assign(Expr::Constant)
    basic_test!(stmt_assign_int,         "x = 134567");
    basic_test!(stmt_assign_hex,         "x = 0xabdef");
    basic_test!(stmt_assign_bin,         "x = 0b01010");
    basic_test!(stmt_assign_oct,         "o = 0o12377");
    basic_test!(stmt_assign_float,       "y = 3.5");
    basic_test!(stmt_assign_complex,     "x = 6j");
    basic_test!(stmt_assign_bool,        "true = False");
    basic_test!(stmt_assign_str,         r#"z = "Zoo""#);
    basic_test!(stmt_assign_raw_str,     r#"mary = r"had a\blittle lamb\r""#);
    basic_test!(stmt_assign_byte_str,    r#"buf = b"somanybytes""#);
    basic_test!(stmt_assign_fmt_str,     r#"message = f"Hi, {name}!""#);

    // Expr::BinOp
    basic_test!(expr_binop_logicand,   "a and b");
    basic_test!(expr_binop_logicor,    "a or b");
    basic_test!(expr_binop_add,        "a + b");
    basic_test!(expr_binop_sub,        "a - b");
    basic_test!(expr_binop_mul,        "a * b");
    basic_test!(expr_binop_pow,        "a ** b");
    basic_test!(expr_binop_truediv,    "a / b");
    basic_test!(expr_binop_floordiv,   "a // b");
    basic_test!(expr_binop_or,         "a | b");
    basic_test!(expr_binop_and,        "a & b");
    basic_test!(expr_binop_xor,        "a ^ b");
    basic_test!(expr_binop_mod,        "a % b");
    basic_test!(expr_binop_matmul,     "a @ b");
    basic_test!(expr_binop_lshif,      "a << b");
    basic_test!(expr_binop_rshift,     "a >> b");


    basic_test!(multiline, r#"
x = 1
y = "somewhere over the dynamic language rainbow"
z = x + y
"#);
}
