use std::borrow::Borrow;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

use nom;
use nom::{IResult, Slice, Compare, CompareResult, FindToken};


use tokenizer::Lexer;
use token::{Id, Tk, pprint_tokens};
use slice::{TkSlice};
use ast::{self, Ast, Module, Stmt, Expr, DynExpr, Atom, Op};
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

/// matches one of the provided tokens
macro_rules! tk_is_one_of (
  ($i:expr, $inp: expr) => (
    {
      use nom::Slice;
      use nom::AsChar;
      use nom::FindToken;
      use nom::InputIter;

      match ($i).iter_elements().next().map(|c| {
        c.find_token($inp)
      }) {
        None        => nom::IResult::Incomplete::<_, _>(nom::Needed::Size(1)),
        Some(false) => nom::IResult::Error(error_position!(nom::ErrorKind::OneOf, $i)),
        //the unwrap should be safe here
        Some(true)  => nom::IResult::Done($i.slice(1..), $i.iter_elements().next().unwrap())
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
    use nom::ErrorKind;


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
    tk_named!(assign_token <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Equal])));
    tk_named!(newline_token <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Newline])));

    tk_named!(unaryop_token <TkSlice<'a>>, ignore_spaces!(
        alt_complete!(
            tag!(&[Id::Plus])     |
            tag!(&[Id::Minus])    |
            tag!(&[Id::Tilde])
        )
     ));

    tk_named!(binop_token <TkSlice<'a>>, ignore_spaces!(
        alt_complete!(
            tag!(&[Id::Plus])     |
            tag!(&[Id::Minus])    |
            tag!(&[Id::Star])     |
            tag!(&[Id::Slash])    |
            tag!(&[Id::Pipe])     |
            tag!(&[Id::Percent])  |
            tag!(&[Id::Amp])      |
            tag!(&[Id::At])       |
            tag!(&[Id::Caret])
        )
     ));


    tk_named!(augassign_token <TkSlice<'a>>, ignore_spaces!(
        alt_complete!(
            tag!(&[Id::LeftShiftEqual])   |
            tag!(&[Id::RightShiftEqual])  |
            tag!(&[Id::DoubleSlashEqual]) |
            tag!(&[Id::DoubleStarEqual])  |
            tag!(&[Id::PipeEqual])        |
            tag!(&[Id::PercentEqual])     |
            tag!(&[Id::AmpEqual])         |
            tag!(&[Id::PlusEqual])        |
            tag!(&[Id::MinusEqual])       |
            tag!(&[Id::StarEqual])        |
            tag!(&[Id::SlashEqual])       |
            tag!(&[Id::CaretEqual])       |
            tag!(&[Id::AtEqual])
        )
     ));


    /// START(ast)
    tk_named!(tkslice_to_ast <Ast<'a>>, do_parse!(
    ast: alt!(
            module_start               => { |m: Module<'a> | (Ast::Module(m))     } |
            ignore_spaces!(stmt_start) => { |r: Stmt<'a>   | (Ast::Statement(r))  } |
            tk_sanity                  => { |r: Vec<TkSlice<'a>> | (Ast::Expression(Expr::Sanity(r))) } ) >>
    (ast)
    ));

    tk_named!(module_start <Module<'a>>, do_parse!(
        body: many0!(stmt_start) >>
        (Module::Body(body))
    ));

    /// START(stmt)
    tk_named!(stmt_start <Stmt<'a>>, do_parse!(
        statement: alt!(
            sub_stmt_assign      |
            sub_stmt_augassign   |
            sub_stmt_expr        ) >>
        (statement)
    ));

    /// 5.   | Assign(expr* targets, expr value)
    tk_named!(sub_stmt_assign <Stmt<'a>>, do_parse!(
        target: atom_name    >>
                assign_token >>
        number: atom_number  >>
        (Stmt::Assign {
            target: Box::new(Expr::Atom(Atom::Name(target))),
            value: Box::new(Expr::Atom(Atom::Number(number)))
         })
    ));

    /// 6.   | AugAssign(expr target, operator op, expr value)
    tk_named!(sub_stmt_augassign <Stmt<'a>>, do_parse!(
        // TODO: Allow subparsing of target and number as actual expr
        target: atom_name       >>
            op: augassign_token >>
        number: atom_number     >>
        (Stmt::AugAssign {
            op: Op(op),
            target: Box::new(Expr::Atom(Atom::Name(target))),
            value: Box::new(Expr::Atom(Atom::Number(number)))
         })
    ));

    /// 20.   | Expr(expr value)
    tk_named!(sub_stmt_expr <Stmt<'a>>, do_parse!(
        expression: start_expr >>
        (Stmt::Expr { value: expression })
    ));


    /// 22.   └ attributes (int lineno, int col_offset)
    /// Inject a empty statement for the next line
    tk_named!(sub_stmt_next_line<Stmt<'a>>, do_parse!(
        newline_token >>
        (Stmt::Newline)
    ));


    /// START(expr)
    tk_named!(start_expr <Expr<'a>>, do_parse!(
        expression: alt!(
            sub_expr_binop        |
            sub_expr_nameconstant |
            sub_expr_constant     |
            sub_expr_ended
                                  ) >>
        (expression)
    ));

    /// 5.   | Assign(expr* targets, expr value)
    tk_named!(sub_expr_binop <Expr<'a>>, do_parse!(
        target: atom_name   >>
            op: binop_token >>
        number: atom_number >>
        (Expr::BinOp {
            op: Op(op),
            left: Box::new(Expr::Atom(Atom::Name(target))),
            right: Box::new(Expr::Atom(Atom::Number(number)))
         })
    ));

    tk_named!(sub_expr_nameconstant <Expr<'a>>, do_parse!(
        constant: alt_complete!(
            tag!(&[Id::True])     |
            tag!(&[Id::False])    |
            tag!(&[Id::False])    ) >>
        (Expr::NameConstant(constant))
    ));

    tk_named!(sub_expr_constant <Expr<'a>>, do_parse!(
        constant: alt!(
                atom_name    |
                atom_number  )>>
        (Expr::Constant(constant))
    ));

    const LOOKAHEAD_ERROR: u32 = 1024;

    /// 31.   └ attributes (int lineno, int col_offset)
    tk_named!(sub_expr_ended<Expr<'a>>, do_parse!(
        token: newline_token >>
        (Expr::End)
    ));


    #[cfg(test)]
    mod tests {

        use std::borrow::Borrow;
        use std::rc::Rc;

        use nom::IResult;
        use serde_json;

        use tokenizer::Lexer;
        use super::*;

        fn assert_complete<'a>(tokens: &Vec<Tk<'a>>, result: &IResult<TkSlice<'a>,Ast<'a>>) {
            match *result {
                IResult::Error(_) => panic!("AST Error"),
                IResult::Incomplete(_) => panic!("Ast Incomplete"),
                IResult::Done(left, ref ast) if left.len() == 0 => {
                    println!("Ast({:?}) \n{}", tokens.len(), serde_json::to_string_pretty(&ast).unwrap());
                    println!("Ast({:?}) \n{:?}", tokens.len(), ast);
                },
                IResult::Done(_, _) => panic!("Ast did not consume all tokens"),
            }
        }

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
                    let result = tkslice_to_ast(slice);
                    assert_complete(&tokens, &result);
                },
                _ => unreachable!()
            }
        }

        #[test]
        fn ast_simple_augassign() {
            let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::tokenize(r#"f **= 14"#.as_bytes());
            let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

            match b {
                &IResult::Done(_, ref tokens) => {
                    let slice = TkSlice(tokens);
                    let result = tkslice_to_ast(slice);
                    assert_complete(&tokens, &result);
                },
                _ => unreachable!()
            }
        }

        #[test]
        fn ast_multiple_stmts() {
            let input =
r#"
f **= 14
g = 0x00123
"#;

            let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::tokenize(r#"
f **= 14
g = 0x00123
"#.as_bytes());
            let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

            match b {
                &IResult::Done(_, ref tokens) => {
                    println!("{}", input);
                    pprint_tokens(tokens);
                    let slice = TkSlice(tokens);
                    let result = tkslice_to_ast(slice);
                    assert_complete(&tokens, &result);
                },
                _ => unreachable!()
            }
        }

    }

}
