#!/usr/bin/env python
from phabricator import Phabricator


def make_titles(obj):
    return [
        f"{obj} Hashed: __hash__",
        f"{obj} StringCast: __string__",
        f"{obj} BytesCast: __bytes__",
        f"{obj} StringRepresentation: __repr__",
        f"{obj} StringFormat: __format__",
        f"{obj} Equal: __eq__",
        f"{obj} NotEqual: __ne__",
        f"{obj} LessThan: __lt__",
        f"{obj} LessOrEqual: __le__",
        f"{obj} GreaterOrEqual: __ge__",
        f"{obj} GreaterThan: __gt__",
        f"{obj} BooleanCast: __bool__",
        f"{obj} IntegerCast: __int__",
        f"{obj} FloatCast: __float__",
        f"{obj} ComplexCast: __complex__",
        f"{obj} Rounding: __round__",
        f"{obj} Index: __index__",
        f"{obj} NegateValue: __neg__",
        f"{obj} AbsValue: __abs__",
        f"{obj} PositiveValue: __pos__",
        f"{obj} InvertValue: __invert__",
        f"{obj} Add: __add__",
        f"{obj} BitwiseAnd: __and__",
        f"{obj} DivMod: __divmod__",
        f"{obj} FloorDivision: __floordiv__",
        f"{obj} LeftShift: __lshift__",
        f"{obj} Modulus: __mod__",
        f"{obj} Multiply: __mul__",
        f"{obj} MatrixMultiply: __matmul__",
        f"{obj} BitwiseOr: __or__",
        f"{obj} Pow: __pow__",
        f"{obj} RightShift: __rshift__",
        f"{obj} Subtract: __sub__",
        f"{obj} TrueDivision: __truediv__",
        f"{obj} XOr: __xor__",
        f"{obj} ReflectedAdd: __radd__",
        f"{obj} ReflectedBitwiseAnd: __rand__",
        f"{obj} ReflectedDivMod: __rdivmod__",
        f"{obj} ReflectedFloorDivision: __rfloordiv__",
        f"{obj} ReflectedLeftShift: __rlshift__",
        f"{obj} ReflectedModulus: __rmod__",
        f"{obj} ReflectedMultiply: __rmul__",
        f"{obj} ReflectedMatrixMultiply: __rmatmul__",
        f"{obj} ReflectedBitwiseOr: __ror__",
        f"{obj} ReflectedPow: __rpow__",
        f"{obj} ReflectedRightShift: __rrshift__",
        f"{obj} ReflectedSubtract: __rsub__",
        f"{obj} ReflectedTrueDivision: __rtruediv__",
        f"{obj} ReflectedXOr: __rxor__",
        f"{obj} InPlaceAdd: __iadd__",
        f"{obj} InPlaceBitwiseAnd: __iand__",
        f"{obj} InPlaceDivMod: __idivmod__",
        f"{obj} InPlaceFloorDivision: __ifloordiv__",
        f"{obj} InPlaceLeftShift: __ilshift__",
        f"{obj} InPlaceModulus: __imod__",
        f"{obj} InPlaceMultiply: __imul__",
        f"{obj} InPlaceMatrixMultiply: __imatmul__",
        f"{obj} InPlaceBitwiseOr: __ior__",
        f"{obj} InPlacePow: __ipow__",
        f"{obj} InPlaceRightShift: __irshift__",
        f"{obj} InPlaceSubtract: __isub__",
        f"{obj} InPlaceTrueDivision: __itruediv__",
        f"{obj} InPlaceXOr: __ixor__",
        f"{obj} Contains: __contains__",
        f"{obj} Iter: __iter__",
        f"{obj} Call: __call__",
        f"{obj} Length: __len__",
        f"{obj} LengthHint: __length_hint__",
        f"{obj} Next: __next__",
        f"{obj} Reversed: __reversed__",
        f"{obj} GetItem: __getitem__",
        f"{obj} SetItem: __setitem__",
        f"{obj} DeleteItem: __delitem__",
        f"{obj} Count: count",
        f"{obj} Append: append",
        f"{obj} Extend: extend",
        f"{obj} Pop: pop",
        f"{obj} Remove: remove",
        f"{obj} IsDisjoint: isdisjoint",
        f"{obj} AddItem: add",
        f"{obj} Discard: discard",
        f"{obj} Clear: clear",
        f"{obj} Get: get",
        f"{obj} Keys: keys",
        f"{obj} Values: values",
        f"{obj} Items: items",
        f"{obj} PopItem: popitem",
        f"{obj} Update: update",
        f"{obj} SetDefault: setdefault",
        f"{obj} Await: __await__",
        f"{obj} Send: send",
        f"{obj} Throw: throw",
        f"{obj} Close: close",
        f"{obj} Exit: __exit__",
        f"{obj} Enter: __enter__",
        f"{obj} DescriptorGet: __get__",
        f"{obj} DescriptorSet: __set__",
        f"{obj} DescriptorSetName: __set_name__",
    ]

