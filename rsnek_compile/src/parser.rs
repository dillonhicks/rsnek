use std;
use nom;
use nom::{IResult, InputLength, Slice, FindSubstring, ErrorKind, Needed, Err};

use ::token::{Id, Tk, OwnedTk};
use ::slice::{TkSlice};
use ::ast::{Ast, Module, Stmt, Expr, Op, FnType};
use ::traits::redefs_nom::InputLengthRedef;
use ::preprocessor::{Preprocessor, BlockScopePreprocessor};

use self::internal::*;


#[derive(Debug, Clone, Serialize)]
pub enum ParserResult {
    Ok(ParsedAst),
    Error(ParsedAst),
}


#[derive(Debug, Clone, Serialize)]
pub struct ParsedAst{
    pub ast: Ast,
    pub p1_tokens: Vec<OwnedTk>,
    pub p2_tokens: Vec<OwnedTk>,
    pub remaining_tokens: Vec<OwnedTk>,
}


impl ParsedAst {
    pub fn new<'a>(ast: Option<&Ast>,
                   remaining: Option<TkSlice<'a>>,
                   p1_tokens: TkSlice<'a>,
                   p2_tokens: &'a [Tk<'a>]) -> Self {

        ParsedAst {
            ast: match ast {
                Some(a) => a.clone(),
                None => Ast::default()
            },
            remaining_tokens: remaining.iter()
                .flat_map(TkSlice::iter)
                .map(OwnedTk::from)
                .collect::<Vec<OwnedTk>>(),
            p1_tokens: p1_tokens.iter().map(OwnedTk::from).collect::<Vec<OwnedTk>>(),
            p2_tokens: p2_tokens.iter().map(OwnedTk::from).collect::<Vec<OwnedTk>>(),
        }
    }
}


#[derive(Debug, Copy, Clone, Serialize, Default)]
struct ParserState<'a> {
    line: usize,
    column: usize,
    indent: usize,
    unused: Option<TkSlice<'a>>
}


#[derive(Debug, Copy, Clone, Serialize)]
pub struct Parser<'a> {
    state: ParserState<'a>,
}

#[repr(u32)]
enum ParserError {
    ConditionalExpr = 1024
}

impl ParserError {
    pub const fn code<'a>(self) -> Err<TkSlice<'a>, u32> {
        Err::Code(ErrorKind::Custom(self as u32))
    }
}


#[allow(unused_mut, dead_code, unused_imports)]
impl<'a> Parser<'a> {

    pub fn new() -> Self {
        let state = ParserState::default();
        Parser { state: state }
    }

