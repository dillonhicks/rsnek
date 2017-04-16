use std::borrow::Borrow;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

use nom;
use nom::{IResult, Slice, Compare, CompareResult};


use tokenizer::Lexer;
use token::{Id, Tk, pprint_tokens};
use slice::TokenSlice;
use ast::{self, Ast, Mod, Stmt, Expr, DynExpr};

pub struct Parser {}

impl Parser {

    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse_file(&self, filename: &str) {

        let mut contents: Vec<u8> = Vec::new();
        {
            let mut file = File::open(filename).unwrap();
            file.read_to_end(&mut contents).unwrap();
        }

        let bytes = contents.as_slice();
        let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::tokenize(bytes);
        let b: &IResult<&[u8], Vec<Tk>> = r.borrow();
        match b {
            &IResult::Done(_, ref tokens) => {
                pprint_tokens(&tokens)
            },
            _ => {}
        }
    }
}

macro_rules! tks_take(
    ($i:expr, $count:expr) => (
      {
        let cnt = $count as usize;
        let res: nom::IResult<TokenSlice<'a>,TokenSlice<'a>> = if $i.len() < cnt {
          nom::IResult::Incomplete(nom::Needed::Size(cnt))
        } else {
          nom::IResult::Done($i.slice(cnt..), $i.slice(0..cnt))
        };
        res
      }
    );
);


macro_rules! tag_id(
    ($i:expr, $bytes: expr) => (
      {
        let blen = $bytes.len();
        let reduced = $i.slice(..blen);
        let b = &$bytes[..];

        let res: nom::IResult<_,_> = match reduced.compare(b) {
            CompareResult::Ok => nom::IResult::Done($i.slice(blen..), $i.slice(..blen)),
            CompareResult::Error => nom::IResult::Error(nom::Err::Code(nom::ErrorKind::Tag)),
            CompareResult::Incomplete => nom::IResult::Incomplete(nom::Needed::Size(blen))
        };
        println!("TRACE: Compare {:?} {:?} {:?}", $i, $bytes, res);

        res
      }
    );
);

macro_rules! tk_id(
    ($i:expr, $id:expr) => (
      {
        tag_id!($i, &[$id])
      }
    );
);

#[macro_export]
macro_rules! tks_named (
    ($name:ident, $submac:ident!( $($args:tt)* )) => (
        // mine
        fn $name<'a>( i: TokenSlice<'a>) -> nom::IResult<TokenSlice<'a>, TokenSlice<'a>, u32> {
          $submac!(i, $($args)*)
        }
    );
    ($name:ident<$o:ty>, $submac:ident!( $($args:tt)* )) => (
        // mine
        fn $name<'a>( i: TokenSlice<'a>) -> nom::IResult<TokenSlice<'a>, $o, u32> {
          $submac!(i, $($args)*)
        }
    );


//
//    (#$($args:tt)*) => (
//        named_attr!(#$($args)*);
//    );
//    ($name:ident( $i:ty ) -> $o:ty, $submac:ident!( $($args:tt)* )) => (
//        #[allow(unused_variables)]
//        fn $name( i: $i ) -> $crate::IResult<$i,$o,u32> {
//            $submac!(i, $($args)*)
//        }
//    );
//    ($name:ident<$i:ty,$o:ty,$e:ty>, $submac:ident!( $($args:tt)* )) => (
//        #[allow(unused_variables)]
//        fn $name( i: $i ) -> $crate::IResult<$i, $o, $e> {
//            $submac!(i, $($args)*)
//        }
//    );
//    ($name:ident<$i:ty,$o:ty>, $submac:ident!( $($args:tt)* )) => (
//        #[allow(unused_variables)]
//        fn $name( i: $i ) -> $crate::IResult<$i, $o, u32> {
//            $submac!(i, $($args)*)
//        }
//    );
//    ($name:ident<$o:ty>, $submac:ident!( $($args:tt)* )) => (
//        #[allow(unused_variables)]
//        fn $name<'a>( i: &'a[u8] ) -> $crate::IResult<&'a [u8], $o, u32> {
//            $submac!(i, $($args)*)
//        }
//    );
//    ($name:ident, $submac:ident!( $($args:tt)* )) => (
//        #[allow(unused_variables)]
//        fn $name( i: &[u8] ) -> $crate::IResult<&[u8], &[u8], u32> {
//            $submac!(i, $($args)*)
//        }
//    );
//    (pub $name:ident( $i:ty ) -> $o:ty, $submac:ident!( $($args:tt)* )) => (
//        #[allow(unused_variables)]
//        pub fn $name( i: $i ) -> nom::IResult<$i,$o, u32> {
//            $submac!(i, $($args)*)
//        }
//    );
//    (pub $name:ident<$i:ty,$o:ty,$e:ty>, $submac:ident!( $($args:tt)* )) => (
//        #[allow(unused_variables)]
//        pub fn $name( i: $i ) -> nom::IResult<$i, $o, $e> {
//            $submac!(i, $($args)*)
//        }
//    );
//    (pub $name:ident<$i:ty,$o:ty>, $submac:ident!( $($args:tt)* )) => (
//        #[allow(unused_variables)]
//        pub fn $name( i: $i ) -> nom::IResult<$i, $o, u32> {
//            $submac!(i, $($args)*)
//        }
//    );
//    (pub $name:ident<$o:ty>, $submac:ident!( $($args:tt)* )) => (
//        #[allow(unused_variables)]
//        pub fn $name<'a>( i: &'a TokenSlice ) -> nom::IResult<TokenSlice<'a>, $o, u32> {
//            $submac!(i, $($args)*)
//        }
//    );
//    (pub $name:ident, $submac:ident!( $($args:tt)* )) => (
//        #[allow(unused_variables)]
//        pub fn $name<'a>( i: &'a TokenSlice) -> nom::IResult<TokenSlice<'a>, TokenSlice<'a>, u32> {
//            $submac!(i, $($args)*)
//        }
//    );
);