objects = [
    #"Float",
    #"Integer",
    # "Complex",
    # "List",
    # "Tuple",
    # "Boolean",
    # "String",
    # "Bytes",
    # "Frame",
    # "Generator",
    # "Iterator",
    # "Dictionary",
    # "Set",
    # "FrozenSet",
    # "Code",
    # "Range",
    # "Slice",
    # "Descriptor",
    # "Class",
    # "File",
    # "Function",
    # "MemoryView",
    # "Type",
    # "WeakRef",
    # "Exception",
    # "Enum",
    #"Module",
]

builtins = [
    "abs",	"dict",	"help",	"min",	"setattr",
    "all",	"dir",	"hex",	"next",	"slice",
    "any",	"divmod",	"id",	"object",	"sorted",
    "ascii",	"enumerate",	"input",	"oct",	"staticmethod",
    "bin",	"eval",	"int",	"open",	"str",
    "bool",	"exec",	"isinstance",	"ord",	"sum",
"bytearray",	"filter",	"issubclass",	"pow",	"super",
"bytes",	"float",	"iter",	"print",	"tuple",
"callable",	"format",	"len",	"property",	"type",
"chr",	"frozenset",	"list",	"range",	"vars",
"classmethod",	"getattr",	"locals",	"repr",	"zip",
"compile",	"globals",	"map",	"reversed",	"__import__",
"complex",	"hasattr",	"max",	"round",
"delattr",	"hash",	"memoryview",	"set",
]

exceptions = ['ArithmeticError',
 'AssertionError',
 'AttributeError',
 'BaseException',
 'BlockingIOError',
 'BrokenPipeError',
 'BufferError',
 'ChildProcessError',
 'ConnectionAbortedError',
 'ConnectionError',
 'ConnectionRefusedError',
 'ConnectionResetError',
 'EOFError',
 'EnvironmentError',
 'Exception',
 'FileExistsError',
 'FileNotFoundError',
 'FloatingPointError',
 'IOError',
 'ImportError',
 'IndentationError',
 'IndexError',
 'InterruptedError',
 'IsADirectoryError',
 'KeyError',
 'LookupError',
 'MemoryError',
 'ModuleNotFoundError',
 'NameError',
 'NotADirectoryError',
 'NotImplementedError',
 'OSError',
 'OverflowError',
 'PermissionError',
 'ProcessLookupError',
 'RecursionError',
 'ReferenceError',
 'RuntimeError',
 'SyntaxError',
 'SystemError',
 'TabError',
 'TimeoutError',
 'TypeError',
 'UnboundLocalError',
 'UnicodeDecodeError',
 'UnicodeEncodeError',
 'UnicodeError',
 'UnicodeTranslateError',
 'ValueError',
 'ZeroDivisionError']


warnings = ['BytesWarning',
 'DeprecationWarning',
 'FutureWarning',
 'ImportWarning',
 'PendingDeprecationWarning',
 'ResourceWarning',
 'RuntimeWarning',
 'SyntaxWarning',
 'UnicodeWarning',
 'UserWarning',
 'Warning']

