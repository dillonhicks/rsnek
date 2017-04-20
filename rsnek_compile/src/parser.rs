use std::borrow::Borrow;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

use nom;
use nom::{IResult, Slice, Compare, CompareResult, FindToken};

use lexer::Lexer;
use token::{Id, Tk, pprint_tokens};
use slice::{TkSlice};
use ast::{self, Ast, Module, Stmt, Expr, DynExpr, Atom, Op};
use traits::redefs_nom::InputLength;

pub type ParseResult<'a> = IResult<TkSlice<'a>, Ast<'a>>;


enum ParserPass {}

/// Generalized form of nom's `eat_seperator!` macro
//
/// helper macros to build a separator parser
///
/// ```ignore
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



#[derive(Debug, Clone, Copy, Serialize, Default)]
struct ParserState<'a> {
    line: usize,
    column: usize,
    indent: Option<TkSlice<'a>>
}


#[derive(Debug, Clone, Copy, Serialize)]
pub struct Parser<'a> {
    state: ParserState<'a>,
}


impl<'a> Parser<'a> {

    pub fn new() -> Self {
        Parser { state: ParserState::default() }
    }

    /// Public wrapper to the macro generated tkslice_to_ast which will take a slice of
    /// tokens, turn those into a TkSlice, and parse that into an AST.
    pub fn parse_tokens<'b>(&mut self, tokens: &'b [Tk<'b>]) -> ParseResult {
        self.tkslice_to_ast(TkSlice(tokens)).1
    }

    #[deprecated]
    pub fn parse_file(&self, filename: &str) {

        let mut contents: Vec<u8> = Vec::new();
        {
            let mut file = File::open(filename).unwrap();
            file.read_to_end(&mut contents).unwrap();
        }

        let bytes = contents.as_slice();
        let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::new().tokenize(bytes);
        let b: &IResult<&[u8], Vec<Tk>> = r.borrow();
        match b {
            &IResult::Done(_, ref tokens) => {
                pprint_tokens(&tokens)
            },
            _ => {}
        }
    }


    // AST Builders

    /// START(ast)
    tk_method!(tkslice_to_ast <Parser<'a>, Ast>, mut self, do_parse!(
        ast: alt!(
            call_m!(self.module_start)      => { |m: Module | (Ast::Module(m))     } |
            ignore_spaces!(
                call_m!(self.stmt_start))   => { |r: Stmt   | (Ast::Statement(r))  } ) >>
        (ast)
    ));

    tk_method!(module_start <Parser<'a>, Module>, mut self, do_parse!(
        body: many0!(call_m!(self.stmt_start)) >>
        (Module::Body(body))
    ));

    /// START(stmt)
    tk_method!(stmt_start <Parser<'a>, Stmt<'b>>, mut self, do_parse!(
        statement: alt!(
            call_m!(self.sub_stmt_assign)      |
            call_m!(self.sub_stmt_augassign)   |
            call_m!(self.sub_stmt_expr)        |
            call_m!(self.sub_stmt_next_line)   ) >>
        (statement)
    ));

    /// 5.   | Assign(expr* targets, expr value)
    tk_method!(sub_stmt_assign <Parser<'a>, Stmt<'b>>, mut self, do_parse!(
         // TODO: Allow subparsing of target and number as actual expr
        target: atom_name           >>
                assign_token        >>
         value: start_expr >>
        (Stmt::Assign {
            target: Box::new(Expr::Constant(target.as_token())),
            value: Box::new(value)
         })
    ));

    /// 6.   | AugAssign(expr target, operator op, expr value)
    tk_method!(sub_stmt_augassign <Parser<'a>, Stmt<'b>>, mut self, do_parse!(
        // TODO: Allow subparsing of target and number as actual expr
        target: atom_name       >>
            op: augassign_token >>
        number: atom_number     >>
        (Stmt::AugAssign {
            op: Op(op.as_token()),
            target: Box::new(Expr::Atom(Atom::Name(target))),
            value: Box::new(Expr::Atom(Atom::Number(number)))
         })
    ));

    /// 20.   | Expr(expr value)
    tk_method!(sub_stmt_expr <Parser<'a>, Stmt<'b>>, mut self, do_parse!(
        expression: call_m!(self.start_expr) >>
        (Stmt::Expr(expression))
    ));


    /// 22.   └ attributes (int lineno, int col_offset)
    /// Inject a empty statement for the next line
    tk_method!(sub_stmt_next_line <Parser<'a>, Stmt<'b>>, mut self, do_parse!(
        newline_token >>
        (Stmt::Newline)
    ));


    /// START(expr)
    tk_method!(start_expr <Parser<'a>, Expr<'b>>, mut self, do_parse!(
        expression: alt_complete!(
            call_m!(self.sub_expr_binop)        |
            call_m!(self.sub_expr_nameconstant) |
            call_m!(self.sub_expr_constant)     ) >>
        (expression)
    ));


    /// 1.  =  BoolOp(boolop op, expr* values)
    /// 2.  | BinOp(expr left, operator op, expr right)
    tk_method!(sub_expr_binop <Parser<'a>, Expr<'b>>, mut self, do_parse!(
        // TODO: T45 - Generalize to allow recursion into the L and R parts of a tree
        // on start_expr not just the constant expressions
        lhs: call_m!(self.sub_expr_constant)  >>
         op: binop_token                      >>
        rhs: call_m!(self.sub_expr_constant)  >>
        (Expr::BinOp {
            op: Op(op.as_token()),
            left: Box::new(lhs),
            right: Box::new(rhs)
         })
    ));

    tk_method!(sub_expr_nameconstant <Parser<'a>, Expr<'b>>, mut self, do_parse!(
        constant: alt_complete!(
            tag!(&[Id::True])     |
            tag!(&[Id::False])    |
            tag!(&[Id::None])     ) >>
        (Expr::NameConstant(constant))
    ));

    tk_method!(sub_expr_constant <Parser<'a>, Expr<'b>>, mut self, do_parse!(
        constant: constant_token >>
        (Expr::Constant(constant.as_token()))
    ));

    /// 31.   └ attributes (int lineno, int col_offset)
    tk_method!(sub_expr_ended <Parser<'a>, Expr<'b>>, mut self, do_parse!(
        token: newline_token >>
        (Expr::End)
    ));
}


