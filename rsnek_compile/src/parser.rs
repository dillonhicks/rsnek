use std::borrow::Borrow;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

use nom;
use nom::{IResult, Slice, Compare, CompareResult, FindToken, ErrorKind};

use lexer::Lexer;
use fmt;
use token::{Id, Tk, Tag, OwnedTk, pprint_tokens, New, BLOCK_START, BLOCK_END, TK_BLOCK_END, TK_BLOCK_START, NEWLINE};
use slice::{TkSlice};
use ast::{self, Ast, Module, Stmt, Expr, DynExpr, Op, FnType};
use traits::redefs_nom::InputLength;

const INDENT_STACK_SIZE: usize = 100;


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

                (j, item.find_token($arr))

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


// TODO: {T91} move to rsnek_runtime::macros
macro_rules! strings_error_indent_mismatch {
    ($len:expr, $indent:expr) => {
        format!("SPAN LEN {} IS NOT A MULTIPLE OF YOUR MOMS SPEED DIAL NUMBER {}", $len, $indent);
    }
}

// TODO: {T91} move to rsnek_runtime::macros
macro_rules! strings_error_indent_overflow {
    ($max:expr) => {
        format!("Number of INDENT is more than the max allowed {}", $max);
    }
}

trait Preprocessor<T, V> {
    fn transform(&self, input: T) -> Result<V, String>;
}

#[derive(Debug, Clone, Copy, Serialize, Default)]
struct BlockScopeProcessor;

impl BlockScopeProcessor {
    fn new() -> Self {
        BlockScopeProcessor {}
    }

    /// Determine the length of an indent in spaces.
    fn determine_indent<'a>(&self, tokens: &TkSlice<'a>) -> usize {

        let mut start: Option<usize> = None;
        let mut last: (usize, Tk<'a>) = (0, Tk::default());

        for (idx, tk) in tokens.iter().enumerate() {
            match (last.1.id(), tk.id(), start) {
                (Id::Newline, Id::Space, _) => start = Some(idx),
                (Id::Space, Id::Newline, _) => {
                    start = None;
                },
                (Id::Space, id, Some(s)) => {
                    if id != Id::Space && id != Id::Newline {
                        return tokens.slice(s..idx).len();
                    }
                },
                _ => {}
            };

            last = (idx, tk.clone());
        }

        return 0
    }

    ///
    #[inline]
    fn balance_scopes<'b>(&self, span_len: usize, indent: usize,
                          stack_idx_start: usize, indent_stack: &mut [usize],
                          acc: &mut Vec<TkSlice<'b>>) -> Result<usize, String> {

        let mut stack_idx = stack_idx_start;

        match indent_stack[stack_idx] {
            curr if span_len == curr + indent => {
                if INDENT_STACK_SIZE <= stack_idx + 1 {
                    return Err(strings_error_indent_overflow!(INDENT_STACK_SIZE))
                }
                stack_idx += 1;
                indent_stack[stack_idx] = curr + indent;
                //println!("Emit: {:?}", TK_BLOCK_START.id());
                acc.push(TkSlice(&BLOCK_START));
            },
            curr if span_len < curr => {
                'backtrack: while stack_idx != 0 {
                    stack_idx -= 1;

                    //println!("Emit: {:?}", TK_BLOCK_END.id());
                    acc.push(TkSlice(&BLOCK_END));

                    if indent_stack[stack_idx] == span_len {
                        break 'backtrack;
                    }
                }
            },
            _ => {
                //println!("No change in self.indent stack");
                acc.push(TkSlice(&NEWLINE));
            }
        }

        Ok(stack_idx)
    }
}

