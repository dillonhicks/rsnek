use std::fmt::Debug;
use std::any::Any;
use std::cell::RefCell;
use std::ops::Deref;
use std::borrow::Borrow;

use result::{RuntimeResult, NativeResult};
use runtime::Runtime;
use error::Error;

use typedef::objectref::ObjectRef;
use typedef::objectref::RtObject;
use typedef::integer::IntegerObject;
use typedef::builtin::Builtin;
use typedef::boolean::{SINGLETON_FALSE_BUILTIN, SINGLETON_TRUE_BUILTIN};
use typedef::native;


/// # Identity and Equality Traits

/// Get the address of some reference as u64
macro_rules! ident_from_ptr {
    ($name:ident) => {(&$name as *const _) as native::ObjectId}
}

/// Identity and Equals  __eq__ and is
pub trait Identifiable: Debug {
    fn identity(&self, runtime: &mut Runtime) -> RuntimeResult {
        let objref = IntegerObject::new_u64(self.native_identity()).as_builtin().as_object_ref();
        return runtime.alloc(objref);
    }

    fn native_identity(&self) -> native::ObjectId {
        return (&self as *const _) as native::ObjectId;
    }

    fn op_is(&self, rt: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let rhs_builtin: &Box<Builtin> = rhs.0.borrow();

        if self.native_is(rhs_builtin).unwrap() {
            Ok(rt.True())
        } else {
            Ok(rt.False())
        }
    }