//
//#[macro_use]
//mod parser_internal {
//    use super::*;
    use nom::ErrorKind;


    #[inline(always)]
    pub fn parse_tokens<'a>(tokens: &'a [Tk<'a>]) -> ParseResult<'a> {
        tkslice_to_ast(TkSlice(tokens))
    }



    /// Matches one of the provided tokens.
    /// Generalized form of nom's `one_of!` macro.
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
    

    // Specific Tokens and Groups of Tokens
    tk_named!(atom_name <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Name])));
    tk_named!(atom_number <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Number])));
    tk_named!(assign_token <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Equal])));
    tk_named!(newline_token <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Newline])));


    /// Binary Operators: `a + b`, `a | b`, etc.
    tk_named!(binop_token <TkSlice<'a>>, ignore_spaces!(
        alt_complete!(
            tag!(&[Id::And])                |
            tag!(&[Id::Or])                 |
            tag!(&[Id::Plus])               |
            tag!(&[Id::Minus])              |
            tag!(&[Id::Star])               |
            tag!(&[Id::DoubleStar])         |
            tag!(&[Id::Slash])              |
            tag!(&[Id::DoubleSlash])        |
            tag!(&[Id::Pipe])               |
            tag!(&[Id::Percent])            |
            tag!(&[Id::Amp])                |
            tag!(&[Id::At])                 |
            tag!(&[Id::Caret])              |
            tag!(&[Id::LeftShift])          |
            tag!(&[Id::RightShift])
        )
    ));


    /// Unary Operatos: `+`, `-`,
    tk_named!(unaryop_token <TkSlice<'a>>, ignore_spaces!(
        alt_complete!(
            tag!(&[Id::Plus])               |
            tag!(&[Id::Minus])              |
            tag!(&[Id::Tilde])
        )
     ));


    tk_named!(constant_token <TkSlice<'a>>, ignore_spaces!(
        alt_complete!(
            tag!(&[Id::Name])               |
            tag!(&[Id::String])             |
            tag!(&[Id::Number])
        )
     ));


    /// Augmented Assignment Operators: `a += b`, ` a <<= b`, etc.
    tk_named!(augassign_token <TkSlice<'a>>, ignore_spaces!(
        alt_complete!(
            tag!(&[Id::LeftShiftEqual])     |
            tag!(&[Id::RightShiftEqual])    |
            tag!(&[Id::DoubleSlashEqual])   |
            tag!(&[Id::DoubleStarEqual])    |
            tag!(&[Id::PipeEqual])          |
            tag!(&[Id::PercentEqual])       |
            tag!(&[Id::AmpEqual])           |
            tag!(&[Id::PlusEqual])          |
            tag!(&[Id::MinusEqual])         |
            tag!(&[Id::StarEqual])          |
            tag!(&[Id::SlashEqual])         |
            tag!(&[Id::CaretEqual])         |
            tag!(&[Id::AtEqual])
        )
     ));


    // AST Builders

    /// START(ast)
    tk_named!(tkslice_to_ast <Ast<'a>>, do_parse!(
    ast: alt!(
            module_start               => { |m: Module<'a> | (Ast::Module(m))     } |
            ignore_spaces!(stmt_start) => { |r: Stmt<'a>   | (Ast::Statement(r))  } ) >>
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
            sub_stmt_expr        |
            sub_stmt_next_line   ) >>
        (statement)
    ));

    /// 5.   | Assign(expr* targets, expr value)
    tk_named!(sub_stmt_assign <Stmt<'a>>, do_parse!(
         // TODO: Allow subparsing of target and number as actual expr
        target: atom_name           >>
                assign_token        >>
         value: start_expr >>
        (Stmt::Assign {
            target: Box::new(Expr::Constant(target.as_token())),
            value: Box::new(value)
         })
    ));

    /// 6.   | AugAssign(expr target, operator op, expr value)
    tk_named!(sub_stmt_augassign <Stmt<'a>>, do_parse!(
        // TODO: Allow subparsing of target and number as actual expr
        target: atom_name       >>
            op: augassign_token >>
        number: atom_number     >>
        (Stmt::AugAssign {
            op: Op(op.as_token()),
            target: Box::new(Expr::Atom(Atom::Name(target))),
            value: Box::new(Expr::Atom(Atom::Number(number)))
         })
    ));

    /// 20.   | Expr(expr value)
    tk_named!(sub_stmt_expr <Stmt<'a>>, do_parse!(
        expression: start_expr >>
        (Stmt::Expr(expression))
    ));


    /// 22.   └ attributes (int lineno, int col_offset)
    /// Inject a empty statement for the next line
    tk_named!(sub_stmt_next_line<Stmt<'a>>, do_parse!(
        newline_token >>
        (Stmt::Newline)
    ));


    /// START(expr)
    tk_named!(start_expr <Expr<'a>>, do_parse!(
        expression: alt_complete!(
            sub_expr_binop        |
            sub_expr_nameconstant |
            sub_expr_constant
            //sub_expr_ended
                                  ) >>
        (expression)
    ));



    /// 1.  =  BoolOp(boolop op, expr* values)
    /// 2.  | BinOp(expr left, operator op, expr right)
    tk_named!(sub_expr_binop <Expr<'a>>, do_parse!(
        // TODO: T45 - Generalize to allow recursion into the L and R parts of a tree
        // on start_expr not just the constant expressions
        lhs: sub_expr_constant  >>
         op: binop_token        >>
        rhs: sub_expr_constant >>
        (Expr::BinOp {
            op: Op(op.as_token()),
            left: Box::new(lhs),
            right: Box::new(rhs)
         })
    ));

    tk_named!(sub_expr_nameconstant <Expr<'a>>, do_parse!(
        constant: alt_complete!(
            tag!(&[Id::True])     |
            tag!(&[Id::False])    |
            tag!(&[Id::None])     ) >>
        (Expr::NameConstant(constant))
    ));

    tk_named!(sub_expr_constant <Expr<'a>>, do_parse!(
        constant: constant_token >>
        (Expr::Constant(constant.as_token()))
    ));

    /// 31.   └ attributes (int lineno, int col_offset)
    tk_named!(sub_expr_ended<Expr<'a>>, do_parse!(
        token: newline_token >>
        (Expr::End)
    ));

