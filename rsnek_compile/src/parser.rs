use std::borrow::Borrow;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

use nom;
use nom::{IResult, Slice, Compare, CompareResult, FindToken};


use tokenizer::Lexer;
use token::{Id, Tk, pprint_tokens};
use slice::{TokenSlice, TkSlice};
use ast::{self, Ast, Mod, Stmt, Expr, DynExpr};
use traits::redefs_nom::InputLength;

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

/// helper macros to build a separator parser
///
/// ```
/// # #[macro_use] extern crate nom;
/// # use nom::IResult::Done;
///
/// named!(pub consume_spaces_and_tabs, drop_tokens!(&[Id::Space, Id::Tab]));
/// # fn main() {}
/// ```
macro_rules! drop_tokens (
  ($i:expr, $arr:expr) => (
    {
      use nom::{AsChar,InputLength,InputIter,Slice,FindToken};
      if ($i).input_len() == 0 {
        nom::IResult::Done(($i).slice(0..), ($i).slice(0..0))
      } else {
        match ($i).iter_indices().map(|(j, item)| {

            let f = (j, item.find_token($arr));
            f
            })
            .filter(|&(_, is_token)| !is_token)
            .map(|(j, _)| j)
            .next() {
          ::std::option::Option::Some(index) => {
            nom::IResult::Done(($i).slice(index..), ($i).slice(..index))
          },
          ::std::option::Option::None        => {
            nom::IResult::Done(($i).slice(($i).input_len()..), ($i))
          }
        }
      }
    }
  );
);


/// For intra statement and expression space filtering
tk_named!(pub consume_space_and_tab_tokens, drop_tokens!(&[Id::Space, Id::Tab]));


/// Ignores spaces and tabs for the scope of the parser
macro_rules! ignore_spaces (
  ($i:expr, $($args:tt)*) => (
    {
      sep!($i, consume_space_and_tab_tokens, $($args)*)
    }
  )
);


mod tk_impl {
    use super::*;