tks_named!(construct_ast <Ast<'a>>, do_parse!(
    ast: terminated!(alt!(
//            module  => { |r: Mod<'a> | (Ast::Module(r))     } |
//            stmt    => { |r: Stmt<'a>| (Ast::Statement(r))  } |
            expr    => { |r: Expr<'a>| (Ast::Expression(r)) } ),
        eof!()) >>
    (ast)
));


tks_named!(module <Mod<'a>>, do_parse!(
    // should fail
    slice: tag_id!(&[Id::Caret]) >>
    (Mod::Any(slice))
));


tks_named!(stmt <Stmt<'a>>, do_parse!(
    // should fail
    slice: tag_id!(&[Id::Caret]) >>
    (Stmt::Any(slice))
));

tks_named!(expr <Expr<'a>>, do_parse!(
    result:  call!(expr_binop) >>
    (result)
));

tks_named!(drain1 <Expr<'a>>, do_parse!(
    slice: tks_take!(1) >>
    (Expr::Any(vec![slice]))
));

tks_named!(expr_binop <Expr<'a>>, do_parse!(
    lhs: call!(expr_atom)    >>
    op: tks_take!(1) >>  // !(&[Id::Plus]) >>
    rhs: call!(expr_atom) >>
    (Expr::BinOp {op: ast::Op::Plus, left: Box::new(lhs), right: Box::new(rhs)})
));

tks_named!(expr_atom <Expr<'a>>, do_parse!(
    slice: alt!(tag_id!(&[Id::Name]) |
                tag_id!(&[Id::Number]) )>>
    (Expr::Atom(slice))
));

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::rc::Rc;

    use nom::IResult;
    use serde_json;

    use parser::construct_ast;
    use tokenizer::Lexer;
    use super::*;

    //#[test]
    fn constr_ast() {
        let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::tokenize("lambda x, **y: x(y['1'], y['2'], y.get(10))".as_bytes());
        let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

        match b {
            &IResult::Done(_, ref tokens) => {
                let slice = TokenSlice::new(tokens);
                match construct_ast(slice) {
                    IResult::Error(_) => panic!("AST Error"),
                    IResult::Incomplete(_) => panic!("Ast Incomplete"),
                    IResult::Done(left, ref ast) if left.len() == 0 => {
                        println!("Ast({:?}) \n{}", slice.len(), serde_json::to_string_pretty(&ast).unwrap());
                    },
                    IResult::Done(_, _) => panic!("Ast did not consume all tokens"),
                }},
            _ => unreachable!()
        }
    }

    #[test]
    fn parse_binop() {
        let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::tokenize("x+1".as_bytes());
        let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

        match b {
            &IResult::Done(_, ref tokens) => {
                let slice = TokenSlice::new(tokens);
                match construct_ast(slice) {
                    IResult::Error(_) => panic!("AST Error"),
                    IResult::Incomplete(ref thing) => panic!("Ast Incomplete \n{:?}", thing),
                    IResult::Done(left, ref ast) if left.len() == 0 => {
                        println!("Ast({:?}) \n{}", slice.len(), serde_json::to_string_pretty(&ast).unwrap());
                    },
                    IResult::Done(ref l, ref r) => panic!(format!("Ast did not consume all tokens\n{:?}\n{:?}", l, r)),
                }},
            _ => unreachable!()
        }
    }

}


