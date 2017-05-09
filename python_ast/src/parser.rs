//! Take a slice of tokens `TkSlice` and convert it into an `Ast`.
use nom;
use nom::{IResult, ErrorKind, Err};

use ::token::{Id, Tk, Tag, OwnedTk};
use ::slice::{TkSlice};
use ::ast::{Ast, Module, Stmt, Expr, Op, FnType};
use ::traits::redefs_nom::InputLengthRedef;
use ::preprocessor::{Preprocessor, BlockScopePreprocessor};

use self::internal::*;


// Hacks to splice in an is not token
const IS_NOT_BYTES: &'static [u8] = &[105, 115, 32, 110, 111, 116];
const IS_NOT_TKSLICE: TkSlice<'static>  = TkSlice(&[Tk::const_(Id::IsNot, IS_NOT_BYTES, Tag::None)]);

/// The result type returned by `Parser`. Both `Ok` and `Error` variants contain an
/// instance of `ParsedAst`.  `Ok` variants are considered to be the case where the
/// `TkSlice` was fully consumed.
#[derive(Debug, Clone, Serialize)]
pub enum ParserResult {
    Ok(ParsedAst),
    Error(ParsedAst),
}


/// Wraps an `Ast` to give extra debug information when bits hit the fan
#[derive(Debug, Clone, Serialize)]
pub struct ParsedAst{
    /// This is your father's Ast. An elegant representation for a more
    /// civilized compilation.
    pub ast: Ast,

    /// Tokens received by the parser
    #[serde(skip_serializing)]
    pub p1_tokens: Vec<OwnedTk>,

    /// The tokens after the first phase of pre processing
    pub p2_tokens: Vec<OwnedTk>,

    /// Tokens that were not consumed by the parser. Useful to debug
    /// where the parsing stopped.
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
            p2_tokens: p2_tokens.iter().map(OwnedTk::from).collect::<Vec<OwnedTk>>()
        }
    }
}


#[derive(Debug, Copy, Clone, Serialize, Default)]
struct ParserState<'a> {
    /// Track number of `\n` seen in an erratic and
    /// inconsistent manner
    line: usize,
    column: usize,
    indent: usize,
    unused: Option<TkSlice<'a>>
}


/// The custom result that happens when the parser is unable to parse
/// a nested expression.
#[repr(u32)]
enum ParserError {
    SubExpr = 1024,
}


impl ParserError {
    pub const fn code<'a>(self) -> Err<TkSlice<'a>, u32> {
        Err::Code(ErrorKind::Custom(self as u32))
    }
}


/// Create a Python AST from slice of Tokens created from the `lexer::Lexer`.
///
#[derive(Debug, Copy, Clone, Serialize)]
pub struct Parser<'a> {
    state: ParserState<'a>,
}


#[allow(unused_mut, dead_code, unused_imports)]
impl<'a> Parser<'a> {

    /// Create a new instance of `Parser` eager to consume `TkSlice` instances.
    pub fn new() -> Self {
        let state = ParserState::default();
        Parser { state: state }
    }