impl<'a> Preprocessor<TkSlice<'a>, Box<[Tk<'a>]>> for BlockScopeProcessor {

    fn transform<'b>(&self, tokens: TkSlice<'b>) -> Result<Box<[Tk<'b>]>, String> {
        let indent = self.determine_indent(&tokens);
        //println!("Indent is len: {}", indent);

        if indent == 0 {
            return Ok(tokens.tokens().to_owned().into_boxed_slice());
        }

        let mut acc: Vec<TkSlice<'b>> = Vec::new();

        let mut stack_idx = 0;
        let mut indent_stack: [usize; INDENT_STACK_SIZE]  = [0; INDENT_STACK_SIZE];

        let mut start: Option<usize>        = None;
        let mut end: Option<usize>          = None;
        let mut is_continuation             = false;
        let mut seen_non_ws                 = false;

        let mut last_tk: (usize, Tk<'b>)    = (0, Tk::default());

        for (idx, tk) in tokens.iter().enumerate() {
            match (last_tk.1.id(), tk.id(), is_continuation, seen_non_ws, start) {
                // Just seen a newline on the last pass and it is not a \ escaped
                // continuation so start collapsing the whitespace
                (Id::Newline, Id::Space, false, false, None) => {
                    if let Some(e) = end{
                        acc.push(tokens.slice(e..idx - 1))
                    } else {
                        acc.push(tokens.slice(..idx - 1));
                    }

                    start = Some(idx);
                    end = None;
                },
                // Continue to consume spaces
                (Id::Space,   Id::Space, false, false, _) => {},
                // Found the first non ws char
                (Id::Space, id, false, false, Some(s)) if id != Id::Space => {
                    // TODO: {T92} Formalize preprocessing to allow for injection of expression start
                    // and end
                    let span = tokens.slice(s..idx);

                    seen_non_ws = true;
                    start = None;
                    end = Some(idx);

                    //println!("Found self.indent span: {:?}", span.len());
                    if span.len() % indent != 0 {
                        return Err(strings_error_indent_mismatch!(span.len(), indent));
                    }

                    match self.balance_scopes(span.len(), indent, stack_idx, &mut indent_stack, &mut acc) {
                        Ok(new_stack_idx) => stack_idx = new_stack_idx,
                        Err(string) => return Err(string)
                    };

                    acc.push(span);
                },
                // A top level scope cannot close its scopes until it finds the next one
                (Id::Newline, id, false, false, _) if id != Id::Space && id != Id::Newline => {
                    // TODO: {T92} Formalize preprocessing to allow for injection of expression start
                    // and end
                    if let Some(e) = end{
                        acc.push(tokens.slice(e..idx - 1))
                    } else {
                        acc.push(tokens.slice(..idx - 1));
                    }

                    seen_non_ws = true;
                    start = None;
                    end = Some(idx);

                    match self.balance_scopes(0, indent, stack_idx, &mut indent_stack, &mut acc) {
                        Ok(new_stack_idx) => stack_idx = new_stack_idx,
                        Err(string) => return Err(string)
                    };

                },
                // Continuation case.
                (_, Id::Backslash, _, _, _) => {
                    // TODO: {T92} Formalize preprocessing to allow for injection of expression start
                    // and end
                    // TODO: {T93} Handle backslash continuations... rewrite the
                    //   newline masked as a Id::Space??
                },
                // Normal newline
                (_, Id::Newline, false, _, _) => {
                    seen_non_ws = false;
                },
                _ => {}
            };

            last_tk = (idx, tk.clone());
        }

        // Put the rest of the tokens in the acc if the loops did not end on a
        // scope boundary.
        match (start, end) {
            (Some(i), _) |
            (_, Some(i)) => acc.push(tokens.slice(i..)),
            _ => unreachable!()
        };

        'emit_trailing_end_scopes: while stack_idx != 0 {
            stack_idx -= 1;
            acc.push(TkSlice(&BLOCK_END));
        }

        //println!("ENDDEBUG: {:?} {:?}", start, end);
        // TODO: {T94} remove this debug shit. This dumps the contents of the accumulator
        // as a string
        let strings: String = acc.iter()
            .map(TkSlice::as_string)
            .collect::<Vec<String>>()
            .concat();
//        println!("CONCAT: {}", strings);

        let scoped_tokens = acc.iter()
            .flat_map(TkSlice::iter)
            .map(Tk::clone)
            .collect::<Vec<Tk<'b>>>();

//        for t in &scoped_tokens {
//            println!("{}", fmt::token(&t));
//        }

        Ok(scoped_tokens.into_boxed_slice())
    }

}

#[derive(Debug, Copy, Clone, Serialize, Default)]
struct ParserState<'a> {
    line: usize,
    column: usize,
    indent: usize,
    //preprocessors: Vec<&'a Preprocessor<TkSlice<'a>>>,
    unused: Option<TkSlice<'a>>
}

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


#[derive(Debug, Copy, Clone, Serialize)]
pub struct Parser<'a> {
    state: ParserState<'a>,
}


impl<'a> Parser<'a> {

    pub fn new() -> Self {
        let mut state = ParserState::default();
        Parser { state: state }
    }

