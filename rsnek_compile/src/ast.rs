use std::fmt;
use std::fmt::{Display, Debug, Formatter};

use std::str;
use std::str::FromStr;
use itertools::Itertools;
use serde::ser::{Serialize, Serializer, SerializeSeq};
use serde_bytes;


use tokenizer::Lexer;
use token::{Id, Tk, pprint_tokens};
use slice::{TkSlice, TokenSlice};

use nom::{IResult, digit, multispace};

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Ast<'a> {
    Module(Mod<'a>),
    Statement(Stmt<'a>),
    Expression(Expr<'a>),
    Any(TokenSlice<'a>)
}


#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Mod<'a> {
    Any(TokenSlice<'a>)
}


/*
    expr = BoolOp(boolop op, expr* values)
         | BinOp(expr left, operator op, expr right)
         | UnaryOp(unaryop op, expr operand)
         | Lambda(arguments args, expr body)
         | IfExp(expr test, expr body, expr orelse)
         | Dict(expr* keys, expr* values)
*/

pub type DynExpr<'a> = Box<Expr<'a>>;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Stmt<'a> {
    Assign { target: DynExpr<'a>, value: DynExpr<'a>},
    Expr {value: Expr<'a>},
    Any(TokenSlice<'a>)
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Expr<'a> {
    // TODO: Make assign expandable like a, b = ((a,v) => None None) ()
    Bool {logic: Logic, values: Vec<DynExpr<'a>>},
    BinOp {op: Op, left: DynExpr<'a>, right: DynExpr<'a>},
    Any(Vec<TokenSlice<'a>>),
    Sanity(Vec<TkSlice<'a>>),
    Atom(TkSlice<'a>)
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Logic {
    Test(&'static str)
}

impl Logic {
    pub const AND: Self = Logic::Test("and");
    pub const OR: Self = Logic::Test("or");
}


#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Op {
    Symbol(String),
    Plus,
    Minus
}


impl Op {

    //    Sub,
//    Mult,
//    MatMult,
//    Div,
//    Mod,
//    Pow,
//    LShift,
//    RShift,
//    BitOr,
//    BitXor,
//    BitAnd,
//    FloorDiv,
    pub fn from(sym: &str) -> Self {
        match sym {
            "+" => Op::Plus,
            "-" => Op::Minus,
            _ => Op::Symbol(sym.to_string())
        }
    }
}

pub enum Singleton {
    None,
    False,
    True
}


mod helper {
    use super::*;
}

#[cfg(feature="examples")]
mod example {
    use super::*;

pub enum Expr {
    Value(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Paren(Box<Expr>),
}

pub enum Oper {
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for Expr {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        use self::Expr::*;
        match *self {
            Value(val) => write!(format, "{}", val),
            Add(ref left, ref right) => write!(format, "{} + {}", left, right),
            Sub(ref left, ref right) => write!(format, "{} - {}", left, right),
            Mul(ref left, ref right) => write!(format, "{} * {}", left, right),
            Div(ref left, ref right) => write!(format, "{} / {}", left, right),
            Paren(ref expr) => write!(format, "({})", expr),
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        use self::Expr::*;
        match *self {
            Value(val) => write!(format, "{}", val),
            Add(ref left, ref right) => write!(format, "({:?} + {:?})", left, right),
            Sub(ref left, ref right) => write!(format, "({:?} - {:?})", left, right),
            Mul(ref left, ref right) => write!(format, "({:?} * {:?})", left, right),
            Div(ref left, ref right) => write!(format, "({:?} / {:?})", left, right),
            Paren(ref expr) => write!(format, "[{:?}]", expr),
        }
    }
}


named!(parens< Expr >, delimited!(
    delimited!(opt!(multispace), tag!("("), opt!(multispace)),
    map!(map!(expr, Box::new), Expr::Paren),
    delimited!(opt!(multispace), tag!(")"), opt!(multispace))
  )
);

named!(factor< Expr >, alt_complete!(
    map!(
      map_res!(
        map_res!(
          delimited!(opt!(multispace), digit, opt!(multispace)),
          str::from_utf8
        ),
      FromStr::from_str
    ),
    Expr::Value)
  | parens
  )
);

fn fold_exprs(initial: Expr, remainder: Vec<(Oper, Expr)>) -> Expr {
    remainder.into_iter().fold(initial, |acc, pair| {
        let (oper, expr) = pair;
        match oper {
            Oper::Add => Expr::Add(Box::new(acc), Box::new(expr)),
            Oper::Sub => Expr::Sub(Box::new(acc), Box::new(expr)),
            Oper::Mul => Expr::Mul(Box::new(acc), Box::new(expr)),
            Oper::Div => Expr::Div(Box::new(acc), Box::new(expr)),
        }
    })
}

named!(term< Expr >, do_parse!(
    initial: factor >>
    remainder: many0!(
           alt!(
             do_parse!(tag!("*") >> mul: factor >> (Oper::Mul, mul)) |
             do_parse!(tag!("/") >> div: factor >> (Oper::Div, div))
           )
         ) >>
    (fold_exprs(initial, remainder))
));

named!(expr< Expr >, do_parse!(
    initial: term >>
    remainder: many0!(
           alt!(
             do_parse!(tag!("+") >> add: term >> (Oper::Add, add)) |
             do_parse!(tag!("-") >> sub: term >> (Oper::Sub, sub))
           )
         ) >>
    (fold_exprs(initial, remainder))
));

#[test]
fn factor_test() {
    assert_eq!(factor(&b"  3  "[..]).map(|x| format!("{:?}", x)),
    IResult::Done(&b""[..], String::from("3")));
}

#[test]
fn term_test() {
    assert_eq!(term(&b" 3 *  5   "[..]).map(|x| format!("{:?}", x)),
    IResult::Done(&b""[..], String::from("(3 * 5)")));
}

#[test]
fn expr_test() {
    assert_eq!(expr(&b" 1 + 2 *  3 "[..]).map(|x| format!("{:?}", x)),
    IResult::Done(&b""[..], String::from("(1 + (2 * 3))")));
    assert_eq!(expr(&b" 1 + 2 *  3 / 4 - 5 "[..]).map(|x| format!("{:?}", x)),
    IResult::Done(&b""[..], String::from("((1 + ((2 * 3) / 4)) - 5)")));
    assert_eq!(expr(&b" 72 / 2 / 3 "[..]).map(|x| format!("{:?}", x)),
    IResult::Done(&b""[..], String::from("((72 / 2) / 3)")));
}

#[test]
fn parens_test() {
    assert_eq!(expr(&b" ( 1 + 2 ) *  3 "[..]).map(|x| format!("{:?}", x)),
    IResult::Done(&b""[..], String::from("([(1 + 2)] * 3)")));
}
}

/*
-- ASDL's 7 builtin types are:
-- identifier, int, string, bytes, object, singleton, constant
--
-- singleton: None, True or False
-- constant can be None, whereas None means "no value" for object.

module Python
{
    mod = Module(stmt* body)
        | Interactive(stmt* body)
        | Expression(expr body)

        -- not really an actual node but useful in Jython's typesystem.
        | Suite(stmt* body)

    stmt = FunctionDef(identifier name, arguments args,
                       stmt* body, expr* decorator_list, expr? returns)
          | AsyncFunctionDef(identifier name, arguments args,
                             stmt* body, expr* decorator_list, expr? returns)

          | ClassDef(identifier name,
             expr* bases,
             keyword* keywords,
             stmt* body,
             expr* decorator_list)
          | Return(expr? value)

          | Delete(expr* targets)
          | Assign(expr* targets, expr value)
          | AugAssign(expr target, operator op, expr value)
          -- 'simple' indicates that we annotate simple name without parens
          | AnnAssign(expr target, expr annotation, expr? value, int simple)

          -- use 'orelse' because else is a keyword in target languages
          | For(expr target, expr iter, stmt* body, stmt* orelse)
          | AsyncFor(expr target, expr iter, stmt* body, stmt* orelse)
          | While(expr test, stmt* body, stmt* orelse)
          | If(expr test, stmt* body, stmt* orelse)
          | With(withitem* items, stmt* body)
          | AsyncWith(withitem* items, stmt* body)

          | Raise(expr? exc, expr? cause)
          | Try(stmt* body, excepthandler* handlers, stmt* orelse, stmt* finalbody)
          | Assert(expr test, expr? msg)

          | Import(alias* names)
          | ImportFrom(identifier? module, alias* names, int? level)

          | Global(identifier* names)
          | Nonlocal(identifier* names)
          | Expr(expr value)
          | Pass | Break | Continue

          -- XXX Jython will be different
          -- col_offset is the byte offset in the utf8 string the parser uses
          attributes (int lineno, int col_offset)

          -- BoolOp() can use left & right?
    expr = BoolOp(boolop op, expr* values)
         | BinOp(expr left, operator op, expr right)
         | UnaryOp(unaryop op, expr operand)
         | Lambda(arguments args, expr body)
         | IfExp(expr test, expr body, expr orelse)
         | Dict(expr* keys, expr* values)
         | Set(expr* elts)
         | ListComp(expr elt, comprehension* generators)
         | SetComp(expr elt, comprehension* generators)
         | DictComp(expr key, expr value, comprehension* generators)
         | GeneratorExp(expr elt, comprehension* generators)
         -- the grammar constrains where yield expressions can occur
         | Await(expr value)
         | Yield(expr? value)
         | YieldFrom(expr value)
         -- need sequences for compare to distinguish between
         -- x < 4 < 3 and (x < 4) < 3
         | Compare(expr left, cmpop* ops, expr* comparators)
         | Call(expr func, expr* args, keyword* keywords)
         | Num(object n) -- a number as a PyObject.
         | Str(string s) -- need to specify raw, unicode, etc?
         | FormattedValue(expr value, int? conversion, expr? format_spec)
         | JoinedStr(expr* values)
         | Bytes(bytes s)
         | NameConstant(singleton value)
         | Ellipsis
         | Constant(constant value)

         -- the following expression can appear in assignment context
         | Attribute(expr value, identifier attr, expr_context ctx)
         | Subscript(expr value, slice slice, expr_context ctx)
         | Starred(expr value, expr_context ctx)
         | Name(identifier id, expr_context ctx)
         | List(expr* elts, expr_context ctx)
         | Tuple(expr* elts, expr_context ctx)

          -- col_offset is the byte offset in the utf8 string the parser uses
          attributes (int lineno, int col_offset)

    expr_context = Load | Store | Del | AugLoad | AugStore | Param

    slice = Slice(expr? lower, expr? upper, expr? step)
          | ExtSlice(slice* dims)
          | Index(expr value)

    boolop = And | Or

    operator = Add | Sub | Mult | MatMult | Div | Mod | Pow | LShift
                 | RShift | BitOr | BitXor | BitAnd | FloorDiv

    unaryop = Invert | Not | UAdd | USub

    cmpop = Eq | NotEq | Lt | LtE | Gt | GtE | Is | IsNot | In | NotIn

    comprehension = (expr target, expr iter, expr* ifs, int is_async)

    excepthandler = ExceptHandler(expr? type, identifier? name, stmt* body)
                    attributes (int lineno, int col_offset)

    arguments = (arg* args, arg? vararg, arg* kwonlyargs, expr* kw_defaults,
                 arg? kwarg, expr* defaults)

    arg = (identifier arg, expr? annotation)
           attributes (int lineno, int col_offset)

    -- keyword arguments supplied to call (NULL identifier for **kwargs)
    keyword = (identifier? arg, expr value)

    -- import name with optional 'as' alias.
    alias = (identifier name, identifier? asname)

    withitem = (expr context_expr, expr? optional_vars)
}


# Grammar for Python

# NOTE WELL: You should also follow all the steps listed at
# https://docs.python.org/devguide/grammar.html

# Start symbols for the grammar:
#       single_input is a single interactive statement;
#       file_input is a module or sequence of commands read from an input file;
#       eval_input is the input for the eval() functions.
# NB: compound_stmt in single_input is followed by extra NEWLINE!
single_input: NEWLINE | simple_stmt | compound_stmt NEWLINE
file_input: (NEWLINE | stmt)* ENDMARKER
eval_input: testlist NEWLINE* ENDMARKER

decorator: '@' dotted_name [ '(' [arglist] ')' ] NEWLINE
decorators: decorator+
decorated: decorators (classdef | funcdef | async_funcdef)

async_funcdef: ASYNC funcdef
funcdef: 'def' NAME parameters ['->' test] ':' suite

parameters: '(' [typedargslist] ')'
typedargslist: (tfpdef ['=' test] (',' tfpdef ['=' test])* [',' [
        '*' [tfpdef] (',' tfpdef ['=' test])* [',' ['**' tfpdef [',']]]
      | '**' tfpdef [',']]]
  | '*' [tfpdef] (',' tfpdef ['=' test])* [',' ['**' tfpdef [',']]]
  | '**' tfpdef [','])
tfpdef: NAME [':' test]
varargslist: (vfpdef ['=' test] (',' vfpdef ['=' test])* [',' [
        '*' [vfpdef] (',' vfpdef ['=' test])* [',' ['**' vfpdef [',']]]
      | '**' vfpdef [',']]]
  | '*' [vfpdef] (',' vfpdef ['=' test])* [',' ['**' vfpdef [',']]]
  | '**' vfpdef [',']
)
vfpdef: NAME

stmt: simple_stmt | compound_stmt
simple_stmt: small_stmt (';' small_stmt)* [';'] NEWLINE
small_stmt: (expr_stmt | del_stmt | pass_stmt | flow_stmt |
             import_stmt | global_stmt | nonlocal_stmt | assert_stmt)
expr_stmt: testlist_star_expr (annassign | augassign (yield_expr|testlist) |
                     ('=' (yield_expr|testlist_star_expr))*)
annassign: ':' test ['=' test]
testlist_star_expr: (test|star_expr) (',' (test|star_expr))* [',']
augassign: ('+=' | '-=' | '*=' | '@=' | '/=' | '%=' | '&=' | '|=' | '^=' |
            '<<=' | '>>=' | '**=' | '//=')
# For normal and annotated assignments, additional restrictions enforced by the interpreter
del_stmt: 'del' exprlist
pass_stmt: 'pass'
flow_stmt: break_stmt | continue_stmt | return_stmt | raise_stmt | yield_stmt
break_stmt: 'break'
continue_stmt: 'continue'
return_stmt: 'return' [testlist]
yield_stmt: yield_expr
raise_stmt: 'raise' [test ['from' test]]
import_stmt: import_name | import_from
import_name: 'import' dotted_as_names
# note below: the ('.' | '...') is necessary because '...' is tokenized as ELLIPSIS
import_from: ('from' (('.' | '...')* dotted_name | ('.' | '...')+)
              'import' ('*' | '(' import_as_names ')' | import_as_names))
import_as_name: NAME ['as' NAME]
dotted_as_name: dotted_name ['as' NAME]
import_as_names: import_as_name (',' import_as_name)* [',']
dotted_as_names: dotted_as_name (',' dotted_as_name)*
dotted_name: NAME ('.' NAME)*
global_stmt: 'global' NAME (',' NAME)*
nonlocal_stmt: 'nonlocal' NAME (',' NAME)*
assert_stmt: 'assert' test [',' test]

compound_stmt: if_stmt | while_stmt | for_stmt | try_stmt | with_stmt | funcdef | classdef | decorated | async_stmt
async_stmt: ASYNC (funcdef | with_stmt | for_stmt)
if_stmt: 'if' test ':' suite ('elif' test ':' suite)* ['else' ':' suite]
while_stmt: 'while' test ':' suite ['else' ':' suite]
for_stmt: 'for' exprlist 'in' testlist ':' suite ['else' ':' suite]
try_stmt: ('try' ':' suite
           ((except_clause ':' suite)+
            ['else' ':' suite]
            ['finally' ':' suite] |
           'finally' ':' suite))
with_stmt: 'with' with_item (',' with_item)*  ':' suite
with_item: test ['as' expr]
# NB compile.c makes sure that the default except clause is last
except_clause: 'except' [test ['as' NAME]]
suite: simple_stmt | NEWLINE INDENT stmt+ DEDENT

test: or_test ['if' or_test 'else' test] | lambdef
test_nocond: or_test | lambdef_nocond
lambdef: 'lambda' [varargslist] ':' test
lambdef_nocond: 'lambda' [varargslist] ':' test_nocond
or_test: and_test ('or' and_test)*
and_test: not_test ('and' not_test)*
not_test: 'not' not_test | comparison
comparison: expr (comp_op expr)*
# <> isn't actually a valid comparison operator in Python. It's here for the
# sake of a __future__ import described in PEP 401 (which really works :-)
comp_op: '<'|'>'|'=='|'>='|'<='|'<>'|'!='|'in'|'not' 'in'|'is'|'is' 'not'
star_expr: '*' expr
expr: xor_expr ('|' xor_expr)*
xor_expr: and_expr ('^' and_expr)*
and_expr: shift_expr ('&' shift_expr)*
shift_expr: arith_expr (('<<'|'>>') arith_expr)*
arith_expr: term (('+'|'-') term)*
term: factor (('*'|'@'|'/'|'%'|'//') factor)*
factor: ('+'|'-'|'~') factor | power
power: atom_expr ['**' factor]
atom_expr: [AWAIT] atom trailer*
atom: ('(' [yield_expr|testlist_comp] ')' |
       '[' [testlist_comp] ']' |
       '{' [dictorsetmaker] '}' |
       NAME | NUMBER | STRING+ | '...' | 'None' | 'True' | 'False')
testlist_comp: (test|star_expr) ( comp_for | (',' (test|star_expr))* [','] )
trailer: '(' [arglist] ')' | '[' subscriptlist ']' | '.' NAME
subscriptlist: subscript (',' subscript)* [',']
subscript: test | [test] ':' [test] [sliceop]
sliceop: ':' [test]
exprlist: (expr|star_expr) (',' (expr|star_expr))* [',']
testlist: test (',' test)* [',']
dictorsetmaker: ( ((test ':' test | '**' expr)
                   (comp_for | (',' (test ':' test | '**' expr))* [','])) |
                  ((test | star_expr)
                   (comp_for | (',' (test | star_expr))* [','])) )

classdef: 'class' NAME ['(' [arglist] ')'] ':' suite

arglist: argument (',' argument)*  [',']

# The reason that keywords are test nodes instead of NAME is that using NAME
# results in an ambiguity. ast.c makes sure it's a NAME.
# "test '=' test" is really "keyword '=' test", but we have no such token.
# These need to be in a single rule to avoid grammar that is ambiguous
# to our LL(1) parser. Even though 'test' includes '*expr' in star_expr,
# we explicitly match '*' here, too, to give it proper precedence.
# Illegal combinations and orderings are blocked in ast.c:
# multiple (test comp_for) arguments are blocked; keyword unpackings
# that precede iterable unpackings are blocked; etc.
argument: ( test [comp_for] |
            test '=' test |
            '**' test |
            '*' test )

comp_iter: comp_for | comp_if
comp_for: [ASYNC] 'for' exprlist 'in' or_test [comp_iter]
comp_if: 'if' test_nocond [comp_iter]

# not used in grammar, but may appear in "node" passed from Parser to Compiler
encoding_decl: NAME

yield_expr: 'yield' [yield_arg]
yield_arg: 'from' test | testlist

*/

mod example2 {
    use nom;
    use bytes;

    use nom::{Compare,CompareResult,InputLength,InputIter,Slice,HexDisplay};

    use std::str;
    use std::str::FromStr;
    use bytes::{Buf,MutBuf};
    use bytes::buf::{BlockBuf,BlockBufCursor};
    use std::ops::{Range,RangeTo,RangeFrom,RangeFull};
    use std::iter::{Enumerate,Iterator};
    use std::fmt;
    use std::cmp::{min,PartialEq};

    #[derive(Clone,Copy)]
    #[repr(C)]
    pub struct BlockSlice<'a> {
      buf: &'a BlockBuf,
      start: usize,
      end:   usize,
    }

    impl<'a> BlockSlice<'a> {
      fn cursor(&self) -> WrapCursor<'a> {
        let mut cur = self.buf.buf();
        cur.advance(self.start);
        WrapCursor {
          cursor: cur,
          length: self.end - self.start,
        }
      }
    }

    impl<'a> fmt::Debug for BlockSlice<'a> {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BlockSlice {{ start: {}, end: {}, data:\n{}\n}}", self.start, self.end, self.buf.bytes().unwrap_or(&b""[..]).to_hex(16))
      }
    }

    impl<'a> PartialEq for BlockSlice<'a> {
      fn eq(&self, other: &BlockSlice<'a>) -> bool {
        let bufs = (self.buf as *const BlockBuf) == (other.buf as *const BlockBuf);
        self.start == other.start && self.end == other.end && bufs
      }
    }

    impl<'a> Slice<Range<usize>> for BlockSlice<'a> {
      fn slice(&self, range:Range<usize>) -> Self {
        BlockSlice {
          buf:   self.buf,
          start: self.start + range.start,
          //FIXME: check for valid end here
          end:   self.start + range.end,
        }
      }
    }

    impl<'a> Slice<RangeTo<usize>> for BlockSlice<'a> {
      fn slice(&self, range:RangeTo<usize>) -> Self {
        self.slice(0..range.end)
      }
    }

    impl<'a> Slice<RangeFrom<usize>> for BlockSlice<'a> {
      fn slice(&self, range:RangeFrom<usize>) -> Self {
        self.slice(range.start..self.end - self.start)
      }
    }

    impl<'a> Slice<RangeFull> for BlockSlice<'a> {
      fn slice(&self, _:RangeFull) -> Self {
        BlockSlice {
          buf:   self.buf,
          start: self.start,
          end:   self.end,
        }
      }
    }


    impl<'a> InputIter for BlockSlice<'a> {
        type Item     = u8;
        type RawItem  = u8;
        type Iter     = Enumerate<WrapCursor<'a>>;
        type IterElem = WrapCursor<'a>;

        fn iter_indices(&self)  -> Self::Iter {
          self.cursor().enumerate()
        }
        fn iter_elements(&self) -> Self::IterElem {
          self.cursor()
        }
        fn position<P>(&self, predicate: P) -> Option<usize> where P: Fn(Self::RawItem) -> bool {
          self.cursor().position(|b| predicate(b))
        }
        fn slice_index(&self, count:usize) -> Option<usize> {
          if self.end - self.start >= count {
            Some(count)
          } else {
            None
          }
        }
    }


    impl<'a> InputLength for BlockSlice<'a> {
      fn input_len(&self) -> usize {
        self.end - self.start
      }
    }

    impl<'a,'b> Compare<&'b[u8]> for BlockSlice<'a> {
      fn compare(&self, t: &'b[u8]) -> CompareResult {
        let len     = self.end - self.start;
        let blen    = t.len();
        let m       = if len < blen { len } else { blen };
        let reduced = self.slice(..m);
        let b       = &t[..m];

        for (a,b) in reduced.cursor().zip(b.iter()) {
          if a != *b {
            return CompareResult::Error;
          }
        }
        if m < blen {
          CompareResult::Incomplete
        } else {
          CompareResult::Ok
        }
      }


      #[inline(always)]
      fn compare_no_case(&self, t: &'b[u8]) -> CompareResult {
        let len     = self.end - self.start;
        let blen    = t.len();
        let m       = if len < blen { len } else { blen };
        let reduced = self.slice(..m);
        let other   = &t[..m];

        if !reduced.cursor().zip(other).all(|(a, b)| {
          match (a,*b) {
            (0...64, 0...64) | (91...96, 91...96) | (123...255, 123...255) => a == *b,
            (65...90, 65...90) | (97...122, 97...122) | (65...90, 97...122 ) |(97...122, 65...90) => {
              a & 0b01000000 == *b & 0b01000000
            }
            _ => false
          }
        }) {
          CompareResult::Error
        } else if m < blen {
          CompareResult::Incomplete
        } else {
          CompareResult::Ok
        }
      }
    }

    impl<'a,'b> Compare<&'b str> for BlockSlice<'a> {
      fn compare(&self, t: &'b str) -> CompareResult {
        self.compare(str::as_bytes(t))
      }
      fn compare_no_case(&self, t: &'b str) -> CompareResult {
        self.compare_no_case(str::as_bytes(t))
      }
    }

    //Wrapper to implement Iterator on BlockBufCursor
    pub struct WrapCursor<'a> {
      pub cursor: BlockBufCursor<'a>,
      pub length: usize,
    }

    impl<'a> Iterator for WrapCursor<'a> {
      type Item = u8;
      fn next(&mut self) -> Option<u8> {
        //println!("NEXT: length={}, remaining={}", self.length, self.cursor.remaining());
        if min(self.length, self.cursor.remaining()) > 0 {
          self.length -=1;
          Some(self.cursor.read_u8())
        } else {
          None
        }
      }
    }

    //Reimplement eat_separator instead of fixing iterators
    #[macro_export]
    macro_rules! block_eat_separator (
      ($i:expr, $arr:expr) => (
        {
          use nom::{InputLength,InputIter,Slice};
          if ($i).input_len() == 0 {
            nom::IResult::Done($i, ($i).slice(0..0))
          } else {
            match ($i).iter_indices().position(|(_, item)| {
              for (_,c) in ($arr).iter_indices() {
                if *c == item { return false; }
              }
              true
            }) {
              Some(index) => {
                nom::IResult::Done(($i).slice(index..), ($i).slice(..index))
              },
              None => {
                nom::IResult::Done(($i).slice(($i).input_len()..), $i)
              }
            }
          }
        }
      )
    );

    #[macro_export]
    macro_rules! block_named (
      ($name:ident, $submac:ident!( $($args:tt)* )) => (
        fn $name<'a>( i: BlockSlice<'a> ) -> nom::IResult<BlockSlice<'a>, BlockSlice<'a>, u32> {
          $submac!(i, $($args)*)
        }
      );
      ($name:ident<$o:ty>, $submac:ident!( $($args:tt)* )) => (
        fn $name<'a>( i: BlockSlice<'a> ) -> nom::IResult<BlockSlice<'a>, $o, u32> {
          $submac!(i, $($args)*)
        }
      );
    );

    block_named!(sp, block_eat_separator!(&b" \t\r\n"[..]));

    macro_rules! block_ws (
      ($i:expr, $($args:tt)*) => (
        {
          sep!($i, sp, $($args)*)
        }
      )
    );

    block_named!(digit, is_a!("0123456789"));

    block_named!(parens<i64>, block_ws!(delimited!( tag!("("), expr, tag!(")") )) );


    block_named!(factor<i64>, alt!(
          map_res!(
            block_ws!(digit),
            to_i64
        )
      | parens
      )
    );

    block_named!(term <i64>, do_parse!(
        init: factor >>
        res:  fold_many0!(
            pair!(alt!(tag!("*") | tag!("/")), factor),
            init,
            |acc, (op, val): (BlockSlice, i64)| {
                if (op.cursor().next().unwrap() as char) == '*' { acc * val } else { acc / val }
            }
        ) >>
        (res)
      )
    );

    block_named!(expr <i64>, do_parse!(
        init: term >>
        res:  fold_many0!(
            pair!(alt!(tag!("+") | tag!("-")), term),
            init,
            |acc, (op, val): (BlockSlice, i64)| {
                if (op.cursor().next().unwrap() as char) == '+' { acc + val } else { acc - val }
            }
        ) >>
        (res)
      )
    );


    fn blockbuf_from(input: &[u8]) -> BlockBuf {
      let mut b = BlockBuf::new(2, 100);
      b.copy_from(input);
      b
    }


    fn sl<'a>(input: &'a BlockBuf) -> BlockSlice<'a> {
      BlockSlice {
        buf: input,
        start: 0,
        end:   input.len(),
      }
    }

    fn to_i64<'a>(input: BlockSlice<'a>) -> Result<i64, ()> {
      let v: Vec<u8> = input.cursor().collect();

      match str::from_utf8(&v) {
        Err(_) => Err(()),
        Ok(s) => match FromStr::from_str(s) {
          Err(_) => Err(()),
          Ok(i)  => Ok(i)
        }
      }
    }

    #[test]
    fn factor_test() {
      let a = blockbuf_from(&b"3"[..]);
      println!("calculated: {:?}", factor(sl(&a)));
    }

    #[test]
    fn parens_test() {
      let input1 = blockbuf_from(&b" 2* (  3 + 4 ) "[..]);
      println!("calculated 1: {:?}", expr(sl(&input1)));
      let input2 = blockbuf_from(&b"  2*2 / ( 5 - 1) + 3"[..]);
      println!("calculated 2: {:?}", expr(sl(&input2)));
    }

}