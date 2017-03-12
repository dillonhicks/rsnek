#![feature(associated_consts)]

use opcode::*;
use std::result;

//
//type Result = result::Result<Object, InterpreterError>;
//
//#[derive(Debug,Clone,Copy)]
//pub enum FunctionType {
////    const Inquery: fn(Object) -> isize;
////    const Unary: fn(Object) -> Object;
////    const Binary: fn(Object, Object) -> Object;
////    const Ternary: fn(Object, Object, Object) -> Object;
////    Inquery(fn(Object) -> Result),
////    Unary(fn(Object) -> Result),
////    Binary(fn(Object, Object) -> Result),
////    Ternary(fn(Object, Object, Object) -> Result)
//}
//
//
//#[derive(Debug)]
//pub struct Instruction {
//    op: OpCode,
//    func: FunctionType
//}
//
//
//pub struct InterpreterError {
//    message: &'static str
//}
//
//
//fn add<T: Object>(lhs: T, rhs: T) -> T {
//    match lhs.object_type() {
//        TypeType::Number => {
//            match rhs.object_type() {
//                TypeType::Number => {
//
//                    BuiltinObject { object: true, value: rhs.value + lhs.value}
//                },
//                _ => panic!("Dunno what to do 1!")
//            }
//        }
//        _ => panic!("Dunno what to do 2!")
//    }
//}
//
//fn subtract<T: Object>(lhs: T, rhs: T) -> T {
//    lhs - rhs
//}
//
//fn multiply<T: Object>(lhs: T, rhs: T) -> T {
//    lhs * rhs
//}
//
//fn divide<T: Object>(lhs: T, rhs: T) -> T {
//    lhs / rhs
//}
//
//
////pub const PYFUNC_ADD : Instruction = Instruction { op: OpCode::BinaryAdd, func: FunctionType::Binary(add)};
//
//#[derive(Clone,Copy,Debug)]
//pub struct BuiltinObject<T> {
//    object: bool,
//    value: T
//}
//
//
//pub enum TypeType {
//    Number,
//    String,
//    Bytes,
//    Map,
//    List,
//    Tuple,
//    Object
//}
//
//
//pub trait Object {
//    fn object_type(&self) -> TypeType;
//}
//
//type IntegerObject = BuiltinObject<BigInt>;
//
//impl Object for IntegerObject {
//    fn object_type(&self) -> TypeType {
//        TypeType::Number
//    }
//}
//
//type ObjectMethods = bool;

//#[derive(Debug,Clone,Copy)]
//pub struct ObjectCLike {
//
//    //    binaryfunc nb_add;
//    //    binaryfunc nb_subtract;
//    //    binaryfunc nb_multiply;
//    //    binaryfunc nb_remainder;
//    //    binaryfunc nb_divmod;
//    //    ternaryfunc nb_power;
//    //    unaryfunc nb_negative;
//    //    unaryfunc nb_positive;
//    //    unaryfunc nb_absolute;
//    func_add: FunctionType,
//    func_subtract: FunctionType,
//    func_multiply: FunctionType,
//    func_remainder: FunctionType,
//    func_divmod: FunctionType,
//    func_power: FunctionType,
//    func_negative: FunctionType,
//    func_positive: FunctionType,
//    func_absolute: FunctionType,
//
//    //    inquiry nb_bool;
//    func_inquery: FunctionType,
//
//    //    unaryfunc nb_invert;
//    //    binaryfunc nb_lshift;
//    //    binaryfunc nb_rshift;
//    //    binaryfunc nb_and;
//    //    binaryfunc nb_xor;
//    //    binaryfunc nb_or;
//    //    unaryfunc nb_int;
//    //    void *nb_reserved;  /* the slot formerly known as nb_long */
//    //    unaryfunc nb_float;
//    func_invert: FunctionType,
//    func_lshift: FunctionType,
//    func_rshift: FunctionType,
//    func_and: FunctionType,
//    func_xor: FunctionType,
//    func_or: FunctionType,
//
//    //    binaryfunc nb_inplace_add;
//    //    binaryfunc nb_inplace_subtract;
//    //    binaryfunc nb_inplace_multiply;
//    //    binaryfunc nb_inplace_remainder;
//    //    ternaryfunc nb_inplace_power;
//    //    binaryfunc nb_inplace_lshift;
//    //    binaryfunc nb_inplace_rshift;
//    //    binaryfunc nb_inplace_and;
//    //    binaryfunc nb_inplace_xor;
//    //    binaryfunc nb_inplace_or;
//
//    func_inplace_add: FunctionType,
//    func_inplace_subtract: FunctionType,
//    func_inplace_multiply: FunctionType,
//    func_inplace_remainder: FunctionType,
//    func_inplace_power: FunctionType,
//    func_inplace_invert: FunctionType,
//    func_inplace_lshift: FunctionType,
//    func_inplace_rshift: FunctionType,
//    func_inplace_and: FunctionType,
//    func_inplace_xor: FunctionType,
//    func_inplace_or: FunctionType,
//
//
//    //    binaryfunc nb_floor_divide;
//    //    binaryfunc nb_true_divide;
//    //    binaryfunc nb_inplace_floor_divide;
//    //    binaryfunc nb_inplace_true_divide;
//
//    func_floor_divide: FunctionType,
//    func_true_divide: FunctionType,
//    func_inplace_floor_divide: FunctionType,
//    func_inplace_true_divide: FunctionType,
//
//    //    unaryfunc nb_index;
//    func_index: FunctionType,
//
//    //    binaryfunc nb_matrix_multiply;
//    //    binaryfunc nb_inplace_matrix_multiply;
//    func_matrix_multiply: FunctionType,
//    func_inplace_matrix_multiply: FunctionType
//}
//
//
//trait TypeInfo {
//
//}
//
//pub struct ObjectType<'tp>{
//    type_name: &'tp str,
//
//}
//
//impl<'tp> TypeInfo for ObjectType<'tp> {
//
//}