    /// Public wrapper to the macro generated tkslice_to_ast which will take a slice of
    /// tokens, turn those into a TkSlice, and parse that into an AST.
    pub fn parse_tokens<'b, 'c>(&mut self, tokens: &'b [Tk<'b>]) -> ParserResult {

        let bspp = BlockScopeProcessor::new();
        let boxed_tks: Box<[Tk<'b>]> = bspp.transform(TkSlice(tokens)).unwrap();
        // Dereference the box to remove the indirection and get the real
        // address to the slice.
        let slice = TkSlice(&(*boxed_tks));
        let result = self.tkslice_to_ast(slice).1;

        // TODO: {T94} Try to incorporate error messages here
        let presult = match result {
            IResult::Error(ref error) => {
                // TODO: {T94} Consume error in parse result in some useful message
                ParserResult::Error(
                    ParsedAst {
                        ast: Ast::default(),
                        remaining_tokens: Vec::new(),
                        p1_tokens: tokens.iter().map(OwnedTk::from).collect(),
                        p2_tokens: boxed_tks.iter().map(OwnedTk::from).collect()
                    })
            }
            IResult::Incomplete(_) => {
                // TODO: {T94} nom::Needed enum has some extra info about parsing
                ParserResult::Error(
                    ParsedAst {
                        ast: Ast::default(),
                        remaining_tokens: Vec::new(),
                        p1_tokens: tokens.iter().map(OwnedTk::from).collect(),
                        p2_tokens: boxed_tks.iter().map(OwnedTk::from).collect()
                    })
            },
            IResult::Done(ref remaining, ref ast) if remaining.len() == 0 => {
                ParserResult::Ok(
                    ParsedAst {
                        ast: ast.clone(),
                        remaining_tokens: Vec::new(),
                        p1_tokens: tokens.iter().map(OwnedTk::from).collect(),
                        p2_tokens: boxed_tks.iter().map(OwnedTk::from).collect()
                    })
            },
            IResult::Done(ref remaining, ref ast) => {
                ParserResult::Error(
                    ParsedAst {
                        ast: ast.clone(),
                        remaining_tokens: remaining.iter().map(OwnedTk::from).collect(),
                        p1_tokens: tokens.iter().map(OwnedTk::from).collect(),
                        p2_tokens: boxed_tks.iter().map(OwnedTk::from).collect()
                    })
            }
        };

        presult
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

    // Example of keeping parser state
    fn inc_lineno(&mut self) {
        self.state.line += 1;
        //println!("{}", fmt::json(&self));
    }

    // AST Builders

    /// START(ast)
    tk_method!(tkslice_to_ast, 'b, <Parser<'a>, Ast>, mut self, do_parse!(
        ast: alt!(
            call_m!(self.module_start)      => { |m: Module | (Ast::Module(m))     } |
            ignore_spaces!(
                call_m!(self.stmt_start))   => { |r: Stmt   | (Ast::Statement(r))  } ) >>
        (ast)
    ));

    tk_method!(module_start, 'b, <Parser<'a>, Module>, mut self, do_parse!(
        body: many0!(call_m!(self.stmt_start)) >>
        (Module::Body(body))
    ));

    /// START(stmt)
    tk_method!(stmt_start, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
        statement: alt!(
            call_m!(self.sub_stmt_funcdef)     |
            call_m!(self.sub_stmt_block)       |
            call_m!(self.sub_stmt_return)      |
            call_m!(self.sub_stmt_assign)      |
            call_m!(self.sub_stmt_augassign)   |
            call_m!(self.sub_stmt_expr)        |
            call_m!(self.sub_stmt_next_line)   ) >>
        (statement)
    ));

    /// Functions are just some window dressing on blocks
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
                arguments: args //Vec::new() /vec![Expr::Constant(margs.as_owned_token())]//args.iter().map(|ts| Expr::Constant(ts.as_owned_token())).collect()
           })
    ));

    /// Blocks are a unit of nesting that can contain many statements including
    /// other nested blocks and functions and stuff.
    ///
    /// Note that they do not have a representation in Grammar.txt
    tk_method!(sub_stmt_block, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
               block_start                         >>
        stmts: many0!(call_m!(self.stmt_start))    >>
               block_end                           >>
        (Stmt::Block(stmts))
    ));

    /// 4.   | Return(expr? value)
    tk_method!(sub_stmt_return, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
        return_keyword                          >>
        value: opt!(call_m!(self.start_expr))   >>
        (Stmt::Return(value))
    ));

    /// 5.   | Assign(expr* targets, expr value)
    tk_method!(sub_stmt_assign, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
         // TODO: {T95} Enabled parser to handle nested expressions
        target: name_token                  >>
                assign_token                >>
         value: call_m!(self.start_expr)    >>
        (Stmt::Assign {
            target: Expr::Constant(target.as_owned_token()),
            value: value
         })
    ));

    /// 6.   | AugAssign(expr target, operator op, expr value)
    tk_method!(sub_stmt_augassign, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
        // TODO: {T95} Enabled parser to handle nested expressions
        target: name_token       >>
            op: augassign_token  >>
        value: call_m!(self.start_expr)  >>
        (Stmt::AugAssign {
            op: Op(op.as_owned_token()),
            target: Expr::Constant(target.as_owned_token()),
            value: value
         })
    ));

    /// 20.   | Expr(expr value)
    tk_method!(sub_stmt_expr, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
        expression: call_m!(self.start_expr) >>
        (Stmt::Expr(expression))
    ));


    /// 22.   └ attributes (int lineno, int col_offset)
    /// Inject a empty statement for the next line
    tk_method!(sub_stmt_next_line, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
        newline_token >>
        ({self.inc_lineno(); Stmt::Newline})
    ));


    /// START(expr)
    tk_method!(start_expr, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        expression: alt_complete!(
            call_m!(self.sub_expr_binop)        |
            call_m!(self.sub_expr_call)         |
            call_m!(self.sub_expr_nameconstant) |
            call_m!(self.sub_expr_constant)     ) >>
        (expression)
    ));


    /// 1.   =  BoolOp(boolop op, expr* values)
    /// 2.   | BinOp(expr left, operator op, expr right)
    tk_method!(sub_expr_binop, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        // TODO: {T95} Enabled parser to handle nested expressions
        lhs: call_m!(self.sub_expr_constant)  >>
         op: binop_token                      >>
        rhs: call_m!(self.sub_expr_constant)  >>
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
            tag!(&[Id::True])     |
            tag!(&[Id::False])    |
            tag!(&[Id::None])     ) >>
        (Expr::NameConstant(constant.as_owned_token()))
    ));

    /// 24.  | Constant(constant value)
    tk_method!(sub_expr_constant, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        constant: constant_token >>
        (Expr::Constant(constant.as_owned_token()))
    ));

    // 31.   └ attributes (int lineno, int col_offset)