    /// Public wrapper to the macro generated tkslice_to_ast which will take a slice of
    /// tokens, turn those into a TkSlice, and parse that into an AST.
    pub fn parse_tokens<'b, 'c>(&mut self, tokens: &'b [Tk<'b>]) -> ParserResult {

        let p1_tokens = TkSlice(tokens);

        let bspp = BlockScopePreprocessor::new();
        let bspp_tokens: Box<[Tk<'b>]> = match bspp.transform(p1_tokens) {
            Ok(boxed_tks) => boxed_tks,
            Err(err) => {
                warn!("Had to eat error due to unspecific return type";
                    "Error" => format!("{}", err));

                return ParserResult::Error(
                    ParsedAst::new(None, None, TkSlice(tokens), &[]));
            }
        };
        let p2_tokens = &(*bspp_tokens);
        
        
        // The (&(*(val))) pattern is to dereference the box to remove the indirection and
        // get the real address to the slice instead of the address to the box.
        //  *(box -> value) => value
        //  &(value) => ptr value
        let slice = TkSlice(p2_tokens);
        let result = self.tkslice_to_ast(slice).1;


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
    fn inc_lineno(&mut self) -> usize{
        self.state.line += 1;
        self.state.line
    }

    // AST Parser-Builders - they map approximately with Grammar.txt exceptions
    // and extensions are noted.
    //

    /// START(ast)
    tk_method!(tkslice_to_ast, 'b, <Parser<'a>, Ast>, mut self, do_parse!(
        ast: alt!(
            call_m!(self.module_start)  => { |m: Module | (Ast::Module(m))     } |
            call_m!(self.stmt_start)    => { |r: Stmt   | (Ast::Statement(r))  } ) >>

        (ast)
    ));

    /// START(mod)
    tk_method!(module_start, 'b, <Parser<'a>, Module>, mut self, do_parse!(
        body: many0!(call_m!(self.stmt_start))                  >>

        (Module::Body(body))
    ));


    /// START(stmt)
    tk_method!(stmt_start, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
        statement: ignore_spaces!(alt!(
            call_m!(self.sub_stmt_funcdef)                      |
            call_m!(self.sub_stmt_block)                        |
            call_m!(self.sub_stmt_return)                       |
            call_m!(self.sub_stmt_assign)                       |
            call_m!(self.sub_stmt_augassign)                    |
            call_m!(self.sub_stmt_assert)                       |
            call_m!(self.sub_stmt_expr)                         |
            call_m!(self.sub_stmt_next_line)                    )) >>

        (statement)
    ));

    ///1.    = FunctionDef(identifier name, arguments args,
    ///
    /// Functions are just some fancy window dressing on blocks
    ///
    /// ```python
    /// def call_me(maybe):
    ///     return 1
    /// ```
    tk_method!(sub_stmt_funcdef, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
                   def_keyword                                  >>
        func_name: name_token                                   >>
                   lparen_token                                 >>
            args:  call_m!(self.sub_expr_func_args)             >>
                   rparen_token                                 >>
                   colon_token                                  >>
       body_block: call_m!(self.sub_stmt_block)                 >>

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
               block_start                                      >>
        stmts: many0!(call_m!(self.stmt_start))                 >>
               block_end                                        >>

        (Stmt::Block(stmts))
    ));

    /// 4.   | Return(expr? value)
    ///
    /// ```python
    /// return
    /// ```
    ///
    /// and
    ///
    /// ```python
    /// return to_sender
    /// ```
    tk_method!(sub_stmt_return, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
        return_keyword                                          >>
        value: opt!(call_m!(self.start_expr))                   >>

        (Stmt::Return(value))
    ));

    /// 5.   | Assign(expr* targets, expr value)
    ///
    /// ```python
    /// orange = 'you glad I didnt say banana?'
    /// ```
    tk_method!(sub_stmt_assign, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
        target: name_token                                      >>
                assign_token                                    >>
         value: call_m!(self.start_expr)                        >>

        (Stmt::Assign {
            target: Expr::Constant(target.as_owned_token()),
            value: value
         })
    ));

    /// 6.   | AugAssign(expr target, operator op, expr value)
    ///
    /// ```python
    /// x += 15
    /// ```
    tk_method!(sub_stmt_augassign, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
        target: name_token                                      >>
            op: augassign_token                                 >>
        value: call_m!(self.start_expr)                         >>

        (Stmt::AugAssign {
            op: Op(op.as_owned_token()),
            target: Expr::Constant(target.as_owned_token()),
            value: value
         })
    ));

    /// 16.1   | Assert(expr test, expr? msg)
    ///
    /// ```python
    /// assert [1,2,3,4]
    /// ```
    tk_method!(sub_stmt_assert, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
                 assert_keyword                                 >>
            stmt: alt_complete!(
                    call_m!(self.sub_stmt_assert_2arg)          |
                    map!(
                        call_m!(self.start_expr),
                        |expr: Expr| {
                            Stmt::Assert {
                                test:expr,
                                message: None
                                }
                        })                                      )>>

            (stmt)
    ));

    /// 16.2 - an assert with an additional expression as a message
    ///
    /// ```python
    /// assert False, "halp!"
    /// ```
    tk_method!(sub_stmt_assert_2arg, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
           test: terminated!(
                    call_m!(self.start_expr),
                    comma_token)                            >>
        message: call_m!(self.start_expr)                   >>

        (Stmt::Assert {
            test: test,
            message: Some(message)
        })
    ));


    /// 20.   | Expr(expr value)
    tk_method!(sub_stmt_expr, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
        expression: call_m!(self.start_expr)                    >>

        (Stmt::Expr(expression))
    ));


    /// 22.   └ attributes (int lineno, int col_offset)
    /// Inject a empty statement for the next line
    tk_method!(sub_stmt_next_line, 'b, <Parser<'a>, Stmt>, mut self, do_parse!(
        newline_token                                           >>

        (Stmt::Newline(self.inc_lineno()))
    ));



    /// START(expr)
    tk_method!(pub start_expr, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        expression: alt_complete!(
            call_m!(self.sub_expr_lambda)                       |
            call_m!(self.sub_expr_conditional)                  |
            call_m!(self.sub_expr_call)                         |
            call_m!(self.sub_expr_getattr)                      |
            call_m!(self.sub_expr_list)                         |
            call_m!(self.sub_expr_dict)                         |
            call_m!(self.sub_expr_operator)                     |
            call_m!(self.sub_expr_nameconstant)                 |
            call_m!(self.sub_expr_constant)                     ) >>
        (expression)
    ));

    /// 4.   | Lambda(arguments args, expr body)
    ///
    /// ```python
    /// lambda fleece: fleece == 'white as snow'
    /// ```
    tk_method!(sub_expr_lambda, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
              lambda_keyword                                    >>
        args: call_m!(self.sub_expr_func_args)                  >>
              colon_token                                       >>
        body: call_m!(self.start_expr)                          >>

        (Expr::Lambda {
            arguments: args,
            body: Box::new(body)
        })
    ));

    /// 5.   | IfExp(expr test, expr body, expr orelse)
    ///
    /// ```python
    /// cant_handle if True else ignorance_is_bliss
    /// ```
    tk_method!(sub_expr_conditional, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        cons: many1!(not_if_keyword)                            >>
              if_keyword                                        >>
        cond: many1!(not_else_keyword)                          >>
              else_keyword                                      >>
         alt: call_m!(self.start_expr)                          >>
        expr: call_m!(self.build_conditional, cons, cond, alt)  >>
       (expr)
    ));

    /// 1.   = BoolOp(boolop op, expr* values)
    /// 2.   | BinOp(expr left, operator op, expr right)
    /// 3.   | UnaryOp(unaryop op, expr operand)
    ///
    ///  Uses the order of the explicit rules to do a variation on
    ///  using a greedily consuming recursive decent with backtracking.
    ///  Note that this implementation considers boolop to be a binary op.
    tk_method!(sub_expr_operator, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        expression: alt_complete!(
          call_m!(self.sub_expr_boolop_logic_or)                |
          call_m!(self.sub_expr_boolop_logic_and)               |
          call_m!(self.sub_expr_binop_equality)                 |
          call_m!(self.sub_expr_binop_inequality)               |
          call_m!(self.sub_expr_binop_is)                       |
          call_m!(self.sub_expr_binop_in)                       |
          call_m!(self.sub_expr_binop_not_in)                   |
          call_m!(self.sub_expr_unaryop_logicnot)               |
          call_m!(self.sub_expr_unaryop_neg)                    |
          call_m!(self.sub_expr_binop_lt)                       |
          call_m!(self.sub_expr_binop_lte)                      |
          call_m!(self.sub_expr_binop_gt)                       |
          call_m!(self.sub_expr_binop_gte)                      |
          call_m!(self.sub_expr_binop_or)                       |
          call_m!(self.sub_expr_binop_xor)                      |
          call_m!(self.sub_expr_binop_and)                      |
          call_m!(self.sub_expr_binop_lshift)                   |
          call_m!(self.sub_expr_binop_rshift)                   |
          call_m!(self.sub_expr_binop_add)                      |
          call_m!(self.sub_expr_binop_sub)                      |
          call_m!(self.sub_expr_binop_mul)                      |
          call_m!(self.sub_expr_binop_matmul)                   |
          call_m!(self.sub_expr_binop_truediv)                  |
          call_m!(self.sub_expr_binop_floordiv)                 |
          call_m!(self.sub_expr_binop_mod)                      |
          // NOTE: The power operator ** binds less tightly than an arithmetic or
          // bitwise unary operator on its right, that is, 2**-1 is 0.5.
          call_m!(self.sub_expr_binop_pow)                      ) >>

       (expression)
    ));


    /// 1.1.  `a or b`
    tk_method!(sub_expr_boolop_logic_or, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_or_token)                               >>
         op: or_token                                           >>
        rhs: call_m!(self.start_expr)                           >>
       expr: call_m!(self.build_binop, op, lhs, rhs)            >>

        (expr)
    ));


    /// 1.2. `a and b`
    tk_method!(sub_expr_boolop_logic_and, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_and_token)                              >>
         op: and_token                                          >>
        rhs: call_m!(self.start_expr)                           >>
       expr: call_m!(self.build_binop, op, lhs, rhs)            >>

        (expr)
    ));


    /// 2.1. `a | b`
    tk_method!(sub_expr_binop_or, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_pipe_token)                             >>
         op: pipe_token                                         >>
        rhs: call_m!(self.start_expr)                           >>
       expr: call_m!(self.build_binop, op, lhs, rhs)            >>

        (expr)
    ));


    /// 2.2. `a ^ b`
    tk_method!(sub_expr_binop_xor, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_caret_token)                            >>
         op: caret_token                                        >>
        rhs: call_m!(self.start_expr)                           >>
       expr: call_m!(self.build_binop, op, lhs, rhs)            >>

        (expr)
    ));

    /// 2.3. `a & b`
    tk_method!(sub_expr_binop_and, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_amp_token)                              >>
         op: amp_token                                          >>
        rhs: call_m!(self.start_expr)                           >>
       expr: call_m!(self.build_binop, op, lhs, rhs)            >>

        (expr)
    ));

    ///  `not a`
    tk_method!(sub_expr_unaryop_logicnot, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
             op: not_token                                      >>
        operand: call_m!(self.start_expr)                       >>
        (Expr::UnaryOp {
            op: Op(op.as_owned_token()),
            operand: Box::new(operand)
        })
    ));

    ///  `-a`
    tk_method!(sub_expr_unaryop_neg, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
             op: minus_token                                    >>
        operand: call_m!(self.start_expr)                       >>
        (Expr::UnaryOp {
            op: Op(op.as_owned_token()),
            operand: Box::new(operand)
        })
    ));

    ///  `a == b`
    tk_method!(sub_expr_binop_equality, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_doubleequal_token)                      >>
        op: doubleequal_token                                   >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));

    /// `a != b`
    tk_method!(sub_expr_binop_inequality, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_notequal_token)                         >>
         op: notequal_token                                     >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));

    /// `a is (not?) b`
    tk_method!(sub_expr_binop_is, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_is_token)                               >>
         op: alt_complete!(
                // inline subparser to ensure `is not`
                // is captured regardless of the spaces
                // between 'is' and 'not' which can not
                // be done with a tag.
                do_parse!(is_token      >>
                          not_token     >>
                      (IS_NOT_TKSLICE))     |
                is_token)                                       >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));

    /// `a in b`
    tk_method!(sub_expr_binop_in, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_in_token)                               >>
         op: in_token                                           >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));

    /// `a not in b`
    tk_method!(sub_expr_binop_not_in, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_notin_token)                            >>
         op: notin_token                                        >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));

    /// `a < b`
    tk_method!(sub_expr_binop_lt, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_less_token)                             >>
         op: less_token                                         >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));

    /// `a <= b`
    tk_method!(sub_expr_binop_lte, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_lessorequal_token)                      >>
         op: lessorequal_token                                  >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));

    /// `a > b`
    tk_method!(sub_expr_binop_gt, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_greater_token)                          >>
         op: greater_token                                      >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));

    /// `a >= b`
    tk_method!(sub_expr_binop_gte, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_greaterorequal_token)                   >>
         op: greaterorequal_token                               >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));

    /// 2.4. `a << b`
    tk_method!(sub_expr_binop_lshift, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_leftshift_token)                        >>
         op: leftshift_token                                    >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));


    /// 2.5. `a >> b`
    tk_method!(sub_expr_binop_rshift, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_rightshift_token)                       >>
         op: rightshift_token                                   >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));


    /// 2.6. `a + b`
    tk_method!(sub_expr_binop_add, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_plus_token)                             >>
         op: plus_token                                         >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));


    /// 2.7. `a - b`
    tk_method!(sub_expr_binop_sub, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_minus_token)                            >>
         op: minus_token                                        >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));

    /// 2.8. `a * b`
    tk_method!(sub_expr_binop_mul, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_star_token)                             >>
         op: star_token                                         >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));


    /// 2.9. `a @ b`
    tk_method!(sub_expr_binop_matmul, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_at_token)                               >>
         op: at_token                                           >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));


    /// 2.10. `a / b`
    tk_method!(sub_expr_binop_truediv, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_slash_token)                            >>
         op: slash_token                                        >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));


    /// 2.11. `a // b`
    tk_method!(sub_expr_binop_floordiv, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_doubleslash_token)                      >>
         op: doubleslash_token                                  >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));


    /// 2.12. `a % b`
    tk_method!(sub_expr_binop_mod, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_percent_token)                          >>
         op: percent_token                                      >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));

    /// 2.13. `a ** b`
    tk_method!(sub_expr_binop_pow, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_doublestar_token)                       >>
         op: doublestar_token                                   >>
        rhs: call_m!(self.start_expr)                           >>
        expr: call_m!(self.build_binop, op, lhs, rhs)           >>

        (expr)
    ));


    /// 16.  | Call(expr func, expr* args, keyword* keywords)
    ///
    /// `a(b)`
    tk_method!(sub_expr_call, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        // TODO: func_name needs to be a scan and build like
        // many1!(not_doublestar_token) because (1+2)() is valid in the
        // grammar.
        func_name: name_token                                   >>
                   lparen_token                                 >>
             args: call_m!(self.sub_expr_call_args)             >>
                   rparen_token                                 >>

        (Expr::Call {
            func: func_name.as_owned_token(),
            args: args,
            keywords: (),
         })
    ));


    /// 25.  | Attribute(expr value, identifier attr, expr_context ctx)
    ///
    /// `a.b`
    tk_method!(sub_expr_getattr, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        name: name_token                                        >>
        expr: fold_many1!(
                preceded!(dot_token, name_token),
                Expr::Constant(name.as_owned_token()),
                |acc, attr: TkSlice<'b>| {
                    Expr::Attribute {
                        value: Box::new(acc),
                        attr: attr.as_owned_token()
                    }
                  }                                             )>>

        (expr)
    ));


    /// 29. | List(expr* elts, expr_context ctx)
    ///
    /// `[a, b, ...]`
    tk_method!(sub_expr_list, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
               lbracket_token                                   >>
        elems: call_m!(self.sub_expr_call_args)                 >>
               rbracket_token                                   >>
        (Expr::List { elems: elems })
    ));

    tk_method!(sub_expr_dict, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        expr: alt_complete!(
            do_parse!(
                       lbrace_token                         >>
                items: call_m!(self.sub_expr_dict_items)    >>
                       rbrace_token                         >>
                (Expr::Dict { items: items}))                   |
            do_parse!(
                 lbrace_token                               >>
                 rbrace_token                               >>
                 (Expr::Dict {items: Vec::new()})
            )) >>
        (expr)
    ));

    /// 22.  | NameConstant(singleton value)
    tk_method!(sub_expr_nameconstant, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        constant: alt_complete!(
            tag!(&[Id::True])                                   |
            tag!(&[Id::False])                                  |
            tag!(&[Id::None])                                   ) >>

        (Expr::NameConstant(constant.as_owned_token()))
    ));

    /// 24.  | Constant(constant value)
    tk_method!(sub_expr_constant, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        constant: constant_token                                >>

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
    tk_method!(sub_expr_func_args, 'b, <Parser<'a>, Vec<Expr>>, mut self, do_parse!(
        opt_arg_names: opt!(pair!(
                                name_token,
                                many0!(
                                    preceded!(
                                        comma_token,
                                        name_token))))          >>

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
    /// Create an optional pair tuple (TkSlice, Vec<TkSlice>) by matching
    /// against the special case of the single argument (e.g. `hello(name):`
    /// and then the rest as the general case of argument names preceded by a comma
    /// (e.g. `def add_all(a, b, c, d, e, f):`.
    ///
    /// Notes:
    ///   1. Only supports positional arguments. (no *args, or **kwargs).
    tk_method!(sub_expr_call_args, 'b, <Parser<'a>, Vec<Expr>>, mut self, do_parse!(
        opt_arg_names: opt!(pair!(
                            call_m!(self.start_expr),
                            many0!(
                                preceded!(
                                    comma_token,
                                    call_m!(self.start_expr))))
                                                                ) >>
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

    /// Call Args Sub Expression Parser
    ///
    tk_method!(sub_expr_dict_items, 'b, <Parser<'a>, Vec<(Expr, Expr)>>, mut self, do_parse!(
        items: separated_list!(comma_token, call_m!(self.sub_expr_dict_item)) >>
        (items)
    ));

    tk_method!(sub_expr_dict_item, 'b, <Parser<'a>, (Expr, Expr)>, mut self, do_parse!(
        key: many1!(not_colon_token)                      >>
        colon_token                                  >>
        value: call_m!(self.start_expr)                     >>
        item: call_m!(self.build_dict_item, key, value)    >>
        (item)
    ));

    /// Helper for the recursive parsing of a subexpression as is common in
    /// in cases where something has an infix-y semantic. For example:
    /// ```x = 1 if y else 2``` or ```q = [i for i in entries if i >= 3]```
    fn parse_sub_expr<'b>(mut self,
                          slices: &'b [TkSlice<'b>]
    ) -> Result<Expr, ParserError> {

        // Flatten the tokens in all tokens slices into a single TkSlice
        // and then try to parse that as an expression
        let flattened: Vec<Tk<'b>> = slices.iter()
            .flat_map(TkSlice::iter)
            .map(|tk| *tk)
            .collect::<Vec<Tk<'b>>>();

        match self.start_expr(TkSlice(&flattened)) {
            (_, IResult::Done(_, expr)) => Ok(expr),
            _ => Err(ParserError::SubExpr)
        }
    }

    /// Offload the work to parse tokens in the LHS here instead of in the binop parsers
    /// as a way to get around type inference hell. See `build_conditional`.
    fn build_binop<'b>(mut self,
                       i: TkSlice<'b>,
                       op: TkSlice<'b>,
                       lhs: Vec<TkSlice<'b>>,
                       rhs: Expr) -> (Parser<'a>, IResult<TkSlice<'b>, Expr>) {

        let left = match self.parse_sub_expr(&lhs) {
            Ok(expr) => expr,
            Err(error) => return (self, IResult::Error(error.code()))
        };

        let binop_expr = Expr::BinOp {
            op: Op(op.as_owned_token()),
            left: Box::new(left),
            right: Box::new(rhs)
        };

        let result: IResult<TkSlice<'b>, Expr> = IResult::Done(i, binop_expr);
        (self, result)
    }

    /// Offloads the work to parse the conditional subexpressions found in `sub_expr_conditional`.
    /// This could probably be done in that function however, there were issues with
    /// proper error type inference. In the future, a brave soul may wish to try to
    /// add a map!(..., ...) onto the subexpression matchers for purity.
    fn build_conditional<'b>(mut self, i: TkSlice<'b>,
                             cons: Vec<TkSlice<'b>>,
                             cond: Vec<TkSlice<'b>>,
                             alt: Expr
    ) -> (Parser<'a>, IResult<TkSlice<'b>, Expr>) {

        let consequent = match self.parse_sub_expr(&cons) {
            Ok(expr) => expr,
            Err(error) => return (self, IResult::Error(error.code()))
        };

        let conditional = match self.parse_sub_expr(&cond) {
            Ok(expr) => expr,
            Err(error) => return (self, IResult::Error(error.code()))
        };

        let cond_expr = Expr::Conditional {
            consequent: Box::new(consequent),
            condition: Box::new(conditional),
            alternative: Box::new(alt)
        };

        let result: IResult<TkSlice<'b>, Expr> = IResult::Done(i, cond_expr);
        (self, result)
    }

    fn build_dict_item<'b>(mut self,
                           i: TkSlice<'b>,
                           key: Vec<TkSlice<'b>>,
                           value: Expr) -> (Parser<'a>, IResult<TkSlice<'b>, (Expr, Expr)>) {

        let key_expr = match self.parse_sub_expr(&key) {
            Ok(expr) => expr,
            Err(error) => return (self, IResult::Error(error.code()))
        };

        let result: IResult<TkSlice<'b>, (Expr, Expr)> = IResult::Done(i, (key_expr, value));
        (self, result)
    }
}


