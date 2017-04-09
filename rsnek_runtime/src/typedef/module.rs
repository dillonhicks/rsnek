use std::ops::Deref;
use std::borrow::Borrow;

use runtime::Runtime;
use object::typing;
use object::selfref::{self, SelfRef};
use object::typing::BuiltinType;

use typedef::dictionary::PyDictType;
use typedef::tuple::PyTupleType;
use typedef::builtin::Builtin;
use typedef::native;
use typedef::object::{PyObject, ObjectValue};
use typedef::objectref::ObjectRef;

// TODO: pretty Obvious need to have classobjs since PyModule is just an object with
// a few required params
pub struct PyModuleType {
    pub object: ObjectRef,
    pub pytype: ObjectRef,
}


impl PyModuleType {
    pub fn init_type(typeref: &ObjectRef) -> Self {
        let typ = PyModuleType::inject_selfref(PyModuleType::alloc(native::Object {
            class: typeref.clone(),
            dict: PyDictType::inject_selfref(PyDictType::alloc(native::Dict::new())),
            bases: PyTupleType::inject_selfref(PyTupleType::alloc(native::Tuple::new())),
        }));


        let object = PyModuleType::inject_selfref(PyModuleType::alloc(native::Object {
            class: typeref.clone(),
            dict: PyDictType::inject_selfref(PyDictType::alloc(native::Dict::new())),
            bases:
            PyTupleType::inject_selfref(PyTupleType::alloc(native::Tuple::new())),
        }));

        PyModuleType {
            object: object,
            pytype: typ,
        }
    }
}

impl typing::BuiltinType for PyModuleType {
    type T = PyObject;
    type V = native::Object;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        PyModuleType::inject_selfref(PyModuleType::alloc(value))
    }

    fn init_type() -> Self {
        unimplemented!()
    }

    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::Module(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Module(ref module) => {
                module.rc.set(&objref.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(object: Self::V) -> Self::T {
        PyObject {
            value: ObjectValue(object),
            rc: selfref::RefCount::default(),
        }
    }
}