//    tk_method!(sub_expr_ended, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
//        token: newline_token >>
//        (Expr::End)
//    ));

    tk_method!(sub_expr_func_args, 'b, <Parser<'a>, Vec<Expr>>, mut self, do_parse!(
        opt_arg_names: opt!(pair!(name_token, many0!(
                        preceded!(
                            comma_token, name_token)))) >>

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
tk_named!(name_token    <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Name])));
tk_named!(number_token  <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Number])));
tk_named!(assign_token  <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Equal])));
tk_named!(newline_token <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Newline])));
tk_named!(lparen_token  <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::LeftParen])));
tk_named!(rparen_token  <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::RightParen])));
tk_named!(colon_token   <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Colon])));
tk_named!(comma_token   <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Comma])));

// Special tokens
tk_named!(async_keyword     <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Async])));
tk_named!(await_keyword     <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Await])));
tk_named!(def_keyword       <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Def])));
tk_named!(if_keyword        <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::If])));
tk_named!(else_keyword      <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Else])));
tk_named!(elif_keyword      <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Elif])));
tk_named!(return_keyword    <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Return])));
tk_named!(block_start       <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::BlockStart])));
tk_named!(block_end         <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::BlockEnd])));


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
            tag!(&[Id::Number])             |
            tag!(&[Id::String])             |
            tag!(&[Id::RawString])          |
            tag!(&[Id::FormatString])       |
            tag!(&[Id::ByteString])
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


// TODO: {107} Add asserts to verify values of produced asts
#[cfg(test)]
mod tests {

    use std::borrow::Borrow;
    use std::rc::Rc;

    use nom::IResult;
    use serde_json;
    use ast::Ast;
    use lexer::Lexer;
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
                pprint_tokens(tokens);
                let result = parser.parse_tokens(tokens);
                assert_complete(&tokens, &result);
            },
            _ => unreachable!()
        }
    }

    fn assert_complete<'a>(tokens: &Vec<Tk<'a>>, result: &ParserResult) {
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


    #[test]
    fn determine_indent() {
        let input = r#"
def things():
    if thing:
        thing2
        thing3
    pass


def morethings():
    pass

print("hello world")
"#;
        let mut parser = Parser::new();
        let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::new().tokenize(input.as_bytes());
        let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

        match b {
            &IResult::Done(_, ref tokens) => {
                parser.parse_tokens(tokens);
            },
            _ => unreachable!()
        }
    }

    #[test]
    fn funcs_and_blocks() {
        let input = r#"
def hello():
    x = 1
    def potato():
        y = 2
        return "yup, a potato alright"
    return potato

"#;
        let mut parser = Parser::new();
        let r: Rc<IResult<&[u8], Vec<Tk>>> = Lexer::new().tokenize(input.as_bytes());
        let b: &IResult<&[u8], Vec<Tk>> = r.borrow();

        match b {
            &IResult::Done(_, ref tokens) => {
                match parser.parse_tokens(tokens) {
                    ParserResult::Ok(parsed) => {
                        println!("{}", fmt::json(&parsed.ast));
                    },
                    result => println!("{}", fmt::json(&result))
                }
            },
            _ => unreachable!()
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
    
}
