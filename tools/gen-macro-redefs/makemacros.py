from pathlib import Path

import fire
from rutabaga import Renderer


_render = Renderer()


@_render.context_for('{{type_prefix}}_macros.rs')
def ctx():
    return {
        'type_prefix': "tk",
        'crate_traits': "use $crate::traits::redefs_nom::InputLength;",
        'input_slice_type_explicit_lifetime': "TkSlice<'a>",
        'input_slice_type_implicit_lifetime': "TkSlice<'a>"
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
