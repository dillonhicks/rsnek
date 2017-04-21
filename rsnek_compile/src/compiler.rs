use std;
use std::ops::Deref;
use std::borrow::Borrow;
use std::str::FromStr;
use std::convert::TryFrom;

use serde::Serialize;
use nom::IResult;

use ::fmt;
use ::ast::{self, Ast, Module, Stmt, Expr, DynExpr, Op};
use ::token::{OwnedTk, Tk, Id, Tag, Num};
use ::lexer::Lexer;
use ::parser::{Parser, ParserResult, ParsedAst};


#[derive(Debug, Clone, Serialize)]
pub struct Instr(OpCode, Option<Value>);

impl Instr {
    pub fn tuple(&self) -> (OpCode, Option<Value>) {
        (self.0.clone(), self.1.clone())
    }
}

// TODO: Use rsnek_runtime::typedef::native types here
#[derive(Debug, Clone, Serialize)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64)
}


#[derive(Debug, Clone, Serialize)]
pub struct ValueError(pub String);


impl Value {
    // TODO: Refactor to use stdlib traits From / TryFrom if possible
    // TODO: unwrap() can cause panics, make this able to return a result

    fn from<'a>(tk: &'a OwnedTk) -> Self {
        let parsed = String::from_utf8(tk.bytes().to_vec()).unwrap();
        let content = parsed.as_str();

        match (tk.id(), tk.tag()) {
            (Id::Name, _)                    |
            (Id::String, _)                  => Value::Str(parsed.clone()),
            (Id::Number, Tag::N(Num::Int))   => Value::Int(content.parse::<i64>().unwrap()),
            (Id::Number, Tag::N(Num::Float)) => Value::Float(content.parse::<f64>().unwrap()),
            _ => unimplemented!()
        }
    }

}


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

    fn compile_expr_constant(&self, ctx: Context, tk: &'a OwnedTk) -> Box<[Instr]> {
        let instr = match ctx {
            Context::Store => {
                Instr(OpCode::StoreName, Some(Value::from(tk)))
            },
            Context::Load => {
                let code = match tk.id() {
                    Id::Name => OpCode::LoadName,
                    _ => OpCode::LoadConst
                };
                Instr(code, Some(Value::from(tk)))
            }
        };

        vec![instr].into_boxed_slice()
    }

    fn compile_expr_binop(&self, op: &'a Op, left: &'a Expr, right: &'a Expr) -> Box<[Instr]> {
        let mut instructions: Vec<Instr> = vec![];

        match left.deref() {
            &Expr::Constant(ref tk) => {
                let mut ins = self.compile_expr_constant(Context::Load, tk);
                instructions.append(&mut ins.to_vec());
            },
            _ => unimplemented!()
        };

        match right.deref() {
            &Expr::Constant(ref tk) => {
                let mut ins = self.compile_expr_constant(Context::Load, tk);
                instructions.append(&mut ins.to_vec());
            },
            _ => unimplemented!()
        };

        let code = match op.0.id() {
            Id::And         => Instr(OpCode::LogicalAnd, None),
            Id::Or          => Instr(OpCode::LogicalOr, None),
            Id::Plus        => Instr(OpCode::BinaryAdd, None),
            Id::Minus       => Instr(OpCode::BinarySubtract, None),
            Id::Star        => Instr(OpCode::BinaryMultiply, None),
            Id::DoubleStar  => Instr(OpCode::BinaryPower, None),
            Id::Slash       => Instr(OpCode::BinaryTrueDivide, None),
            Id::DoubleSlash => Instr(OpCode::BinaryTrueDivide, None),
            Id::Pipe        => Instr(OpCode::BinaryOr, None),
            Id::Percent     => Instr(OpCode::BinaryModulo, None),
            Id::Amp         => Instr(OpCode::BinaryAnd, None),
            Id::At          => Instr(OpCode::BinaryMatrixMultiply, None),
            Id::Caret       => Instr(OpCode::BinaryXor, None),
            Id::LeftShift   => Instr(OpCode::BinaryLshift, None),
            Id::RightShift  => Instr(OpCode::BinaryRshift, None),
            _ => panic!("{:?} is not a binary op", op)
        };

        instructions.push(code);
        instructions.into_boxed_slice()
    }

    fn compile_expr(&self, expr: &'a Expr, ctx: Context) -> Box<[Instr]> {
        let mut instructions: Vec<Instr> = vec![];

        let mut ins: Box<[Instr]> = match *expr {
            Expr::Constant(ref tk) => {
                self.compile_expr_constant(ctx, tk)
            },
            Expr::BinOp {ref op, ref left, ref right} => {
                self.compile_expr_binop(op, left, right)
            }
            _ => unimplemented!()
        };

        instructions.append(&mut ins.to_vec());
        instructions.into_boxed_slice()
    }

    fn compile_stmt_assign(&self, target: &'a Expr, value: &'a Expr) -> Box<[Instr]> {
        // println!("CompileAssignment(target={:?}, value={:?})", target, value);
        let mut instructions: Vec<Instr> = vec![];

        let mut ins: Box<[Instr]> = self.compile_expr(value, Context::Load);
        instructions.append(&mut ins.to_vec());

        let mut ins: Box<[Instr]> = self.compile_expr(target, Context::Store);
        instructions.append(&mut ins.to_vec());

        instructions.into_boxed_slice()
    }

    fn compile_stmt(&self, stmt: &'a Stmt) -> Box<[Instr]> {
        let mut instructions: Vec<Instr> = vec![];

        let mut ins: Box<[Instr]> = match *stmt {
            Stmt::Assign { ref target, ref value } => self.compile_stmt_assign(target, value),
            Stmt::Expr(ref expr) => self.compile_expr(expr, Context::Load),
            Stmt::Newline => return instructions.into_boxed_slice(),  // TODO: add lineno attrs
            _ => unimplemented!()
        };

        instructions.append(&mut ins.to_vec());
        instructions.into_boxed_slice()
    }

    pub fn compile_module(&self, module: &Module) -> Box<[Instr]> {
        //println!("CompileModule({:?})", module);

        let mut instructions: Vec<Instr> = vec![];

        match *module {
            Module::Body(ref stmts) => {

                for stmt in stmts {
                    let mut ins = self.compile_stmt(&stmt);
                    instructions.append(&mut ins.to_vec());
                }
            }
        }

        instructions.into_boxed_slice()
    }

    pub fn compile_ast(&self, ast: &Ast) -> Box<[Instr]>{
        //println!("CompileAST({:?})", ast);
        let mut instructions: Vec<Instr> = vec![];

        match *ast {
            Ast::Module(ref module) => {
                let mut ins = self.compile_module(module);
                instructions.append(&mut ins.to_vec());
            },
            _ => {}
        }

        instructions.push(Instr(OpCode::ReturnValue, None));
        instructions.into_boxed_slice()
    }

    pub fn compile_str<'b>(&mut self, input: &'b str) -> Box<[Instr]> {
        let mut parser = Parser::new();

        let tokens = match self.lexer.tokenize2(input.as_bytes()) {
            IResult::Done(left, ref tokens) if left.len() == 0 => tokens.clone(),
            _ => panic!("Issue parsing input")
        };

        let ins = match parser.parse_tokens(&tokens) {
            ParserResult::Ok(ref result) if result.remaining_tokens.len() == 0 => {
                self.compile_ast(&result.ast.borrow())
            },
            result => panic!("\n\nERROR: {:#?}\n\n", result)
        };


        ins
    }
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq, Serialize)]
#[repr(usize)]
pub enum OpCode {
    PopTop                   =   1,
    RotTwo                   =   2,
    RotThree                 =   3,
    DupTop                   =   4,
    DupTopTwo                =   5,
    Nop                      =   9,
    UnaryPositive            =  10,
    UnaryNegative            =  11,
    UnaryNot                 =  12,
    UnaryInvert              =  15,
    BinaryMatrixMultiply     =  16,
    InplaceMatrixMultiply    =  17,
    BinaryPower              =  19,
    BinaryMultiply           =  20,
    BinaryModulo             =  22,
    BinaryAdd                =  23,
    BinarySubtract           =  24,
    BinarySubscr             =  25,
    BinaryFloorDivide        =  26,
    BinaryTrueDivide         =  27,
    InplaceFloorDivide       =  28,
    InplaceTrueDivide        =  29,
    GetAiter                 =  50,
    GetAnext                 =  51,
    BeforeAsyncWith          =  52,
    InplaceAdd               =  55,
    InplaceSubtract          =  56,
    InplaceMultiply          =  57,
    InplaceModulo            =  59,
    StoreSubscr              =  60,
    DeleteSubscr             =  61,
    BinaryLshift             =  62,
    BinaryRshift             =  63,
    BinaryAnd                =  64,
    BinaryXor                =  65,
    BinaryOr                 =  66,
    InplacePower             =  67,
    GetIter                  =  68,
    GetYieldFromIter         =  69,
    PrintExpr                =  70,
    LoadBuildClass           =  71,
    YieldFrom                =  72,
    GetAwaitable             =  73,
    InplaceLshift            =  75,
    InplaceRshift            =  76,
    InplaceAnd               =  77,
    InplaceXor               =  78,
    InplaceOr                =  79,
    BreakLoop                =  80,
    WithCleanupStart         =  81,
    WithCleanupFinish        =  82,
    ReturnValue              =  83,
    ImportStar               =  84,
    YieldValue               =  86,
    PopBlock                 =  87,
    EndFinally               =  88,
    PopExcept                =  89,
    StoreName                =  90,
    DeleteName               =  91,
    UnpackSequence           =  92,
    ForIter                  =  93,
    UnpackEx                 =  94,
    StoreAttr                =  95,
    DeleteAttr               =  96,
    StoreGlobal              =  97,
    DeleteGlobal             =  98,
    LoadConst                = 100,
    LoadName                 = 101,
    BuildTuple               = 102,
    BuildList                = 103,
    BuildSet                 = 104,
    BuildMap                 = 105,
    LoadAttr                 = 106,
    CompareOp                = 107,
    ImportName               = 108,
    ImportFrom               = 109,
    JumpForward              = 110,
    JumpIfFalseOrPop         = 111,
    JumpIfTrueOrPop          = 112,
    JumpAbsolute             = 113,
    PopJumpIfFalse           = 114,
    PopJumpIfTrue            = 115,
    LoadGlobal               = 116,
    ContinueLoop             = 119,
    SetupLoop                = 120,
    SetupExcept              = 121,
    SetupFinally             = 122,
    LoadFast                 = 124,
    StoreFast                = 125,
    DeleteFast               = 126,
    RaiseVarargs             = 130,
    CallFunction             = 131,
    MakeFunction             = 132,
    BuildSlice               = 133,
    MakeClosure              = 134,
    LoadClosure              = 135,
    LoadDeref                = 136,
    StoreDeref               = 137,
    DeleteDeref              = 138,
    CallFunctionVar          = 140,
    CallFunctionKw           = 141,
    CallFunctionVarKw        = 142,
    SetupWith                = 143,
    ExtendedArg              = 144,
    ListAppend               = 145,
    SetAdd                   = 146,
    MapAdd                   = 147,
    LoadClassderef           = 148,
    BuildListUnpack          = 149,
    BuildMapUnpack           = 150,
    BuildMapUnpackWithCall   = 151,
    BuildTupleUnpack         = 152,
    BuildSetUnpack           = 153,
    SetupAsyncWith           = 154,