python_grammar = [
    {
        "Node": "Module",
        "Parent": "Parser",
        "Children": [
            ("Module", "(stmt* body)"),
            ("Interactive", "(stmt* body)"),
            ("Expression", "(expr body)"),
            ("Suite", "(stmt* body)"),
        ]
    },
    {
        "Node": "Statement",
        "Parent": "Module",
        "Children": [
            ("FunctionDef", "(identifier name, arguments args, stmt* body, expr* decorator_list, expr? returns)"),
            ("AsyncFunctionDef", "(identifier name, arguments args, stmt* body, expr* decorator_list, expr? returns)"),
            ("ClassDef", "(identifier name, expr* bases,  keyword* keywords,  stmt* body, expr* decorator_list)"),
             ("Return", "(expr? value)"),
             ("Delete", "(expr* targets)"),
             ("Assign", "(expr* targets, expr value)"),
             ("AugAssign", "(expr target, operator op, expr value) "),
             ("AnnAssign", "(expr target, expr annotation, expr? value, int simple)"),
             ("For", "(expr target, expr iter, stmt* body, stmt* orelse)"),
             ("AsyncFor", "(expr target, expr iter, stmt* body, stmt* orelse)"),
             ("While", "(expr test, stmt* body, stmt* orelse)"),
             ("If", "(expr test, stmt* body, stmt* orelse)"),
             ("With", "(withitem* items, stmt* body)"),
             ("AsyncWith", "(withitem* items, stmt* body)"),

             ("Raise", "(expr? exc, expr? cause)"),
             ("Try", "(stmt* body, excepthandler* handlers, stmt* orelse, stmt* finalbody)"),
             ("Assert", "(expr test, expr? msg)"),
             ("Import", "(alias* names)"),
             ("ImportFrom", "(identifier? module, alias* names, int? level)"),
             ("Global", "(identifier* names)"),
             ("Nonlocal", "(identifier* names)"),
             ("Expr", "(expr value)"),
             ("Pass", ""),
             ("Break", ""),
             ("Continue", ""),
        ]
    },
    {
        "Node": "Expression",
        "Parent": "Statement",
        "Children": [
            ("BoolOp", "(boolop op, expr* values)"),
            ("BinOp", "(expr left, operator op, expr right)"),
            ("UnaryOp", "(unaryop op, expr operand)"),
            ("Lambda", "(arguments args, expr body)"),
            ("IfExp", "(expr test, expr body, expr orelse)"),
            ("Dict", "(expr* keys, expr* values)"),
            ("Set", "(expr* elts)"),
            ("ListComp", "(expr elt, comprehension* generators)"),
            ("SetComp", "(expr elt, comprehension* generators)"),
            ("DictComp", "(expr key, expr value, comprehension* generators)"),
            ("GeneratorExp", "(expr elt, comprehension* generators)"),
            ("Await", "(expr value)"),
            ("Yield", "(expr? value)"),
            ("YieldFrom", "(expr value)"),
            ("Compare", "(expr left, cmpop* ops, expr* comparators)"),
            ("Call", "(expr func, expr* args, keyword* keywords)"),
            ("Num", "(object n)"),
            ("Str", "(string s)"),
            ("FormattedValue", "(expr value, int? conversion, expr? format_spec)"),
            ("JoinedStr", "(expr* values)"),
            ("Bytes", "(bytes s)"),
            ("NameConstant", "(singleton value)"),
            ("Ellipsis", "..."),
            ("Constant", "(constant value)"),
            ("Attribute", "(expr value, identifier attr, expr_context ctx)"),
            ("Subscript", "(expr value, slice slice, expr_context ctx)"),
            ("Starred", "(expr value, expr_context ctx)"),
            ("Name", "(identifier id, expr_context ctx)"),
            ("List", "(expr* elts, expr_context ctx)"),
            ("Tuple", "(expr* elts, expr_context ctx)"),
            ("Slice", "(expr? lower, expr? upper, expr? step)"),
            ("ExtSlice", "(slice* dims)"),
            ("Index", "(expr value)"),
        ]
    }

]

parser_key = """

**Parser Key**

```
    expr_context = Load | Store | Del | AugLoad | AugStore | Param

    boolop = And | Or

    operator = Add | Sub | Mult | MatMult | Div | Mod | Pow | LShift
                 | RShift | BitOr | BitXor | BitAnd | FloorDiv

    unaryop = Invert | Not | UAdd | USub

    cmpop = Eq | NotEq | Lt | LtE | Gt | GtE | Is | IsNot | In | NotIn

    comprehension = (expr target, expr iter, expr* ifs, int is_async)

    excepthandler = ExceptHandler(expr? type, identifier? name, stmt* body)
                    attributes (int lineno, int col_offset)

    arguments = (arg* args, arg? vararg, arg* kwonlyargs, expr* kw_defaults,
                 arg? kwarg, expr* defaults)

    arg = (identifier arg, expr? annotation)
           attributes (int lineno, int col_offset)

    -- keyword arguments supplied to call (NULL identifier for **kwargs)
    keyword = (identifier? arg, expr value)

    -- import name with optional 'as' alias.
    alias = (identifier name, identifier? asname)

    withitem = (expr context_expr, expr? optional_vars)
```
"""

