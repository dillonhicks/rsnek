use std::ops::Deref;

use serde::Serialize;

use ::fmt;
use ::ast::{self, Ast, Module, Stmt, Expr, DynExpr, Atom, Op};
use ::token::{Tk, Id};

pub struct Compiler;

pub type Instr = String;

#[derive(Debug, Copy, Clone, Serialize)]
pub enum Context {
    Load,
    Store
}


impl Compiler {
    pub fn new() -> Self {
        Compiler {}
    }

    fn compile_expr_constant<'a>(&self, ctx: Context, tk: &'a Tk<'a>) -> Box<[Instr]> {
        let (code, value) = match ctx {
            Context::Store => {
                (OpCode::StoreName, String::from_utf8(tk.bytes().to_vec()).unwrap())
            },
            Context::Load => {
                let code = match tk.id() {
                    Id::Name => OpCode::LoadName,
                    _ => OpCode::LoadConst
                };
                (code, String::from_utf8(tk.bytes().to_vec()).unwrap())
            }
        };

        let instr = format!("{:>10} {:<16}", format!("{:?}", code), value);
        vec![instr].into_boxed_slice()
    }

    fn compile_expr_binop<'a>(&self, op: &'a Op, left: &'a Expr<'a>, right: &'a Expr<'a>) -> Box<[Instr]> {
        println!("CompileBinOp({:?} {:?} {:?})", op, left, right);
        let mut instructions: Vec<Instr> = vec![];


        match left.deref() {
            &Expr::Constant(ref tk) => {
                let mut ins = self.compile_expr_constant(Context::Load, tk);
                instructions.append(&mut ins.to_vec());
            },
            _ => unreachable!()
        };

        match right.deref() {
            &Expr::Constant(ref tk) => {
                let mut ins = self.compile_expr_constant(Context::Load, tk);
                instructions.append(&mut ins.to_vec());
            },
            _ => unreachable!()
        };

        let mut code = match op.0.id() {
            Id::Plus => format!("{:>10}", format!("{:?}", OpCode::BinaryAdd)),
            _ => unreachable!()
        };
        instructions.push(code);

        instructions.into_boxed_slice()
    }

    fn compile_stmt_assign<'a>(&self, target: &'a Expr<'a>, value: &'a Expr<'a>) -> Box<[Instr]> {
        println!("CompileAssignment(target={:?}, value={:?})", target, value);
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
            _ => println!("(noop)")
        }

        instructions.into_boxed_slice()
    }

    pub fn compile_module(&self, module: &Module) -> Box<[Instr]> {
        println!("CompileModule({:?})", module);

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

    pub fn compile(&self, ast: &Ast) -> Box<[Instr]>{
        println!("CompileAST({:?})", ast);
        let mut instructions: Vec<Instr> = vec![];

        match *ast {
            Ast::Module(ref module) => {
                let mut ins = self.compile_module(module);
                instructions.append(&mut ins.to_vec());
            },
            _ => {}
        }

        instructions.into_boxed_slice()
    }

}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
#[repr(usize)]
pub enum OpCode {
    PopTop,
    RotTwo,
    RotThree,
    DupTop,
    DupTopTwo,
    Nop,
    UnaryPositive,
    UnaryNegative,
    UnaryNot,
    UnaryInvert,
    BinaryMatrixMultiply,
    InplaceMatrixMultiply,
    BinaryPower,
    BinaryMultiply,
    BinaryModulo,
    BinaryAdd,
    BinarySubtract,
    BinarySubscr,
    BinaryFloorDivide,
    BinaryTrueDivide,
    InplaceFloorDivide,
    InplaceTrueDivide,
    GetAiter,
    GetAnext,
    BeforeAsyncWith,
    InplaceAdd,
    InplaceSubtract,
    InplaceMultiply,
    InplaceModulo,
    StoreSubscr,
    DeleteSubscr,
    BinaryLshift,
    BinaryRshift,
    BinaryAnd,
    BinaryXor,
    BinaryOr,
    InplacePower,
    GetIter,
    GetYieldFromIter,
    PrintExpr,
    LoadBuildClass,
    YieldFrom,
    GetAwaitable,
    InplaceLshift,
    InplaceRshift,
    InplaceAnd,
    InplaceXor,
    InplaceOr,
    BreakLoop,
    WithCleanupStart,
    WithCleanupFinish,
    ReturnValue,
    ImportStar,
    SetupAnnotations,
    YieldValue,
    PopBlock,
    EndFinally,
    PopExcept,
    HaveArgument,
    StoreName,
    DeleteName,
    UnpackSequence,
    ForIter,
    UnpackEx,
    StoreAttr,
    DeleteAttr,
    StoreGlobal,
    DeleteGlobal,
    LoadConst,
    LoadName,
    BuildTuple,
    BuildList,
    BuildSet,
    BuildMap,
    LoadAttr,
    CompareOp,
    ImportName,
    ImportFrom,
    JumpForward,
    JumpIfFalseOrPop,
    JumpIfTrueOrPop,
    JumpAbsolute,
    PopJumpIfFalse,
    PopJumpIfTrue,
    LoadGlobal,
    ContinueLoop,
    SetupLoop,
    SetupExcept,
    SetupFinally,
    LoadFast,
    StoreFast,
    DeleteFast,
    StoreAnnotation,
    RaiseVarargs,
    CallFunction,
    MakeFunction,
    BuildSlice,
    LoadClosure,
    LoadDeref,
    StoreDeref,
    DeleteDeref,
    CallFunctionKw,
    CallFunctionEx,
    SetupWith,
    ExtendedArg,
    ListAppend,
    SetAdd,
    MapAdd,
    LoadClassderef,
    BuildListUnpack,
    BuildMapUnpack,
    BuildMapUnpackWithCall,
    BuildTupleUnpack,
    BuildSetUnpack,
    SetupAsyncWith,
    FormatValue,
    BuildConstKeyMap,
    BuildString,
    BuildTupleUnpackWithCall,
    LoadMethod,
    CallMethod
}

#[cfg(test)]
mod tests {
    use ::{fmt, Lexer, Parser};
    use nom::IResult;
    use super::*;

    fn assert_compile<'a>(text: &'a str) {
        println!("<Input>\n\n{}\n\n</Input>", text);

        let compiler = Compiler::new();
        let parser = Parser::new();

        let tokens = match Lexer::tokenize2(text.as_bytes()) {
            IResult::Done(left, ref tokens) if left.len() == 0 => tokens.clone(),
            _ => unreachable!()
        };

        println!("Tokens({}):\n----------------------------------------\n{}\n",
                 tokens.len(), fmt::tokens(&tokens, true));

        match parser.parse_tokens(&tokens) {
            IResult::Done(left, ref ast) if left.len() == 0 => {
                println!("Ast(tokens: {:?})\n{}", tokens.len(), fmt::ast(&ast));
                let ins = compiler.compile(&ast);

                println!();
                println!("Compiled Instructions ({}):", ins.len());
                println!("--------------------------------");
                println!("{}", ins.join("\n"));
            },
            result => panic!("\n\nERROR: {:#?}\n\n", result)
        }
    }

    #[test]
    fn compile_1() {
       // assert_compile("abcd = 1234");
        assert_compile(
r#"
x = 15
y = 45j
z = x + y
"#)
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