    /// Public wrapper to the macro generated tkslice_to_ast which will take a slice of
    /// tokens, turn those into a TkSlice, and parse that into an AST.
    pub fn parse_tokens<'b, 'c>(&mut self, tokens: &'b [Tk<'b>]) -> ParserResult {

        // Get the iterator for each phase of tokens. Wait until calling ParsedAst::new
        // to collapse them so we do not need .clone() everywhere.

        let bspp = BlockScopePreprocessor::new();
        let bspp_tokens: Box<[Tk<'b>]> = match bspp.transform(TkSlice(tokens)) {
            Ok(boxed_tks) => boxed_tks,
            Err(err) => {
                warn!("Had to eat error due to return type";
                "Error" => format!("{}", err));

                return ParserResult::Error(
                    ParsedAst::new(None, None, TkSlice(tokens), &[]));
            }
        };

        // The (&(*(val))) pattern is to dereference the box to remove the indirection and
        // get the real address to the slice instead of the address to the box.
        //  *(box -> value) => value
        //  &(value) => ptr value
        let slice = TkSlice(&(*bspp_tokens));
        let result = self.tkslice_to_ast(slice).1;

        let p1_tokens = TkSlice(tokens);
        let p2_tokens = &(*bspp_tokens);

        // TODO: {T94} Try to incorporate parser error messages here
        match result {
            IResult::Done(ref remaining, ref ast) if remaining.len() == 0 => {
                ParserResult::Ok(ParsedAst::new(Some(ast), None, p1_tokens, p2_tokens))
            },
            // Still an error case since there are remaining tokens
            IResult::Done(ref remaining, ref ast) => {
                ParserResult::Error(ParsedAst::new(
                    Some(ast), Some(*remaining), p1_tokens, p2_tokens))
            },
            IResult::Error(_) => {
                // TODO: {T94} Consume error in parse result in some useful message
                ParserResult::Error(ParsedAst::new(None, None, p1_tokens, p2_tokens))
            }
            IResult::Incomplete(_) => {
                // TODO: {T94} nom::Needed enum has some extra info about parsing
                ParserResult::Error(ParsedAst::new(None, None, p1_tokens, p2_tokens))
            }
        }
    }

    // Example of keeping parser state, see sub_stmt_next_line.
    // Eventually the parser should inject extra context into
    // ast nodes for richer errors.
    fn inc_lineno(&mut self) {
        self.state.line += 1;
    }

    // AST Parser-Builders - they map approximately 1:1 with Grammar.txt exceptions
    // and extensions are noted.
    //

    /// START(ast)
    tk_method!(tkslice_to_ast, 'b, <Parser<'a>, Ast>, mut self, do_parse!(
        ast: alt!(
            call_m!(self.module_start)  => { |m: Module | (Ast::Module(m))     } |
            call_m!(self.stmt_start)    => { |r: Stmt   | (Ast::Statement(r))  } ) >>

        (ast)
    ));

    tk_method!(module_start, 'b, <Parser<'a>, Module>, mut self, do_parse!(
        body: many0!(call_m!(self.stmt_start)) >>

        (Module::Body(body))
    ));

    /// START(stmt)
    tk_method!(stmt_start, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
        statement: ignore_spaces!(alt!(
            call_m!(self.sub_stmt_funcdef)              |
            call_m!(self.sub_stmt_block)                |
            call_m!(self.sub_stmt_return)               |
            call_m!(self.sub_stmt_assign)               |
            call_m!(self.sub_stmt_augassign)            |
            call_m!(self.sub_stmt_expr)                 |
            call_m!(self.sub_stmt_next_line)            )) >>

        (statement)
    ));

    ///1.    = FunctionDef(identifier name, arguments args,
    ///
    /// Functions are just some fancy window dressing on blocks
    tk_method!(sub_stmt_funcdef, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
                   def_keyword                          >>
        func_name: name_token                           >>
                   lparen_token                         >>
            args:  call_m!(self.sub_expr_func_args)     >>
                   rparen_token                         >>
                   colon_token                          >>
       body_block: call_m!(self.sub_stmt_block)         >>

          (Stmt::FunctionDef {
                fntype: FnType::Sync ,
                name: func_name.as_owned_token(),
                body: Box::new(body_block),
                arguments: args
           })
    ));

    /// Blocks are a unit of nesting that can contain many statements including
    /// other nested blocks and functions and stuff.
    ///
    /// Note that they do not have a representation in Grammar.txt but are very
    /// apparent when you start implementing the compiler and interpreter functionality
    /// to handle calling functions. CPython's frame type has its own block stack
    /// for managing blocks even.
    tk_method!(sub_stmt_block, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
               block_start                              >>
        stmts: many0!(call_m!(self.stmt_start))         >>
               block_end                                >>

        (Stmt::Block(stmts))
    ));

    /// 4.   | Return(expr? value)
    tk_method!(sub_stmt_return, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
        return_keyword                                  >>
        value: opt!(call_m!(self.start_expr))           >>

        (Stmt::Return(value))
    ));

    /// 5.   | Assign(expr* targets, expr value)
    tk_method!(sub_stmt_assign, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
         // TODO: {T95} Enabled parser to handle nested expressions
        target: name_token                              >>
                assign_token                            >>
         value: call_m!(self.start_expr)                >>

        (Stmt::Assign {
            target: Expr::Constant(target.as_owned_token()),
            value: value
         })
    ));

    /// 6.   | AugAssign(expr target, operator op, expr value)
    tk_method!(sub_stmt_augassign, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
        // TODO: {T95} Enabled parser to handle nested expressions
        target: name_token                              >>
            op: augassign_token                         >>
        value: call_m!(self.start_expr)                 >>

        (Stmt::AugAssign {
            op: Op(op.as_owned_token()),
            target: Expr::Constant(target.as_owned_token()),
            value: value
         })
    ));

    /// 20.   | Expr(expr value)
    tk_method!(sub_stmt_expr, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
        expression: call_m!(self.start_expr)            >>

        (Stmt::Expr(expression))
    ));


    /// 22.   └ attributes (int lineno, int col_offset)
    /// Inject a empty statement for the next line
    tk_method!(sub_stmt_next_line, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
        newline_token                                   >>

        ({self.inc_lineno(); Stmt::Newline})
    ));



    /// START(expr) -alt
    tk_method!(pub start_expr, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        expression: alt_complete!(
            call_m!(self.sub_expr_lambda)               |
            call_m!(self.sub_expr_conditional)          |
            call_m!(self.sub_expr_binop)                |
            call_m!(self.sub_expr_call)                 |
            call_m!(self.sub_expr_nameconstant)         |
            call_m!(self.sub_expr_constant)             ) >>
        (expression)
    ));

    /// START(expr)
    tk_method!(start_expr_old, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        expression: alt_complete!(
            call_m!(self.sub_expr_binop)                |
            call_m!(self.sub_expr_call)                 |
            call_m!(self.sub_expr_nameconstant)         |
            call_m!(self.sub_expr_constant)             ) >>

        (expression)
    ));


    /// 1.   =  BoolOp(boolop op, expr* values)
    /// 2.   | BinOp(expr left, operator op, expr right)
    tk_method!(sub_expr_binop, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        // TODO: {T95} Enabled parser to handle nested expressions
        lhs: call_m!(self.sub_expr_constant)            >>
         op: binop_token                                >>
        rhs: call_m!(self.sub_expr_constant)            >>

        (Expr::BinOp {
            op: Op(op.as_owned_token()),
            left: Box::new(lhs),
            right: Box::new(rhs)
         })
    ));

    /// 16.  | Call(expr func, expr* args, keyword* keywords)
    tk_method!(sub_expr_call, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        // TODO: {T95} Enabled parser to handle nested expressions
        func_name: name_token                           >>
                   lparen_token                         >>
             args: call_m!(self.sub_expr_call_args)     >>
                   rparen_token                         >>

        (Expr::Call {
            func: func_name.as_owned_token(),
            args: args,
            keywords: (),
         })
    ));

    /// 22.  | NameConstant(singleton value)
    tk_method!(sub_expr_nameconstant, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        constant: alt_complete!(
            tag!(&[Id::True])                           |
            tag!(&[Id::False])                          |
            tag!(&[Id::None])                           ) >>

        (Expr::NameConstant(constant.as_owned_token()))
    ));

    /// 24.  | Constant(constant value)
    tk_method!(sub_expr_constant, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        constant: constant_token                        >>

        (Expr::Constant(constant.as_owned_token()))
    ));

    //    // 31.   └ attributes (int lineno, int col_offset)
    //    tk_method!(sub_expr_ended, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
    //        token: newline_token >>
    //        (Expr::End)
    //    ));


    /// Function Args Sub Expression Parser
    ///
    /// Create an optional pair tuple (TkSlice, Vec<TkSlice>) by matching
    /// against the special case of the single argument (e.g. `def hello(name):`
    /// and then the rest as the general case of argument names preceded by a comma
    /// (e.g. `def add_all(a, b, c, d, e, f):`.
    ///
    /// Notes:
    ///   1. Only supports positional arguments. (no *args, or **kwargs).
    ///   2.
    tk_method!(sub_expr_func_args, 'b, <Parser<'a>, Vec<Expr>>, mut self, do_parse!(
        opt_arg_names: opt!(pair!(
                            name_token,
                            many0!(preceded!(
                                    comma_token,
                                    name_token))))  >>

        ({
            match opt_arg_names {
                Some(arg_names) => {
                    let mut names: Vec<Expr> = Vec::new();
                    names.push(Expr::Constant(arg_names.0.as_owned_token()));
                    for tk in arg_names.1.iter() {
                        names.push(Expr::Constant(tk.as_owned_token()));
                    }

                    names
                },
                None => Vec::new()
            }
        })
    ));

    /// Call Args Sub Expression Parser
    ///
    /// Only supports positional arguments.
    ///
    /// Create an optional pair tuple (TkSlice, Vec<TkSlice>) by matching
    /// against the special case of the single argument (e.g. `hello(name):`
    /// and then the rest as the general case of argument names preceded by a comma
    /// (e.g. `def add_all(a, b, c, d, e, f):`.
    tk_method!(sub_expr_call_args, 'b, <Parser<'a>, Vec<Expr>>, mut self, do_parse!(
        opt_arg_names: opt!(pair!(call_m!(self.start_expr), many0!(
                        preceded!(
                            comma_token, call_m!(self.start_expr))))) >>
        ({
            match opt_arg_names {
                Some(arg_names) => {
                    let mut names: Vec<Expr> = Vec::new();
                    names.push(arg_names.0);
                    for tk in arg_names.1.iter() {
                        names.push(tk.clone());
                    }

                    names
                },
                None => Vec::new()
            }
        })
    ));


    tk_method!(sub_expr_lambda, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
              lambda_keyword                       >>
        args: call_m!(self.sub_expr_func_args)     >>
              colon_token                          >>
        body: call_m!(self.start_expr)             >>
        (Expr::Lambda {
            arguments: args,
            body: Box::new(body)
        })
    ));


    tk_method!(sub_expr_conditional, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        cons: many1!(not_if_keyword)                           >>
              if_keyword                                       >>
        cond: many1!(not_else_keyword)                         >>
              else_keyword                                     >>
         alt: call_m!(self.start_expr)                         >>
        expr: call_m!(self.build_conditional, cons, cond, alt) >>
       (expr)
    ));


    /// Offloads the work to parse the conditional subexpressions found in `sub_expr_conditional`.
    /// This could probably be done in that function however, there were issues with
    /// proper error type inference. In the future, a brave soul may wish to try to
    /// add a map!(..., ...) onto the subexpression matchers for purity.
    fn build_conditional<'b>(mut self, i: TkSlice<'b>,
                             cons: Vec<TkSlice<'b>>,
                             cond: Vec<TkSlice<'b>>,
                             alt: Expr
    ) -> (Parser<'a>, IResult<TkSlice<'b>, Expr>) {

        // Join the tokens in all tokens slices into a single TkSlice
        // and then try to parse that as an expression
        let cons: Vec<Tk<'b>> = cons.iter()
            .flat_map(TkSlice::iter)
            .map(|tk| *tk)
            .collect::<Vec<Tk<'b>>>();

        let consequent = match self.start_expr(TkSlice(&cons)) {
            (_, IResult::Done(ref remaining, ref expr)) => {
                expr.clone()
            },
            other => return (self, IResult::Error(ParserError::ConditionalExpr.code()))
        };

        // Join the tokens in all tokens slices into a single TkSlice
        // and then try to parse that as an expression
        let cond: Vec<Tk<'b>> = cond.iter()
            .flat_map(TkSlice::iter)
            .map(|tk| *tk)
            .collect::<Vec<Tk<'b>>>();

        let conditional = match self.start_expr(TkSlice(&cond)) {
            (_, IResult::Done(ref remaining, ref expr)) => {
                expr.clone()
            },
            other => return (self, IResult::Error(ParserError::ConditionalExpr.code()))
        };

        let o = Expr::Conditional {
            consequent: Box::new(consequent),
            condition: Box::new(conditional),
            alternative: Box::new(alt)
        };


        let f: IResult<TkSlice<'b>, Expr> = IResult::Done(i, o);
        (self, f)
    }

}