    tk_named!(tk_sanity <Vec<TkSlice<'a>>>, do_parse!(
        tks: many0!(take!(1)) >>
        ({tks})
    ));

    tk_named!(tk_sanity2 <TkSlice<'a>>, do_parse!(
        tks: tag!(&[Id::Name, Id::Equal, Id::Number]) >>
        ({tks})
    ));

    tk_named!(tk_sanity3 <Ast<'a>>, do_parse!(
    ast: alt!(
            ignore_spaces!(tk_sanity) => { |r: Vec<TkSlice<'a>> | (Ast::Expression(Expr::Sanity(r))) } ) >>
    (ast)
    ));

    //

    tk_named!(atom_name <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Name])));
    tk_named!(atom_number <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Number])));
    tk_named!(token_equal <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Equal])));

    tk_named!(tk_simple_stmt_assign <Stmt<'a>>, do_parse!(
        target: atom_name >>
                token_equal >>
        number: atom_number >>
        (Stmt::Assign {
            target: Box::new(Expr::Atom(target)),
            value: Box::new(Expr::Atom(number))
         })
    ));


    tk_named!(tkslice_to_ast <Ast<'a>>, do_parse!(
    ast: alt!(
//            module  => { |r: Mod<'a> | (Ast::Module(r))     } |
            ignore_spaces!(tk_simple_stmt_assign) => { |r: Stmt<'a>| (Ast::Statement(r))  } |
            tk_sanity             => { |r: Vec<TkSlice<'a>> | (Ast::Expression(Expr::Sanity(r))) } ) >>
    (ast)
    ));



    #[cfg(test)]
    mod tests {

        use std::borrow::Borrow;
        use std::rc::Rc;

        use nom::IResult;
        use serde_json;

        use tokenizer::Lexer;
        use super::*;

        //#[test]
        fn sanity() {
            let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::tokenize("PI=3.14159".as_bytes());
            let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

            match b {
                &IResult::Done(_, ref tokens) => {
                    let slice = TkSlice(tokens);

                    match tk_sanity(slice) {
                        IResult::Error(_) => panic!("AST Error"),
                        IResult::Incomplete(_) => panic!("Ast Incomplete"),
                        IResult::Done(left, ref ast) if left.len() == 0 => {
                            println!("Ast({:?}) \n{}", tokens.len(), serde_json::to_string_pretty(&ast).unwrap());
                        },
                        IResult::Done(_, _) => panic!("Ast did not consume all tokens"),
                    }
                },
                _ => unreachable!()
            }
        }


        //#[test]
        fn sanity2() {
            let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::tokenize("PI=3.14159".as_bytes());
            let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

            match b {
                &IResult::Done(_, ref tokens) => {
                    let slice = TkSlice(tokens);

                    match tk_sanity2(slice) {
                        IResult::Error(_) => panic!("AST Error"),
                        IResult::Incomplete(_) => panic!("Ast Incomplete"),
                        IResult::Done(left, ref ast) if left.len() == 0 => {
                            println!("Ast({:?}) \n{}", tokens.len(), serde_json::to_string_pretty(&ast).unwrap());
                        },
                        IResult::Done(_, _) => panic!("Ast did not consume all tokens"),
                    }
                },
                _ => unreachable!()
            }
        }

        //#[test]
        fn sanity3() {
            let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::tokenize("PI=3.14159".as_bytes());
            let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

            match b {
                &IResult::Done(_, ref tokens) => {
                    let slice = TkSlice(tokens);

                    match tk_sanity3(slice) {
                        IResult::Error(_) => panic!("AST Error"),
                        IResult::Incomplete(_) => panic!("Ast Incomplete"),
                        IResult::Done(left, ref ast) if left.len() == 0 => {
                            println!("Ast({:?}) \n{}", tokens.len(), serde_json::to_string_pretty(&ast).unwrap());
                            println!("Ast({:?}) \n{:?}", tokens.len(), ast);
                        },
                        IResult::Done(_, _) => panic!("Ast did not consume all tokens"),
                    }
                },
                _ => unreachable!()
            }
        }

        #[test]
        fn ast_simple_assign() {
            let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::tokenize("PI   =   3.14159".as_bytes());
            let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

            match b {
                &IResult::Done(_, ref tokens) => {
                    let slice = TkSlice(tokens);

                    match tkslice_to_ast(slice) {
                        IResult::Error(_) => panic!("AST Error"),
                        IResult::Incomplete(_) => panic!("Ast Incomplete"),
                        IResult::Done(left, ref ast) if left.len() == 0 => {
                            println!("Ast({:?}) \n{}", tokens.len(), serde_json::to_string_pretty(&ast).unwrap());
                            println!("Ast({:?}) \n{:?}", tokens.len(), ast);
                        },
                        IResult::Done(_, _) => panic!("Ast did not consume all tokens"),
                    }
                },
                _ => unreachable!()
            }
        }
    }

}

mod tokenslice_impl {
    use super::*;

    macro_rules! tokenslice_take (
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


    macro_rules! tag_id (
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

    #[macro_export]
    macro_rules! tokenslice_named (
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
);


    tokenslice_named!(construct_ast <Ast<'a>>, do_parse!(
    ast: terminated!(alt!(
//            module  => { |r: Mod<'a> | (Ast::Module(r))     } |
//            stmt    => { |r: Stmt<'a>| (Ast::Statement(r))  } |
            expr    => { |r: Expr<'a>| (Ast::Expression(r)) } ),
        eof!()) >>
    (ast)
));


    tokenslice_named!(module <Mod<'a>>, do_parse!(
    // should fail
    slice: tag_id!(&[Id::Caret]) >>
    (Mod::Any(slice))
));


    tokenslice_named!(stmt <Stmt<'a>>, do_parse!(
    // should fail
    slice: tag_id!(&[Id::Caret]) >>
    (Stmt::Any(slice))
));

    tokenslice_named!(expr <Expr<'a>>, do_parse!(
    result:  call!(expr_binop) >>
    (result)
));

    tokenslice_named!(drain1 <Expr<'a>>, do_parse!(
    slice: tokenslice_take!(1) >>
    (Expr::Any(vec![slice]))
));

    tokenslice_named!(expr_binop <Expr<'a>>, do_parse!(
    lhs: call!(expr_atom)    >>
    op: tokenslice_take!(1) >>  // !(&[Id::Plus]) >>
    rhs: call!(expr_atom) >>
    (Expr::BinOp {op: ast::Op::Plus, left: Box::new(lhs), right: Box::new(rhs)})
));

    tokenslice_named!(expr_atom <Expr<'a>>, do_parse!(
    slice: alt!(tag_id!(&[Id::Name]) |
                tag_id!(&[Id::Number]) )>>
    (Expr::Any(vec![slice]))
));


    #[cfg(test)]
    mod tests {
        use std::borrow::Borrow;
        use std::rc::Rc;

        use nom::IResult;
        use serde_json;

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
                    }
                },
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
                    }
                },
                _ => unreachable!()
            }
        }
    }

}