//
//typedef struct {
//    lenfunc sq_length;
//    binaryfunc sq_concat;
//    ssizeargfunc sq_repeat;
//    ssizeargfunc sq_item;
//    void *was_sq_slice;
//    ssizeobjargproc sq_ass_item;
//    void *was_sq_ass_slice;
//    objobjproc sq_contains;
//
//    binaryfunc sq_inplace_concat;
//    ssizeargfunc sq_inplace_repeat;
//} PySequenceMethods;
//
//typedef struct {
//    lenfunc mp_length;
//    binaryfunc mp_subscript;
//    objobjargproc mp_ass_subscript;
//} PyMappingMethods;
//
//typedef struct {
//    unaryfunc am_await;
//    unaryfunc am_aiter;
//    unaryfunc am_anext;
//} PyAsyncMethods;

//typedef struct {
//     getbufferproc bf_getbuffer;
//     releasebufferproc bf_releasebuffer;
//} PyBufferProcs;
//#endif /* Py_LIMITED_API */
//
//
//#ifdef Py_LIMITED_API
//typedef struct _typeobject PyTypeObject; /* opaque */
//#else
//typedef struct _typeobject {
//    PyObject_VAR_HEAD
//    const char *tp_name; /* For printing, in format "<module>.<name>" */
//    Py_ssize_t tp_basicsize, tp_itemsize; /* For allocation */
//
//    /* Methods to implement standard operations */
//
//    destructor tp_dealloc;
//    printfunc tp_print;
//    getattrfunc tp_getattr;
//    setattrfunc tp_setattr;
//    PyAsyncMethods *tp_as_async; /* formerly known as tp_compare (Python 2)
//                                    or tp_reserved (Python 3) */
//    reprfunc tp_repr;
//
//    /* Method suites for standard classes */
//
//    PyNumberMethods *tp_as_number;
//    PySequenceMethods *tp_as_sequence;
//    PyMappingMethods *tp_as_mapping;
//
//    /* More standard operations (here for binary compatibility) */
//
//    hashfunc tp_hash;
//    ternaryfunc tp_call;
//    reprfunc tp_str;
//    getattrofunc tp_getattro;
//    setattrofunc tp_setattro;
//
//    /* Functions to access object as input/output buffer */
//    PyBufferProcs *tp_as_buffer;
//
//    /* Flags to define presence of optional/expanded features */
//    unsigned long tp_flags;
//
//    const char *tp_doc; /* Documentation string */
//
//    /* Assigned meaning in release 2.0 */
//    /* call function for all accessible objects */
//    traverseproc tp_traverse;
//
//    /* delete references to contained objects */
//    inquiry tp_clear;
//
//    /* Assigned meaning in release 2.1 */
//    /* rich comparisons */
//    richcmpfunc tp_richcompare;
//
//    /* weak reference enabler */
//    Py_ssize_t tp_weaklistoffset;
//
//    /* Iterators */
//    getiterfunc tp_iter;
//    iternextfunc tp_iternext;
//
//    /* Attribute descriptor and subclassing stuff */
//    struct PyMethodDef *tp_methods;
//    struct PyMemberDef *tp_members;
//    struct PyGetSetDef *tp_getset;
//    struct _typeobject *tp_base;
//    PyObject *tp_dict;
//    descrgetfunc tp_descr_get;
//    descrsetfunc tp_descr_set;
//    Py_ssize_t tp_dictoffset;
//    initproc tp_init;
//    allocfunc tp_alloc;
//    newfunc tp_new;
//    freefunc tp_free; /* Low-level free-memory routine */
//    inquiry tp_is_gc; /* For PyObject_IS_GC */
//    PyObject *tp_bases;
//    PyObject *tp_mro; /* method resolution order */
//    PyObject *tp_cache;
//    PyObject *tp_subclasses;
//    PyObject *tp_weaklist;
//    destructor tp_del;
//
//    /* Type attribute cache version tag. Added in version 2.6 */
//    unsigned int tp_version_tag;
//
//    destructor tp_finalize;
//
//#ifdef COUNT_ALLOCS
//    /* these must be last and never explicitly initialized */
//    Py_ssize_t tp_allocs;
//    Py_ssize_t tp_frees;
//    Py_ssize_t tp_maxalloc;
//    struct _typeobject *tp_prev;
//    struct _typeobject *tp_next;
//#endif
//} PyTypeObject;
//#endif