//}

#[cfg(test)]
mod tests {

    use std::borrow::Borrow;
    use std::rc::Rc;

    use nom::IResult;
    use serde_json;

    use lexer::Lexer;
    use super::*;

    fn assert_parsable(input: &str) {
        let mut parser = Parser::new();
        let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::new().tokenize(input.as_bytes());
        let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

        match b {
            &IResult::Done(_, ref tokens) => {
                println!("{}", input);
                pprint_tokens(tokens);
                let result = parser.parse_tokens(tokens);
                assert_complete(&tokens, &result);
            },
            _ => unreachable!()
        }
    }

    fn assert_complete<'a>(tokens: &Vec<Tk<'a>>, result: &IResult<TkSlice<'a>,Ast<'a>>) {
        match *result {
            IResult::Error(_) => panic!("AST Error"),
            IResult::Incomplete(_) => {
                panic!("<Panic>\nAst Incomplete\nTokens\n{:?}\n</Panic>", tokens);
            },
            IResult::Done(left, ref ast) if left.len() == 0 => {
                println!("Ast({:?}) \n{}", tokens.len(), serde_json::to_string_pretty(&ast).unwrap());
            },
            IResult::Done(ref remaining, ref ast) => {
                panic!("<Panic>\nAst did not consume all tokens\nTokens\n{:?}\n\nRemaining:\n{}\n\nPartial AST:\n{}\n</Panic>\n",
                       tokens, serde_json::to_string_pretty(&remaining).unwrap(),
                       serde_json::to_string_pretty(&ast).unwrap());
            }
        }
    }

    #[test]
    fn stmt_assign_constant_expr() {
        assert_parsable("PI = 3.14159");
        assert_parsable("stuff = 'hello world!'");
        assert_parsable("spaghetti = True")
    }

    #[test]
    fn stmt_assign_binop_expr() {
        assert_parsable("z = x + y");
    }

    #[test]
    fn ast_simple_augassign() {
        assert_parsable("f **= 14");
    }

    #[test]
    fn ast_multiple_stmts() {
        let input =
            r#"
f **= 14
g = 0x00123
fun = beer + jetski
"#;
        assert_parsable(input);
    }

    #[test]
    fn ast_expr_binop_simple() {
        assert_parsable("y + 1");
        assert_parsable("1 + 1");
        assert_parsable(r#" "string" * 0o472 "#);
        // FIXME: Failing, lack of proper parse stacking or folding
        // assert_parsable("y + 1 + 4");
    }


}
