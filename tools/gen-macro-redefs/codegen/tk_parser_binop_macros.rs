tk_method!(sub_expr_binop, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
    expression: alt_complete!(
          call_m!(self.sub_expr_binop_logicor) |
          call_m!(self.sub_expr_binop_logicand) |
          call_m!(self.sub_expr_binop_logicnot) |
          call_m!(self.sub_expr_binop_equality) |
          call_m!(self.sub_expr_binop_inequality) |
          call_m!(self.sub_expr_binop_is) |
          call_m!(self.sub_expr_binop_not_is) |
          call_m!(self.sub_expr_binop_in) |
          call_m!(self.sub_expr_binop_not_in) |
          call_m!(self.sub_expr_binop_lt) |
          call_m!(self.sub_expr_binop_lte) |
          call_m!(self.sub_expr_binop_gt) |
          call_m!(self.sub_expr_binop_gte) |
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


tk_method!(sub_expr_binop_logicnot, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_not_token)                     >>
         op: not_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_equality, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_doubleequal_token)                     >>
         op: doubleequal_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_inequality, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_notequal_token)                     >>
         op: notequal_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_is, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_is_token)                     >>
         op: is_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_not_is, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_isnot_token)                     >>
         op: isnot_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_in, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_in_token)                     >>
         op: in_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_not_in, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_notin_token)                     >>
         op: notin_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_lt, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_less_token)                     >>
         op: less_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_lte, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_lessorequal_token)                     >>
         op: lessorequal_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_gt, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_greater_token)                     >>
         op: greater_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));


tk_method!(sub_expr_binop_gte, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_greaterorequal_token)                     >>
         op: greaterorequal_token                                 >>
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
tk_named!(pub not_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::Not])));
tk_named!(pub doubleequal_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::DoubleEqual])));
tk_named!(pub notequal_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::NotEqual])));
tk_named!(pub is_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::Is])));
tk_named!(pub isnot_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::IsNot])));
tk_named!(pub in_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::In])));
tk_named!(pub notin_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::NotIn])));
tk_named!(pub less_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::Less])));
tk_named!(pub lessorequal_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::LessOrEqual])));
tk_named!(pub greater_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::Greater])));
tk_named!(pub greaterorequal_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::GreaterOrEqual])));
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


tk_named!(pub not_or_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Or, Id::Newline]));
tk_named!(pub not_and_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::And, Id::Newline]));
tk_named!(pub not_not_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Not, Id::Newline]));
tk_named!(pub not_doubleequal_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::DoubleEqual, Id::Newline]));
tk_named!(pub not_notequal_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::NotEqual, Id::Newline]));
tk_named!(pub not_is_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Is, Id::Newline]));
tk_named!(pub not_isnot_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::IsNot, Id::Newline]));
tk_named!(pub not_in_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::In, Id::Newline]));
tk_named!(pub not_notin_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::NotIn, Id::Newline]));
tk_named!(pub not_less_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Less, Id::Newline]));
tk_named!(pub not_lessorequal_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::LessOrEqual, Id::Newline]));
tk_named!(pub not_greater_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Greater, Id::Newline]));
tk_named!(pub not_greaterorequal_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::GreaterOrEqual, Id::Newline]));
tk_named!(pub not_pipe_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Pipe, Id::Newline]));
tk_named!(pub not_caret_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Caret, Id::Newline]));
tk_named!(pub not_amp_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Amp, Id::Newline]));
tk_named!(pub not_leftshift_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::LeftShift, Id::Newline]));
tk_named!(pub not_rightshift_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::RightShift, Id::Newline]));
tk_named!(pub not_plus_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Plus, Id::Newline]));
tk_named!(pub not_minus_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Minus, Id::Newline]));
tk_named!(pub not_star_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Star, Id::Newline]));
tk_named!(pub not_at_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::At, Id::Newline]));
tk_named!(pub not_slash_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Slash, Id::Newline]));
tk_named!(pub not_doubleslash_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::DoubleSlash, Id::Newline]));
tk_named!(pub not_percent_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::Percent, Id::Newline]));
tk_named!(pub not_doublestar_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::DoubleStar, Id::Newline]));