def make_tasks_for_grammar(client):
    title = '[Component] Parser Grammar'
    print(f'Create Task: {title}')

    result = client.maniphest.edit(transactions=[
        {'type': 'title', 'value': title},
        {'type': 'priority', 'value': 'normal'}
    ])
    phid = result['object']['phid']

    for b in builtins:
        title = f'[Builtin Function] {b}'
        print(f'Create Task: {title}')
        r = client.maniphest.edit(transactions=[
            {'type': 'title', 'value': title},
            {'type': 'parent', 'value': phid},
        ])
    print(r)

def make_tasks_for_builtins(client):
    title = '[Component] Builtin Function'
    print(f'Create Task: {title}')

    result = client.maniphest.edit(transactions=[
        {'type': 'title', 'value': title},
        {'type': 'priority', 'value': 'Normal'}
    ])
    phid = result['object']['phid']

    for b in builtins:
        title = f'[Builtin Function] {b}'
        print(f'Create Task: {title}')
        r = client.maniphest.edit(transactions=[
            {'type': 'title', 'value': title},
            {'type': 'parent', 'value': phid},
            {'type': 'priority', 'value': 'Normal'}
        ])
    print(r)

def make_tasks_for_object(client, obj):
    title = f'[Object] {obj}'
    result = client.maniphest.edit(transactions=[
            {'type': 'title', 'value': title},
    ])
    phid = result['object']['phid']


    for title in make_titles(f"[{obj}]"):
        print(f'Create Task: {title}')
        r = client.maniphest.edit(transactions=[
            {'type': 'title', 'value': title},
            {'type': 'parent', 'value': phid}
        ])
        print(r)


def make_tasks_for_parser_grammar(client):
    title = '[Component] Parser'
    print(f'Create Task: {title}')

    result = client.maniphest.edit(transactions=[
        {'type': 'title', 'value': title},
        {'type': 'priority', 'value': 'normal'}
    ])
    print(result)
    root_phid = result['object']['phid']
    known_phids = {
        'Parser': root_phid
    }

    for node in python_grammar:
        name = node['Node']
        parent_phid = known_phids.get(node['Parent'], None)
        title = f'[Parser] {name}'
        print(f'Create Task: {title}')
        r = client.maniphest.edit(transactions=[
            {'type': 'title', 'value': title},
            {'type': 'parent', 'value': root_phid},
            {'type': 'parent', 'value': parent_phid},
            {'type': 'priority', 'value': 'normal'},
        ])
        print(r)
        node_phid = r['object']['phid']
        known_phids[name] = node_phid
        for child_name, description in node['Children']:
            child_title = f"[{name}] {child_name}"
            print(f'Create Task: {child_title}')
            r = client.maniphest.edit(transactions=[
                {'type': 'title', 'value': child_title},
                {'type': 'parent', 'value': node_phid},
                {'type': 'parent', 'value': root_phid},
                {'type': 'description', 'value': ''.join([description, parser_key])},
                {'type': 'priority', 'value': 'normal'}
            ])
            print(r)

def make_tasks_for_warnings(client):
    title = '[Component] Warnings'
    print(f'Create Task: {title}')

    result = client.maniphest.edit(transactions=[
        {'type': 'title', 'value': title},
        {'type': 'priority', 'value': 'Normal'}
    ])
    phid = result['object']['phid']

    for b in warnings:
        title = f'[Warning] {b}'
        print(f'Create Task: {title}')
        r = client.maniphest.edit(transactions=[
            {'type': 'title', 'value': title},
            {'type': 'parent', 'value': phid},
        ])
    print(r)

def main():
    client = Phabricator()
    client.update_interfaces()

    # for o in objects:
    #     make_tasks_for_object(api, o)

    # make_tasks_fo`r_builtins(api)
    # make_tasks_for_exceptions(client)
    # make_tasks_for_warnings(client)
    make_tasks_for_parser_grammar(client)

if __name__ == '__main__':
    main()
