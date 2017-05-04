tk_method!(sub_expr_binop, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
    expression: alt_complete!(
          {% for binop in binops %}call_m!(self.sub_expr_binop_{{binop.name.lower()}}) |
          {% endfor %}
     ) >>

     (expression)
));

{% for binop in binops %}
tk_method!(sub_expr_binop_{{binop.name.lower()}}, 'b, <Parser<'a>, Expr>, mut self, do_parse!(
        lhs: many1!(not_{{binop.id.lower()}}_token)                     >>
         op: {{binop.id.lower()}}_token                                 >>
        rhs: call_m!(self.start_expr)                   >>
        expr: call_m!(self.build_binop, op, lhs, rhs)   >>

        (expr)
));

{% endfor %}

{% for binop in binops %}
tk_named!(pub {{binop.id.lower()}}_token  <TkSlice<'a>>,    ignore_spaces!(tag!(&[Id::{{binop.id}}])));{% endfor %}

{% for binop in binops %}
tk_named!(pub not_{{binop.id.lower()}}_token  <TkSlice<'a>>,  tk_is_none_of!(&[Id::{{binop.id}}, Id::Newline]));{% endfor %}

