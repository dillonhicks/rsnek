tk_method!(sub_expr_binop, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
    expression: alt_complete!(
          call_m!(self.sub_expr_binop_logicor) |
          call_m!(self.sub_expr_binop_logicand) |
          call_m!(self.sub_expr_binop_or) |
          call_m!(self.sub_expr_binop_xor) |
          call_m!(self.sub_expr_binop_and) |
          call_m!(self.sub_expr_binop_lshift) |
          call_m!(self.sub_expr_binop_rshift) |
          call_m!(self.sub_expr_binop_add) |
          call_m!(self.sub_expr_binop_sub) |
          call_m!(self.sub_expr_binop_mul) |
          call_m!(self.sub_expr_binop_matmul) |
          call_m!(self.sub_expr_binop_truediv) |
          call_m!(self.sub_expr_binop_floordiv) |
          call_m!(self.sub_expr_binop_mod) |
          call_m!(self.sub_expr_binop_pow) |
          
     ) >>

     (expression)
));


tk_method!(sub_expr_binop_logicor, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_or_token)                     >>
         op: or_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_logicand, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_and_token)                     >>
         op: and_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_or, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_pipe_token)                     >>
         op: pipe_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_xor, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_caret_token)                     >>
         op: caret_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_and, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_amp_token)                     >>
         op: amp_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_lshift, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_leftshift_token)                     >>
         op: leftshift_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_rshift, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_rightshift_token)                     >>
         op: rightshift_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_add, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_plus_token)                     >>
         op: plus_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_sub, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_minus_token)                     >>
         op: minus_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_mul, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_star_token)                     >>
         op: star_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_matmul, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_at_token)                     >>
         op: at_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_truediv, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_slash_token)                     >>
         op: slash_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_floordiv, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_doubleslash_token)                     >>
         op: doubleslash_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_mod, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_percent_token)                     >>
         op: percent_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_pow, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_doublestar_token)                     >>
         op: doublestar_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));




tk_named!(pub or_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::Or])));
tk_named!(pub and_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::And])));
tk_named!(pub pipe_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::Pipe])));
tk_named!(pub caret_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::Caret])));
tk_named!(pub amp_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::Amp])));
tk_named!(pub leftshift_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::LeftShift])));
tk_named!(pub rightshift_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::RightShift])));
tk_named!(pub plus_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::Plus])));
tk_named!(pub minus_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::Minus])));
tk_named!(pub star_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::Star])));
tk_named!(pub at_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::At])));
tk_named!(pub slash_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::Slash])));
tk_named!(pub doubleslash_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::DoubleSlash])));
tk_named!(pub percent_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::Percent])));
tk_named!(pub doublestar_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::DoubleStar])));


tk_named!(pub not_or_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Or]));
tk_named!(pub not_and_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::And]));
tk_named!(pub not_pipe_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Pipe]));
tk_named!(pub not_caret_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Caret]));
tk_named!(pub not_amp_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Amp]));
tk_named!(pub not_leftshift_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::LeftShift]));
tk_named!(pub not_rightshift_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::RightShift]));
tk_named!(pub not_plus_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Plus]));
tk_named!(pub not_minus_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Minus]));
tk_named!(pub not_star_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Star]));
tk_named!(pub not_at_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::At]));
tk_named!(pub not_slash_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Slash]));
tk_named!(pub not_doubleslash_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::DoubleSlash]));
tk_named!(pub not_percent_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Percent]));
tk_named!(pub not_doublestar_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::DoubleStar]));
