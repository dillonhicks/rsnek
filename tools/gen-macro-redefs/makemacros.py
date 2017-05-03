from typing import NamedTuple
from pathlib import Path

import fire
from rutabaga import Renderer


_render = Renderer()


@_render.context_for('(.+)[.]rs', regex=True)
def ctx():
    return {
        'type_prefix': "tk",
        'crate_traits': "use $crate::traits::redefs_nom::InputLength;",
        'path_input_slice_type': "slice::TkSlice",
        'input_slice_type_explicit_lifetime': "TkSlice<'a>",
        'input_slice_type_implicit_lifetime': "TkSlice<'a>"
    }

# lambda	Lambda expression
# if – else	Conditional expression
# or	Boolean OR
# and	Boolean AND
# not x	Boolean NOT
# in, not in, is, is not, <, <=, >, >=, !=, ==	Comparisons, including membership tests and identity tests
# |	Bitwise OR
# ^	Bitwise XOR
# &	Bitwise AND
# <<, >>	Shifts
# +, -	Addition and subtraction
# *, @, /, # , %	Multiplication, matrix multiplication division, remainder [5]
# +x, -x, ~x	Positive, negative, bitwise NOT
# **	Exponentiation [6]
# await x	Await expression
# x[index], x[index:index], x(arguments...), x.attribute	Subscription, slicing, call, attribute reference
# (expressions...), [expressions...], {key: value...}, {expressions...}	Binding or tuple display, list display, dictionary display, set display
# const VALUE_MARKER = 1;
class Op(NamedTuple):
    name: str
    id: str
    precedent: int

MAX = 1000

@_render.context_for('(.+)binop_macros[.]rs', regex=True)
def ctx():
    return {
        'binops': list(reversed(sorted([
            Op('logicor',   'Or',           MAX - 3),
            Op('logicand',  'And',          MAX - 4),
            Op('logicnot',  'Not',          MAX - 5),
            Op('equality',  'DoubleEqual',  MAX - 6),
            Op('or',        'Pipe',         MAX - 7),
            Op('xor',       'Caret',        MAX - 8),
            Op('and',       'Amp',          MAX - 9),
            Op('lshift',    'LeftShift',    MAX - 10),
            Op('rshift',    'RightShift',   MAX - 11),
            Op('add',       'Plus',         MAX - 12),
            Op('sub',       'Minus',        MAX - 13),
            Op('mul',       'Star',         MAX - 14),
            Op('matmul',    'At',           MAX - 15),
            Op('truediv',   'Slash',        MAX - 16),
            Op('floordiv',  'DoubleSlash',  MAX - 17),
            Op('mod',       'Percent',      MAX - 18),
            Op('pow',       'DoubleStar',   MAX - 19),
        ], key=lambda op: op.precedent)))
    }


class CLI:
    def render(self, src):
        dest = Path('codegen')
        dest.mkdir(parents=True, exist_ok=True)

        _render.render_file(Path(src), dest)


if __name__ == '__main__':
    import logging
    logging.basicConfig(level=logging.INFO)
    fire.Fire(CLI)
