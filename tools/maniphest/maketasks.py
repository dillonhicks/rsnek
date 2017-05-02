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




def main():
    api = Phabricator()
    api.update_interfaces()
    float_phid = 'PHID-TASK-76klrffvs73cwy7xesmy'
    for title in make_titles("[Float]")[1:]:
        print(f'Create Task: {title}')
        r = api.maniphest.edit(transactions=[
            {'type': 'title', 'value': title},
            {'type': 'parent', 'value': float_phid}
        ])
        print(r)

    #print(api.maniphest('search').json())


if __name__ == '__main__':
    main()
