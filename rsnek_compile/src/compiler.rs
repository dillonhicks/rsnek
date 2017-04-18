use std;
use std::ops::Deref;
use std::str::FromStr;
use std::convert::TryFrom;

use serde::Serialize;
use nom::IResult;

use ::fmt;
use ::ast::{self, Ast, Module, Stmt, Expr, DynExpr, Atom, Op};
use ::token::{Tk, Id, Tag, Num};
use ::lexer::Lexer;
use ::parser::Parser;


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
    fn from<'a>(tk: &'a Tk<'a>) -> Self {
        let parsed = String::from_utf8(tk.bytes().to_vec()).unwrap();
        let content = parsed.as_str();

        match (tk.id(), tk.tag()) {
            (Id::Name, _) |
            (Id::String, _) => Value::Str(parsed.clone()),
            (Id::Number, Tag::N(Num::Int)) => Value::Int(content.parse::<i64>().unwrap()),
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


#[derive(Debug, Copy, Clone, Serialize)]
pub struct Compiler{
    lexer: Lexer,
    parser: Parser
}


impl Compiler {
    pub fn new() -> Self {
        Compiler {
            lexer: Lexer::new(),
            parser: Parser::new(),
        }

    }

    fn compile_expr_constant<'a>(&self, ctx: Context, tk: &'a Tk<'a>) -> Box<[Instr]> {
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

    fn compile_expr_binop<'a>(&self, op: &'a Op, left: &'a Expr<'a>, right: &'a Expr<'a>) -> Box<[Instr]> {
        // println!("CompileBinOp({:?} {:?} {:?})", op, left, right);
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
            Id::Plus => Instr(OpCode::BinaryAdd, None),
            _ => unimplemented!()
        };
        instructions.push(code);

        instructions.into_boxed_slice()
    }

    fn compile_stmt_assign<'a>(&self, target: &'a Expr<'a>, value: &'a Expr<'a>) -> Box<[Instr]> {
        // println!("CompileAssignment(target={:?}, value={:?})", target, value);
        let mut instructions: Vec<Instr> = vec![];

        match value.deref() {
            &Expr::Constant(ref tk) => {
                let mut ins = self.compile_expr_constant(Context::Load, tk);
                instructions.append(&mut ins.to_vec());
            },
            &Expr::BinOp {ref op, ref left, ref right} => {
                let mut ins = self.compile_expr_binop(op, left, right);
                instructions.append(&mut ins.to_vec());
            }
            _ => unreachable!()
        };

        match target.deref() {
            &Expr::Constant(ref tk) => {
                let mut ins = self.compile_expr_constant(Context::Store, tk);
                instructions.append(&mut ins.to_vec());
            },
            _ => unreachable!()
        };

        instructions.into_boxed_slice()
    }

    fn compile_stmt<'a>(&self, stmt: &'a Stmt) -> Box<[Instr]> {
        let mut instructions: Vec<Instr> = vec![];

        match *stmt {
            Stmt::Assign { ref target, ref value } => {
                let mut ins = self.compile_stmt_assign(target, value);
                instructions.append(&mut ins.to_vec());
            },
            Stmt::Newline => {},
            _ => {} //println!("(noop)")
        }

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

    pub fn compile_str(&self, input: &str) -> Box<[Instr]> {
        let tokens = match self.lexer.tokenize2(input.as_bytes()) {
            IResult::Done(left, ref tokens) if left.len() == 0 => tokens.clone(),
            _ => panic!("Issue parsing input")
        };

        let ins = match self.parser.parse_tokens(&tokens) {
            IResult::Done(left, ref ast) if left.len() == 0 => {
                self.compile_ast(&ast)
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
    SetupAsyncWith           = 154
}

#[cfg(test)]
mod tests {
    use ::{fmt, Lexer, Parser};
    use nom::IResult;
    use super::*;

    fn assert_compile<'a>(text: &'a str) {
        println!("<Input>\n\n{}\n\n</Input>", text);

        let compiler = Compiler::new();
        let lexer = Lexer::new();
        let parser = Parser::new();

        let tokens = match lexer.tokenize2(text.as_bytes()) {
            IResult::Done(left, ref tokens) if left.len() == 0 => tokens.clone(),
            _ => unreachable!()
        };

        println!("Tokens({}):\n----------------------------------------\n{}\n",
                 tokens.len(), fmt::tokens(&tokens, true));

        match parser.parse_tokens(&tokens) {
            IResult::Done(left, ref ast) if left.len() == 0 => {
                println!("Ast(tokens: {:?})\n{}", tokens.len(), fmt::ast(&ast));
                let ins = compiler.compile_ast(&ast);

                println!();
                println!("Compiled Instructions ({}):", ins.len());
                println!("--------------------------------");
                println!("{:#?}", ins);
                println!("{}", fmt::bincode(&ins))
            },
            result => panic!("\n\nERROR: {:#?}\n\n", result)
        }
    }

    #[test]
    fn compile_1() {
       // assert_compile("abcd = 1234");
        assert_compile(
r#"
x = 123
y = 45
z = x + y
"#)
    }

    #[test]
    fn compile_2() {
        let compiler = Compiler::new();
        println!("{}", fmt::json(&compiler.compile_str("x = x + 41")));
    }
}
/*
POP_TOP,
ROT_TWO,
ROT_THREE,
DUP_TOP,
DUP_TOP_TWO,
NOP,
UNARY_POSITIVE,
UNARY_NEGATIVE,
UNARY_NOT,
UNARY_INVERT,
BINARY_MATRIX_MULTIPLY,
INPLACE_MATRIX_MULTIPLY,
BINARY_POWER,
BINARY_MULTIPLY,
BINARY_MODULO,
BINARY_ADD,
BINARY_SUBTRACT,
BINARY_SUBSCR,
BINARY_FLOOR_DIVIDE,
BINARY_TRUE_DIVIDE,
INPLACE_FLOOR_DIVIDE,
INPLACE_TRUE_DIVIDE,
GET_AITER,
GET_ANEXT,
BEFORE_ASYNC_WITH,
INPLACE_ADD,
INPLACE_SUBTRACT,
INPLACE_MULTIPLY,
INPLACE_MODULO,
STORE_SUBSCR,
DELETE_SUBSCR,
BINARY_LSHIFT,
BINARY_RSHIFT,
BINARY_AND,
BINARY_XOR,
BINARY_OR,
INPLACE_POWER,
GET_ITER,
GET_YIELD_FROM_ITER,
PRINT_EXPR,
LOAD_BUILD_CLASS,
YIELD_FROM,
GET_AWAITABLE,
INPLACE_LSHIFT,
INPLACE_RSHIFT,
INPLACE_AND,
INPLACE_XOR,
INPLACE_OR,
BREAK_LOOP,
WITH_CLEANUP_START,
WITH_CLEANUP_FINISH,
RETURN_VALUE,
IMPORT_STAR,
SETUP_ANNOTATIONS,
YIELD_VALUE,
POP_BLOCK,
END_FINALLY,
POP_EXCEPT,
HAVE_ARGUMENT,
STORE_NAME,
DELETE_NAME,
UNPACK_SEQUENCE,
FOR_ITER,
UNPACK_EX,
STORE_ATTR,
DELETE_ATTR,
STORE_GLOBAL,
DELETE_GLOBAL,
LOAD_CONST,
LOAD_NAME,
BUILD_TUPLE,
BUILD_LIST,
BUILD_SET,
BUILD_MAP,
LOAD_ATTR,
COMPARE_OP,
IMPORT_NAME,
IMPORT_FROM,
JUMP_FORWARD,
JUMP_IF_FALSE_OR_POP,
JUMP_IF_TRUE_OR_POP,
JUMP_ABSOLUTE,
POP_JUMP_IF_FALSE,
POP_JUMP_IF_TRUE,
LOAD_GLOBAL,
CONTINUE_LOOP,
SETUP_LOOP,
SETUP_EXCEPT,
SETUP_FINALLY,
LOAD_FAST,
STORE_FAST,
DELETE_FAST,
STORE_ANNOTATION,
RAISE_VARARGS,
CALL_FUNCTION,
MAKE_FUNCTION,
BUILD_SLICE,
LOAD_CLOSURE,
LOAD_DEREF,
STORE_DEREF,
DELETE_DEREF,
CALL_FUNCTION_KW,
CALL_FUNCTION_EX,
SETUP_WITH,
EXTENDED_ARG,
LIST_APPEND,
SET_ADD,
MAP_ADD,
LOAD_CLASSDEREF,
BUILD_LIST_UNPACK,
BUILD_MAP_UNPACK,
BUILD_MAP_UNPACK_WITH_CALL,
BUILD_TUPLE_UNPACK,
BUILD_SET_UNPACK,
SETUP_ASYNC_WITH,
FORMAT_VALUE,
BUILD_CONST_KEY_MAP,
BUILD_STRING,
BUILD_TUPLE_UNPACK_WITH_CALL,
LOAD_METHOD,
CALL_METHOD,

/* EXCEPT_HANDLER is a special, implicit block type which is created when
   entering an except handler. It is not an opcode but we define it here
   as we want it to be available to both frameobject.c and ceval.c, while
   remaining private.*/
EXCEPT_HANDLER,


enum cmp_op {PyCmp_LT=Py_LT, PyCmp_LE=Py_LE, PyCmp_EQ=Py_EQ, PyCmp_NE=Py_NE,
                PyCmp_GT=Py_GT, PyCmp_GE=Py_GE, PyCmp_IN, PyCmp_NOT_IN,
                PyCmp_IS, PyCmp_IS_NOT, PyCmp_EXC_MATCH, PyCmp_BAD};

#define HAS_ARG(op) ((op) >= HAVE_ARGUMENT)
*/