    fn op_is_not(&self, rt: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let rhs_builtin: &Box<Builtin> = rhs.0.borrow();

        if self.native_is_not(rhs_builtin).unwrap() {
            Ok(rt.True())
        } else {
            Ok(rt.False())
        }
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn op_equals(&self, rt: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let rhs_builtin: &Box<Builtin> = rhs.0.borrow();

        if self.native_equals(rhs_builtin).unwrap() {
            Ok(rt.True())
        } else {
            Ok(rt.False())
        }
    }

    /// Default implementation of equals fallsbacks to op_is_not.
    fn op_not_equals(&self, rt: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let rhs_builtin: &Box<Builtin> = rhs.0.borrow();

        if self.native_not_equals(rhs_builtin).unwrap() {
            Ok(rt.True())
        } else {
            Ok(rt.False())
        }
    }

    fn native_is(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        Ok(self.native_identity() == other.native_identity())
    }

    fn native_is_not(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        Ok(!self.native_is(other).unwrap())
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn native_equals(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        return self.native_is(other);
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn native_not_equals(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        return Ok(!self.native_equals(other).unwrap());
    }
}


/// Hashable	 	__hash__
///
pub trait Hashable
    where Self: Identifiable
{
    fn op_hash(&self, &mut Runtime) -> RuntimeResult {
        Err(Error::not_implemented())
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
        Err(Error::not_implemented())
    }
}

/// Callable	 	__call__
pub trait Callable {
    /// In the case of call the &ObjectRef should be to a type
    /// that represents arguments
    fn op_call(&self, &mut Runtime, &ObjectRef) -> RuntimeResult {
        Err(Error::not_implemented())
    }
}




/// # Collection Traits

/// Container	 	__contains__
pub trait Container {
    fn op_contains(&self, &mut Runtime, &ObjectRef) -> RuntimeResult {
        Err(Error::not_implemented())
    }
}

/// Iterable	 	__iter__
pub trait Iterable {
    fn op_iter(&self, &mut Runtime) -> RuntimeResult {
        Err(Error::not_implemented())
    }
}

/// Iterator Iterable	__next__	__iter__
pub trait Iterator: Iterable {
    fn _next(&self, &mut Runtime) -> RuntimeResult {
        Err(Error::not_implemented())
    }
}

/// Reversible	Iterable	__reversed__
pub trait Reversible: Iterator {
    fn op_reversed(&self, &mut Runtime) -> RuntimeResult {
        Err(Error::not_implemented())
    }
}

/// Generator	Iterator	send, throw	close, __iter__, __next__
pub trait Generator: Iterator {
    fn send(&self, &mut Runtime) -> RuntimeResult {
        Err(Error::not_implemented())
    }

    fn throw(&self, &mut Runtime) -> RuntimeResult {
        Err(Error::not_implemented())
    }

    fn close(&self, &mut Runtime) -> RuntimeResult {
        Err(Error::not_implemented())
    }
}

/// Sized	 	__len__
pub trait Sized {
    fn op_len(&self, &mut Runtime) -> RuntimeResult {
        Err(Error::not_implemented())
    }
}

/// Collection	Sized, Iterable, Container	__contains__, __iter__, __len__
pub trait Collection: Sized + Iterable + Container {}

/// Sequence	Reversible, Collection	__getitem__, __len__	__contains__, __iter__, __reversed__, index, and count
pub trait Sequence: Reversible + Collection {
    fn op_getitem(&self, &mut Runtime, &ObjectRef) -> RuntimeResult {
        Err(Error::not_implemented())
    }

    fn index(&self, &mut Runtime, &ObjectRef) -> RuntimeResult {
        Err(Error::not_implemented())
    }

    fn count(&self, &mut Runtime, &ObjectRef) -> RuntimeResult {
        Err(Error::not_implemented())
    }
}

/// MutableSequence	Sequence	__getitem__, __setitem__, __delitem__, __len__, insert	Inherited Sequence methods and append, reverse, extend, pop, remove, and __iadd__
pub trait MutableSequence: Sequence {
    fn op_setitem(&self, &mut Runtime, &ObjectRef, &ObjectRef) -> RuntimeResult {
        Err(Error::not_implemented())
    }

    fn op_delitem(&self, &mut Runtime, &ObjectRef) -> RuntimeResult {
        Err(Error::not_implemented())
    }

    fn op_iadd(&self, &mut Runtime, &ObjectRef) -> RuntimeResult {
        Err(Error::not_implemented())
    }

    fn insert(&self, &mut Runtime, &ObjectRef, &ObjectRef) -> RuntimeResult {
        Err(Error::not_implemented())
    }

    fn append(&self, &mut Runtime, &ObjectRef) -> RuntimeResult {
        Err(Error::not_implemented())
    }

    fn extend(&self, &mut Runtime, &ObjectRef) -> RuntimeResult {
        Err(Error::not_implemented())
    }

    fn pop(&self, &mut Runtime, &ObjectRef) -> RuntimeResult {
        Err(Error::not_implemented())
    }

    fn remove(&self, &mut Runtime, &ObjectRef) -> RuntimeResult {
        Err(Error::not_implemented())
    }
}

// Set	Collection	__contains__, __iter__, __len__	__le__, __lt__, __eq__, __ne__, __gt__, __ge__, __and__, __or__, __sub__, __xor__, and isdisjoint
//pub trait Set: Collection {
//    fn op__le__, __lt__, __eq__, __ne__, __gt__, __ge__, __and__, __or__, __sub__, __xor__, and isdisjoint
//}
// MutableSet	Set	__contains__, __iter__, __len__, add, discard	Inherited Set methods and clear, pop, remove, __ior__, __iand__, __ixor__, and __isub__

// ByteString	Sequence	__getitem__, __len__	Inherited Sequence methods
// Mapping	Collection	__getitem__, __iter__, __len__	__contains__, keys, items, values, get, __eq__, and __ne__
// MutableMapping	Mapping	__getitem__, __setitem__, __delitem__, __iter__, __len__	Inherited Mapping methods and pop, popitem, clear, update, and setdefault
// MappingView	Sized	 	__len__
// ItemsView	MappingView, Set	 	__contains__, __iter__
// KeysView	MappingView, Set	 	__contains__, __iter__
// ValuesView	MappingView	 	__contains__, __iter__
// Awaitable	 	__await__
// Coroutine	Awaitable	send, throw	close
// AsyncIterable	 	__aiter__
// AsyncIterator	AsyncIterable	__anext__	__aiter__
// AsyncGenerator	AsyncIterator	asend, athrow	aclose, __aiter__, __anext__

trait ArithmeticMethods {}


trait Map {}

// typedef struct {
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
// PySequenceMethods;
//
// typedef struct {
//    lenfunc mp_length;
//    binaryfunc mp_subscript;
//    objobjargproc mp_ass_subscript;
// PyMappingMethods;
//
// typedef struct {
//    unaryfunc am_await;
//    unaryfunc am_aiter;
//    unaryfunc am_anext;
// PyAsyncMethods;
//
// typedef struct {
//    getbufferproc bf_getbuffer;
//    releasebufferproc bf_releasebuffer;
// PyBufferProcs;
// endif /* Py_LIMITED_API */
//
// typedef void (*freefunc)(void *);
// typedef void (*destructor)(PyObject *);
// ifndef Py_LIMITED_API
// We can't provide a full compile-time check that limited-API
//   users won't implement tp_print. However, not defining printfunc
//   and making tp_print of a different function pointer type
//   should at least cause a warning in most cases. */
// typedef int (*printfunc)(PyObject *, FILE *, int);
// endif
// typedef PyObject *(*getattrfunc)(PyObject *, char *);
// typedef PyObject *(*getattrofunc)(PyObject *, PyObject *);
// typedef int (*setattrfunc)(PyObject *, char *, PyObject *);
// typedef int (*setattrofunc)(PyObject *, PyObject *, PyObject *);
// typedef PyObject *(*reprfunc)(PyObject *);
// typedef Py_hash_t (*hashfunc)(PyObject *);
// typedef PyObject *(*richcmpfunc) (PyObject *, PyObject *, int);
// typedef PyObject *(*getiterfunc) (PyObject *);
// typedef PyObject *(*iternextfunc) (PyObject *);
// typedef PyObject *(*descrgetfunc) (PyObject *, PyObject *, PyObject *);
// typedef int (*descrsetfunc) (PyObject *, PyObject *, PyObject *);
// typedef int (*initproc)(PyObject *, PyObject *, PyObject *);
// typedef PyObject *(*newfunc)(struct _typeobject *, PyObject *, PyObject *);
// typedef PyObject *(*allocfunc)(struct _typeobject *, Py_ssize_t);
//
// ifdef Py_LIMITED_API
// typedef struct _typeobject PyTypeObject; /* opaque */
// else
// typedef struct _typeobject {
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
//    #ifdef COUNT_ALLOCS
//    /* these must be last and never explicitly initialized */
//    Py_ssize_t tp_allocs;
//    Py_ssize_t tp_frees;
//    Py_ssize_t tp_maxalloc;
//    struct _typeobject *tp_prev;
//    struct _typeobject *tp_next;
//    #endif
// PyTypeObject;
// endif