//impl<'a,'b> Compare<&'b str> for BlockSlice<'a> {
//    fn compare(&self, t: &'b str) -> CompareResult {
//        self.compare(str::as_bytes(t))
//    }
//    fn compare_no_case(&self, t: &'b str) -> CompareResult {
//        self.compare_no_case(str::as_bytes(t))
//    }
//}
//
////Wrapper to implement Iterator on BlockBufCursor
//pub struct WrapCursor<'a> {
//    pub cursor: BlockBufCursor<'a>,
//    pub length: usize,
//}
//


////Reimplement eat_separator instead of fixing iterators
//#[macro_export]
//macro_rules! block_eat_separator (
//      ($i:expr, $arr:expr) => (
//        {
//          use nom::{InputLength,InputIter,Slice};
//          if ($i).input_len() == 0 {
//            nom::IResult::Done($i, ($i).slice(0..0))
//          } else {
//            match ($i).iter_indices().position(|(_, item)| {
//              for (_,c) in ($arr).iter_indices() {
//                if *c == item { return false; }
//              }
//              true
//            }) {
//              Some(index) => {
//                nom::IResult::Done(($i).slice(index..), ($i).slice(..index))
//              },
//              None => {
//                nom::IResult::Done(($i).slice(($i).input_len()..), $i)
//              }
//            }
//          }
//        }
//      )
//    );


//#[macro_export]
//macro_rules! block_named (
//      ($name:ident, $submac:ident!( $($args:tt)* )) => (
//        fn $name<'a>( i: BlockSlice<'a> ) -> nom::IResult<BlockSlice<'a>, BlockSlice<'a>, u32> {
//          $submac!(i, $($args)*)
//        }
//      );
//      ($name:ident<$o:ty>, $submac:ident!( $($args:tt)* )) => (
//        fn $name<'a>( i: BlockSlice<'a> ) -> nom::IResult<BlockSlice<'a>, $o, u32> {
//          $submac!(i, $($args)*)
//        }
//      );
//    );
//
//block_named!(sp, block_eat_separator!(&b" \t\r\n"[..]));
//
//macro_rules! block_ws (
//      ($i:expr, $($args:tt)*) => (
//        {
//          sep!($i, sp, $($args)*)
//        }
//      )
//    );
//
//block_named!(digit, is_a!("0123456789"));
//
//block_named!(parens<i64>, block_ws!(delimited!( tag!("("), expr, tag!(")") )) );
//
//
//block_named!(factor<i64>, alt!(
//          map_res!(
//            block_ws!(digit),
//            to_i64
//        )
//      | parens
//      )
//    );
//
//block_named!(term <i64>, do_parse!(
//        init: factor >>
//        res:  fold_many0!(
//            pair!(alt!(tag!("*") | tag!("/")), factor),
//            init,
//            |acc, (op, val): (BlockSlice, i64)| {
//                if (op.cursor().next().unwrap() as char) == '*' { acc * val } else { acc / val }
//            }
//        ) >>
//        (res)
//      )
//    );
//
//block_named!(expr <i64>, do_parse!(
//        init: term >>
//        res:  fold_many0!(
//            pair!(alt!(tag!("+") | tag!("-")), term),
//            init,
//            |acc, (op, val): (BlockSlice, i64)| {
//                if (op.cursor().next().unwrap() as char) == '+' { acc + val } else { acc - val }
//            }
//        ) >>
//        (res)
//      )
//    );
//
//
//fn blockbuf_from(input: &[u8]) -> BlockBuf {
//    let mut b = BlockBuf::new(2, 100);
//    b.copy_from(input);
//    b
//}
//
//
//fn sl<'a>(input: &'a BlockBuf) -> BlockSlice<'a> {
//    BlockSlice {
//        buf: input,
//        start: 0,
//        end:   input.len(),
//    }
//}
//
//fn to_i64<'a>(input: BlockSlice<'a>) -> Result<i64, ()> {
//    let v: Vec<u8> = input.cursor().collect();
//
//    match str::from_utf8(&v) {
//        Err(_) => Err(()),
//        Ok(s) => match FromStr::from_str(s) {
//            Err(_) => Err(()),
//            Ok(i)  => Ok(i)
//        }
//    }
//}
//
//#[test]
//fn factor_test() {
//    let a = blockbuf_from(&b"3"[..]);
//    println!("calculated: {:?}", factor(sl(&a)));
//}
//
//#[test]
//fn parens_test() {
//    let input1 = blockbuf_from(&b" 2* (  3 + 4 ) "[..]);
//    println!("calculated 1: {:?}", expr(sl(&input1)));
//    let input2 = blockbuf_from(&b"  2*2 / ( 5 - 1) + 3"[..]);
//    println!("calculated 2: {:?}", expr(sl(&input2)));
//}