#[allow(unused_mut, dead_code, unused_imports)]
mod internal {
    use nom;
    use ::token::Id;
    use ::slice::TkSlice;
    use traits::redefs_nom::InputLengthRedef;

    // Specific constant and ast defined type tokens
    tk_named!(pub name_token        <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Name])));
    tk_named!(pub number_token      <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Number])));
    tk_named!(pub string_token      <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::String])));

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
    tk_named!(pub assert_keyword     <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Assert])));
    tk_named!(pub async_keyword     <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Async])));
    tk_named!(pub await_keyword     <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Await])));
    tk_named!(pub def_keyword       <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Def])));
    tk_named!(pub class_keyword     <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Class])));
    tk_named!(pub if_keyword        <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::If])));
    tk_named!(pub else_keyword      <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Else])));
    tk_named!(pub elif_keyword      <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Elif])));
    tk_named!(pub lambda_keyword    <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Lambda])));
    tk_named!(pub return_keyword    <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Return])));

    // Operators
    tk_named!(pub or_token          <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Or])));
    tk_named!(pub and_token         <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::And])));

    tk_named!(pub pipe_token        <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Pipe])));
    tk_named!(pub caret_token       <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Caret])));
    tk_named!(pub amp_token         <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Amp])));
    tk_named!(pub not_token         <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Not])));
    tk_named!(pub doubleequal_token <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::DoubleEqual])));
    tk_named!(pub notequal_token    <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::NotEqual])));
    tk_named!(pub is_token          <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Is])));
    tk_named!(pub isnot_token       <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Is, Id::Not])));
    tk_named!(pub in_token          <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::In])));
    tk_named!(pub notin_token       <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Not, Id::In])));
    tk_named!(pub less_token        <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::LeftAngle])));
    tk_named!(pub lessorequal_token <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::LessOrEqual])));
    tk_named!(pub greater_token     <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::RightAngle])));
    tk_named!(pub greaterorequal_token  <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::GreaterOrEqual])));
    tk_named!(pub leftshift_token   <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::LeftShift])));
    tk_named!(pub rightshift_token  <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::RightShift])));
    tk_named!(pub plus_token        <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Plus])));
    tk_named!(pub minus_token       <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Minus])));
    tk_named!(pub star_token        <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Star])));
    tk_named!(pub at_token          <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::At])));
    tk_named!(pub slash_token       <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Slash])));
    tk_named!(pub doubleslash_token <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::DoubleSlash])));
    tk_named!(pub percent_token     <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Percent])));
    tk_named!(pub doublestar_token  <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::DoubleStar])));
    tk_named!(pub dot_token         <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Dot])));

    // Special Whitespace
    tk_named!(pub newline_token     <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::Newline])));
    
    // Artificial Tokens
    tk_named!(pub block_start       <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::BlockStart])));
    tk_named!(pub block_end         <TkSlice<'a>>, ignore_spaces!(tag!(&[Id::BlockEnd])));


    // Flattened not definitions to make the type inference happy to prevent this case:
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
    tk_named!(pub not_if_keyword        <TkSlice<'a>>,  tk_is_none_of!(&[Id::If, Id::Newline]));
    tk_named!(pub not_else_keyword      <TkSlice<'a>>,  tk_is_none_of!(&[Id::Else, Id::Newline]));

    tk_named!(pub not_or_token          <TkSlice<'a>>,  tk_is_none_of!(&[Id::Or, Id::Newline]));
    tk_named!(pub not_and_token         <TkSlice<'a>>,  tk_is_none_of!(&[Id::And, Id::Newline]));
    tk_named!(pub not_pipe_token        <TkSlice<'a>>,  tk_is_none_of!(&[Id::Pipe, Id::Newline]));
    tk_named!(pub not_caret_token       <TkSlice<'a>>,  tk_is_none_of!(&[Id::Caret, Id::Newline]));
    tk_named!(pub not_amp_token         <TkSlice<'a>>,  tk_is_none_of!(&[Id::Amp, Id::Newline]));
    tk_named!(pub not_not_token         <TkSlice<'a>>,  tk_is_none_of!(&[Id::Not, Id::Newline]));
    tk_named!(pub not_doubleequal_token <TkSlice<'a>>,  tk_is_none_of!(&[Id::DoubleEqual, Id::Newline]));
    tk_named!(pub not_notequal_token    <TkSlice<'a>>,  tk_is_none_of!(&[Id::NotEqual, Id::Newline]));
    tk_named!(pub not_is_token          <TkSlice<'a>>,  tk_is_none_of!(&[Id::Is, Id::Newline]));
    tk_named!(pub not_in_token          <TkSlice<'a>>,  tk_is_none_of!(&[Id::In, Id::Newline]));
    tk_named!(pub not_notin_token       <TkSlice<'a>>,  tk_is_none_of!(&[Id::Not, Id::In, Id::Newline]));
    tk_named!(pub not_less_token        <TkSlice<'a>>,  tk_is_none_of!(&[Id::LeftAngle, Id::Newline]));
    tk_named!(pub not_lessorequal_token <TkSlice<'a>>,  tk_is_none_of!(&[Id::LessOrEqual, Id::Newline]));
    tk_named!(pub not_greater_token     <TkSlice<'a>>,  tk_is_none_of!(&[Id::RightAngle, Id::Newline]));
    tk_named!(pub not_greaterorequal_token <TkSlice<'a>>,  tk_is_none_of!(&[Id::GreaterOrEqual, Id::Newline]));
    tk_named!(pub not_leftshift_token   <TkSlice<'a>>,  tk_is_none_of!(&[Id::LeftShift, Id::Newline]));
    tk_named!(pub not_rightshift_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::RightShift, Id::Newline]));
    tk_named!(pub not_plus_token        <TkSlice<'a>>,  tk_is_none_of!(&[Id::Plus, Id::Newline]));
    tk_named!(pub not_minus_token       <TkSlice<'a>>,  tk_is_none_of!(&[Id::Minus, Id::Newline]));
    tk_named!(pub not_star_token        <TkSlice<'a>>,  tk_is_none_of!(&[Id::Star, Id::Newline]));
    tk_named!(pub not_at_token          <TkSlice<'a>>,  tk_is_none_of!(&[Id::At, Id::Newline]));
    tk_named!(pub not_slash_token       <TkSlice<'a>>,  tk_is_none_of!(&[Id::Slash, Id::Newline]));
    tk_named!(pub not_doubleslash_token <TkSlice<'a>>,  tk_is_none_of!(&[Id::DoubleSlash, Id::Newline]));
    tk_named!(pub not_percent_token     <TkSlice<'a>>,  tk_is_none_of!(&[Id::Percent, Id::Newline]));
    tk_named!(pub not_doublestar_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::DoubleStar, Id::Newline]));
    tk_named!(pub not_dot_token         <TkSlice<'a>>,  tk_is_none_of!(&[Id::Dot, Id::Newline]));
    tk_named!(pub not_colon_token       <TkSlice<'a>>,  tk_is_none_of!(&[Id::Colon, Id::Newline]));

    /// Unary Operatos: `+`, `-`,
    tk_named!(pub unaryop_token <TkSlice<'a>>, ignore_spaces!(
        alt_complete!(
            tag!(&[Id::Plus])               |
            tag!(&[Id::Minus])              |
            tag!(&[Id::Tilde])
        )
    ));

    ///
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
    /// string. It will panic if not all of the tokens are consumed.
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
            _ => panic!("Unable to tokenize input")
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

    // Stmt::Assert
    basic_test!(stmt_assert_01, "assert True, 'ok!'");

    // Expr::UnaryOp
    basic_test!(expr_unaryop_logicnot,    "not a");
    basic_test!(expr_unaryop_neg,         "-a");


    // Expr::BinOp
    basic_test!(expr_binop_logicor,     "a or b");
    basic_test!(expr_binop_logicand,    "a and b");
    basic_test!(expr_binop_equality,    "f == 13");
    basic_test!(expr_binop_inequality,  "g != '13'");
    basic_test!(expr_binop_is,          "None is None");
    basic_test!(expr_binop_isnot,       "True is not False");
    basic_test!(expr_binop_lt,          "0.0 < 1.23");
    basic_test!(expr_binop_le,          "f <= z");
    basic_test!(expr_binop_gt,          "[1,2,3] > [0]");
    basic_test!(expr_binop_ge,          "'rust+python' >= 'all the rest'");
    basic_test!(expr_binop_and, "1 & 2");
    basic_test!(expr_binop_or,  "y | w");
    basic_test!(expr_binop_xor, "a ^ b");
    basic_test!(expr_binop_add, "1 + 3");
    basic_test!(expr_binop_sub, "2 - 3");
    basic_test!(expr_binop_mul, "4 * 3");
    basic_test!(expr_binop_pow, "5 ** 3");
    basic_test!(expr_binop_lsh, "23 << 44");
    basic_test!(expr_binop_rsh, "78 >> 3");
    basic_test!(expr_binop_div, "1.0 / 3");
    basic_test!(expr_binop_fdv, "6 // 3");
    basic_test!(expr_binop_mod, "34.4 % 3");

    basic_test!(expr_binop_n1, "1 + 1 + 2");
    basic_test!(expr_binop_n2, "c << 1 + 2");
    basic_test!(expr_binop_n3, "1 - 1 ** 2");
    basic_test!(expr_binop_n4, "1 + 1 // 2");
    basic_test!(expr_binop_n5, "1 + 1 // 2 * 'hello' ^ 'world' ");


    // Expr::Call
    basic_test!(expr_call_3_dbl_quote_str,  r#"print("""He sings the songs that""")"#);
    basic_test!(expr_call_dbl_quote_str,    r#"hash("remind him of the good times")"#);
    basic_test!(expr_call_nargs,            r#"sum_all(1,2,3,3,4,5,6,7,8,'9')"#);
    basic_test!(expr_call_nested,           r#"int(str(sum(slice(list(range(1, 100)), 43))))"#);

    // Expr::Lambda
    basic_test!(expr_lambda_01, r#"lambda: 1"#);
    basic_test!(expr_lambda_02, r#"lambda x: 'hello'"#);
    basic_test!(expr_lambda_03, r#"lambda: lambda: 1 if a else 2 if b else lambda: 3 if c else 4"#);

    // Expr::Conditional
    basic_test!(expr_conditional_01, r#"1 if x else 2"#);
    basic_test!(expr_conditional_02, r#"1 if x else f() if y else z ** 34"#);

    // Expr::Attribute
    basic_test!(expr_attribute_01, r#"object.attribute"#);
    basic_test!(expr_attribute_02, r#"a.b.c"#);
    basic_test!(expr_attribute_03, r#"a.b.c.d"#);

    // Expr::List
    basic_test!(expr_list_01, r#"[]"#);
    basic_test!(expr_list_02, r#"[a]"#);
    basic_test!(expr_list_03, r#"[a, b, c]"#);
    basic_test!(expr_list_04, r#"[int("234"), 5 << 3, [1]]"#);
    // TODO: {T118} Binop Scanning Wrecks Args and Elems
    basic_test!(expr_list_05, r#"[3, [a,b,c,len([1,2,3])], x + y]"#);

    // Expr::Dict
    basic_test!(expr_dict_01, r#"{}"#);
    basic_test!(expr_dict_02, r#"{a: b}"#);
    basic_test!(expr_dict_03, r#"{a: {b: c}}"#);
    basic_test!(expr_dict_04, r#"{2**8: 1, True: True, False: True, "f": {"dict": "bad"}, tuple([1,2,3,4]): 34.2}"#);


    // Sanity Checks
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

    basic_test!(empty_str, r#"f = ''"#);
}