    // Defined for rsnek becuase the jump instructions are kinda wierd
    LogicalAnd               = 200,
    LogicalOr                = 201
}

#[cfg(test)]
mod tests {
    use ::{fmt, Lexer, Parser};
    use nom::IResult;
    use super::*;

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
            IResult::Done(left, ref tokens) if left.len() == 0 => tokens.clone(),
            _ => unreachable!()
        };

        println!("Tokens({}):\n----------------------------------------\n{}\n",
                 tokens.len(), fmt::tokens(&tokens, true));

        match parser.parse_tokens(&tokens) {
            ParserResult::Ok(ref result) if result.remaining_tokens.len() == 0 => {
                println!("Ast(tokens: {:?})\n{}", tokens.len(), fmt::json(result.ast.borrow()));
                let ins = compiler.compile_ast(result.ast.borrow());

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
    
    basic_test!(binop_logicand,   "a and b");
    basic_test!(binop_logicor,    "a or b");
    basic_test!(binop_add,        "a + b");
    basic_test!(binop_sub,        "a - b");
    basic_test!(binop_mul,        "a * b");
    basic_test!(binop_pow,        "a ** b");
    basic_test!(binop_truediv,    "a / b");
    basic_test!(binop_floordiv,   "a // b");
    basic_test!(binop_or,         "a | b");
    basic_test!(binop_and,        "a & b");
    basic_test!(binop_xor,        "a ^ b");
    basic_test!(binop_mod,        "a % b");
    basic_test!(binop_matmul,     "a @ b");
    basic_test!(binop_lshif,      "a << b");
    basic_test!(binop_rshift,     "a >> b");


    basic_test!(multiline, r#"
x = 1
y = "somewhere over the dynamic language rainbow"
z = x + y
"#);
}
