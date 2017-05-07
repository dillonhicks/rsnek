use std::ops::Deref;
use std::borrow::Borrow;

use runtime::Runtime;
use api::typing;
use api::selfref::{self, SelfRef};
use api::typing::BuiltinType;

use objects::dictionary::PyDictType;
use objects::tuple::PyTupleType;
use ::modules::builtins::Type;
use ::system::primitives as rs;
use objects::object::{PyObject, ObjectValue};
use ::api::RtObject;


// TODO: {T49} pretty Obvious need to have classobjs since PyModule is just an object with
// a few required params
pub struct PyModuleType {
    pub object: RtObject,
    pub pytype: RtObject,
}


impl PyModuleType {
    pub fn init_type(typeref: &RtObject) -> Self {
        let typ = PyModuleType::inject_selfref(PyModuleType::alloc(rs::Object {
            class: typeref.clone(),
            dict: PyDictType::inject_selfref(PyDictType::alloc(rs::Dict::new())),
            bases: PyTupleType::inject_selfref(PyTupleType::alloc(rs::Tuple::new())),
        }));

        let object = PyModuleType::inject_selfref(PyModuleType::alloc(rs::Object {
            class: typeref.clone(),
            dict: PyDictType::inject_selfref(PyDictType::alloc(rs::Dict::new())),
            bases:
            PyTupleType::inject_selfref(PyTupleType::alloc(rs::Tuple::new())),
        }));

        PyModuleType {
            object: object,
            pytype: typ,
        }
    }
}

impl typing::BuiltinType for PyModuleType {
    type T = PyObject;
    type V = rs::Object;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> RtObject {
        PyModuleType::inject_selfref(PyModuleType::alloc(value))
    }

    fn init_type() -> Self {
        unimplemented!()
    }

    fn inject_selfref(value: Self::T) -> RtObject {
        let object = RtObject::new(Type::Module(value));
        let new = object.clone();

        match object.as_ref() {
            &Type::Module(ref module) => {
                module.rc.set(&object.clone());
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