///
#[allow(unused_mut, dead_code, unused_imports)]
mod internal {
    use nom;
    use ::token::Id;
    use ::slice::TkSlice;
    use traits::redefs_nom::InputLengthRedef;

    // Specific constant and ast defined type tokens
    tk_named!(pub name_token        <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Name])));
    tk_named!(pub number_token      <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Number])));

    // Braces and Symbols
    tk_named!(pub lparen_token      <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::LeftParen])));
    tk_named!(pub rparen_token      <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::RightParen])));
    tk_named!(pub lbrace_token      <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::LeftBrace])));
    tk_named!(pub rbrace_token      <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::RightBrace])));
    tk_named!(pub lbracket_token    <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::LeftBracket])));
    tk_named!(pub rbracket_token    <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::RightBracket])));

    tk_named!(pub assign_token      <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Equal])));
    tk_named!(pub colon_token       <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Colon])));
    tk_named!(pub comma_token       <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Comma])));

    // Keyword Tokens
    tk_named!(pub async_keyword     <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Async])));
    tk_named!(pub await_keyword     <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Await])));
    tk_named!(pub def_keyword       <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Def])));
    tk_named!(pub class_keyword     <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Class])));
    tk_named!(pub if_keyword        <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::If])));
    tk_named!(pub else_keyword      <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Else])));
    tk_named!(pub elif_keyword      <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Elif])));
    tk_named!(pub lambda_keyword    <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Lambda])));
    tk_named!(pub return_keyword    <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Return])));

    // Special Whitespace
    tk_named!(pub newline_token     <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Newline])));
    
    // Artificial Tokens
    tk_named!(pub block_start       <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::BlockStart])));
    tk_named!(pub block_end         <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::BlockEnd])));


    // Flattened definitions to make the type inference happy to prevent this case:
    //
    // error[E0282]: type annotations needed
    //     --> src/parser.rs:404:5
    //     |
    //     404 |  tk_method!(sub_expr_conditional, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
    //     |   _____^
    //     |  |_____|
    //     | ||
    // 405 | ||         cons: many1!(not!(if_keyword))      >>
    // 406 | ||               if_keyword                    >>
    // 407 | ||         cond: many1!(not!(else_keyword))    >>
    // ...   ||
    // 413 | ||        (expr)
    // 414 | ||     ));
    //     | ||       ^
    //     | ||_______|
    //     |  |_______in this macro invocation
    //     |          cannot infer type for `E`
    //     |
    //     = note: this error originates in a macro outside of the current crate
    tk_named!(pub not_if_keyword    <TkSlice<'a>>, tk_is_none_of!(&[Id::If]));
    tk_named!(pub not_else_keyword  <TkSlice<'a>>, tk_is_none_of!(&[Id::Else]));


    /// Binary Operators: `a + b`, `a | b`, etc.
    tk_named!(pub binop_token <TkSlice<'a>>, ignore_spaces!(
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
    tk_named!(pub unaryop_token <TkSlice<'a>>, ignore_spaces!(
        alt_complete!(
            tag!(&[Id::Plus])               |
            tag!(&[Id::Minus])              |
            tag!(&[Id::Tilde])
        )
    ));

    tk_named!(pub constant_token <TkSlice<'a>>, ignore_spaces!(
        alt_complete!(
            tag!(&[Id::Name])               |
            tag!(&[Id::Number])             |
            tag!(&[Id::String])             |
            tag!(&[Id::RawString])          |
            tag!(&[Id::FormatString])       |
            tag!(&[Id::ByteString])
        )
    ));


    /// Augmented Assignment Operators: `a += b`, ` a <<= b`, etc.
    tk_named!(pub augassign_token <TkSlice<'a>>, ignore_spaces!(
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


    #[cft(test)]
    mod tests {
        use std::borrow::Borrow;
        use std::rc::Rc;

        use nom::IResult;

        use ::token::Tk;
        use ::Parser;
        use ::lexer::Lexer;
        use ::fmt;
        use super::*;

        #[test]
        fn assert_parsable() {
            let input = "something 1 2 3 if ";
            let mut parser = Parser::new();
            let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::new().tokenize(input.as_bytes());
            let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

            let result = match b {
                &IResult::Done(_, ref tokens) => {
                    println!("input: {}", input);
                    println!("{}", fmt::tokens(tokens, true));
                    not_if_keyword(TkSlice(&tokens))
                },
                _ => unreachable!()
            };


            match result {
                IResult::Done(ref out, ref tokens) => {
                    println!("unparsed:\n{:?}", out);
                    println!("parsed:\n{:?}", tokens);
                },
                _ => panic!()
            };
        }


    }
}


// TODO: {107} Add asserts to verify values of produced asts
#[cfg(test)]
mod tests {

    use std::borrow::Borrow;
    use std::rc::Rc;

    use nom::IResult;

    use ::lexer::Lexer;
    use ::fmt;
    use super::*;

    /// Use to create a named test case of a single line snippet of code.
    /// This `basic_test!(print_function, "print('hello world!')`
    /// will create a test function named `print_function` that will try to parse the
    /// string.
    macro_rules! basic_test {
        ($name:ident, $code:expr) => {
            #[test]
            fn $name() {
               assert_parsable($code);
            }
        };
    }

    fn assert_parsable(input: &str) {
        let mut parser = Parser::new();
        let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::new().tokenize(input.as_bytes());
        let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

        match b {
            &IResult::Done(_, ref tokens) => {
                println!("{}", input);
                println!("{}", fmt::tokens(tokens, true));
                let result = parser.parse_tokens(tokens);
                assert_complete(&result);
            },
            _ => unreachable!()
        }
    }

    fn assert_complete<'a>(result: &ParserResult) {
        match *result {
            ParserResult::Error(ref result) => {
                println!("{}", fmt::json(&result));
                panic!("AST Error")
            },
            ParserResult::Ok(ref result) => {
                println!("Ast(ok) \n{}", fmt::json(&result));
            },
        }
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

    // Stmt::Assign(Expr::BinOp)
    basic_test!(stmt_assign_expr_binop_add, "r = 1 + 3");
    basic_test!(stmt_assign_expr_binop_sub, "r = 2 - 3");
    basic_test!(stmt_assign_expr_binop_mul, "r = 'abc' * 3");
    basic_test!(stmt_assign_expr_binop_pow, "r = 5 ** 3");
    basic_test!(stmt_assign_expr_binop_lsh, "r = 23 << 44");
    basic_test!(stmt_assign_expr_binop_rsh, "r = 78 >> 3");
    basic_test!(stmt_assign_expr_binop_div, "r = 1.0 / 3");
    basic_test!(stmt_assign_expr_binop_fdv, "r = 6 // 3");
    basic_test!(stmt_assign_expr_binop_mod, "r = 34.4 % 3");

    // Stmt::Assign(Expr::Call)
    basic_test!(stmt_assign_expr_call,  "x = type(532j)");

    // Stmt::AugAssign(Expr::Call)
    basic_test!(stmt_augassign_expr_call,   "percent += float(string)");

    // Stmt::AugAssign(Expr::Constant)
    basic_test!(stmt_augassign_add, "r += 3");
    basic_test!(stmt_augassign_sub, "r -= 17.34");
    basic_test!(stmt_augassign_mul, "r *= '4'");
    basic_test!(stmt_augassign_pow, "r **= 5");
    basic_test!(stmt_augassign_lsh, "r <<= 44");
    basic_test!(stmt_augassign_rsh, "r >>= 3");
    basic_test!(stmt_augassign_div, "r /= 75");
    basic_test!(stmt_augassign_fdv, "r //= 98");
    basic_test!(stmt_augassign_mod, "r %= 34.4");

    // Expr::BinOp
    basic_test!(expr_binop_add, "1 + 3");
    basic_test!(expr_binop_sub, "2 - 3");
    basic_test!(expr_binop_mul, "4 * 3");
    basic_test!(expr_binop_pow, "5 ** 3");
    basic_test!(expr_binop_lsh, "23 << 44");
    basic_test!(expr_binop_rsh, "78 >> 3");
    basic_test!(expr_binop_div, "1.0 / 3");
    basic_test!(expr_binop_fdv, "6 // 3");
    basic_test!(expr_binop_mod, "34.4 % 3");

    // Expr::Call
    basic_test!(expr_call_3_dbl_quote_str,  r#"print("""He sings the songs that""")"#);
    basic_test!(expr_call_dbl_quote_str,    r#"hash("remind him of the good times")"#);
    basic_test!(expr_call_nargs,            r#"sum_all(1,2,3,3,4,5,6,7,8,'9')"#);
    basic_test!(expr_call_nested,           r#"int(str(sum(slice(list(range(1, 100)), 43))))"#);

    basic_test!(ast_multiple_stmts, r#"
f **= 14
g = 0x00123
fun = beer + jetski
"#);

    basic_test!(stmt_block_nested_func_defs, r#"
def hello():
    x = 1
    def potato():
        y = 2
        return "yup, a potato alright"
    return potato
"#);

    basic_test!(stmt_expr_line_cont, r#"
x = 1 + \
    2
"#);


    basic_test!(expr_lambda_01, r#"lambda: 1"#);
    basic_test!(expr_lambda_02, r#"lambda x: 'hello'"#);
    basic_test!(expr_lambda_03, r#"lambda: lambda: 1 if a else 2 if b else lambda: 3 if c else 4"#);

    basic_test!(expr_conditional_01, r#"1 if x else 2"#);

}
