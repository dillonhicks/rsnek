use std::fmt;
use std::ops::Deref;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use ::api::result::Error;
use ::api::result::{ObjectResult, RtResult};
use runtime::Runtime;
use traits::{NoneProvider, IntegerProvider};
use api::{self, RtValue, typing};
use api::method::{self, Id, GetItem, Hashed, SetItem, Keys};
use api::selfref::{self, SelfRef};
use api::typing::BuiltinType;

use objects::dictionary::PyDictType;
use objects::tuple::PyTupleType;
use objects::builtin::Builtin;
use objects::native::{self, DictKey};
use ::api::RtObject;


pub struct PyObjectType {
    pub object: RtObject,
    pub pytype: RtObject,
}

impl PyObjectType {
    pub fn init_type(typeref: &RtObject) -> Self {

        // TODO: {T106} Fundamental objects should have __setitem__ set to a attribute error
        let typ = PyObjectType::inject_selfref(PyObjectType::alloc(native::Object {
            class: typeref.clone(),
            dict: PyDictType::inject_selfref(PyDictType::alloc(native::Dict::new())),
            bases: PyTupleType::inject_selfref(PyTupleType::alloc(native::Tuple::new())),
        }));

        let object = PyObjectType::inject_selfref(PyObjectType::alloc(native::Object {
            class: typeref.clone(),
            dict: PyDictType::inject_selfref(PyDictType::alloc(native::Dict::new())),
            bases: PyTupleType::inject_selfref(PyTupleType::alloc(native::Tuple::new())),
        }));

        PyObjectType {
            object: object,
            pytype: typ,
        }
    }
}

impl typing::BuiltinType for PyObjectType {
    type T = PyObject;
    type V = native::Object;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> RtObject {
        PyObjectType::inject_selfref(PyObjectType::alloc(value))
    }

    fn init_type() -> Self {
        unimplemented!()
    }

    fn inject_selfref(value: Self::T) -> RtObject {
        let object = RtObject::new(Builtin::Object(value));
        let new = object.clone();

        match object.as_ref() {
            &Builtin::Object(ref value) => {
                value.rc.set(&object.clone());
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


pub struct ObjectValue(pub native::Object);
pub type PyObject = RtValue<ObjectValue>;

impl PyObject {
    pub fn dir(&self) -> RtResult<native::Tuple> {
        self.value.0.dict.native_meth_keys()
    }

}

impl fmt::Display for PyObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value.0)
    }
}

impl fmt::Debug for PyObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value.0)
    }
}



impl api::PyAPI for PyObject {}

impl method::GetAttr for PyObject {
    // TODO: {T63} Need to search the base classes dicts as well, maybe need MRO
    #[allow(unused_variables)]
    fn op_getattr(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {
        self.native_getattr(name.as_ref())
    }

    fn native_getattr(&self, name: &Builtin) -> RtResult<RtObject> {
        match name {
            &Builtin::Str(ref string) => {
                let string_obj = string.rc.upgrade()?;

                let key = DictKey(string.native_hash()?, string_obj);
                let dict = &self.value.0.dict;

                match dict.native_getitem(&Builtin::DictKey(key)) {
                    Ok(objref) => Ok(objref),
                    Err(err) => {
                        let bases = &self.value.0.bases;

                        match bases.as_ref() {
                            &Builtin::Tuple(ref tuple) => {
                                for base in &tuple.value.0 {
                                    info!("{:?}", base);
                                }
                            }
                            _ => unreachable!(),
                        }
                        info!("NOPE!");
                        Err(err)
                    }
                }
            }
            _ => Err(Error::typerr("getattr(): attribute name must be string")),
        }
    }
}

impl method::SetAttr for PyObject {
    fn op_setattr(&self, rt: &Runtime, name: &RtObject, value: &RtObject) -> ObjectResult {
        self.native_setattr(name.as_ref(), value.as_ref())?;
        Ok(rt.none())
    }

    fn native_setattr(&self, name: &Builtin, value: &Builtin) -> RtResult<native::None> {

        let hashid = name.native_hash()?;
        let key_ref = name.upgrade()?;

        let key = DictKey(hashid, key_ref);
        let dict = &self.value.0.dict;

        match dict.native_setitem(&Builtin::DictKey(key), &value) {
            Ok(_) => Ok(native::None()),
            Err(_) => Err(Error::attribute("Could not set attribute")),
        }
    }
}

impl method::Id for PyObject {
    fn native_id(&self) -> native::ObjectId {
        match self.rc.upgrade() {
            Ok(this_object) => this_object.native_id(),
            Err(_) => 0,
        }
    }
}

impl method::Hashed for PyObject {
    fn op_hash(&self, rt: &Runtime) -> ObjectResult {
        let value = self.native_hash()?;
        Ok(rt.int(value))
    }

    fn native_hash(&self) -> RtResult<native::HashId> {
        let mut s = DefaultHasher::new();
        self.native_id().hash(&mut s);
        Ok(s.finish())
    }
}


method_not_implemented!(PyObject,
    AbsValue   Add   AddItem   Append  Await   BitwiseAnd   BitwiseOr   BooleanCast
    BytesCast   Call   Clear   Close  ComplexCast   Contains   Count   DelAttr
    Delete   DeleteItem   DescriptorGet   DescriptorSet DescriptorSetName   Discard   DivMod
    Enter Equal   Exit   Extend   FloatCast FloorDivision   Get  GetAttribute
    GetItem   GreaterOrEqual   GreaterThan   InPlaceAdd   InPlaceBitwiseAnd   InPlaceBitwiseOr
    InPlaceDivMod   InPlaceFloorDivision   InPlaceLeftShift   InPlaceMatrixMultiply
    InPlaceModulus   InPlaceMultiply   InPlacePow   InPlaceRightShift  InPlaceSubtract
    InPlaceTrueDivision   InPlaceXOr   Index   Init   IntegerCast   InvertValue   Is
    IsDisjoint   IsNot   Items   Iter   Keys   LeftShift   Length   LengthHint
    LessOrEqual   LessThan   MatrixMultiply   Modulus  Multiply   NegateValue   New   Next
    NotEqual   Pop   PopItem   PositiveValue  Pow   ReflectedAdd   ReflectedBitwiseAnd
    ReflectedBitwiseOr   ReflectedDivMod   ReflectedFloorDivision   ReflectedLeftShift
    ReflectedMatrixMultiply   ReflectedModulus   ReflectedMultiply   ReflectedPow
    ReflectedRightShift   ReflectedSubtract   ReflectedTrueDivision   ReflectedXOr   Remove
    Reversed   RightShift   Rounding   Send   SetDefault   SetItem   StringCast
    StringFormat   StringRepresentation   Subtract   Throw TrueDivision   Update   Values   XOr
);


#[cfg(test)]
mod tests {
    use traits::{BooleanProvider, TupleProvider, NoneProvider, DictProvider,
                 StringProvider, IntegerProvider, ObjectProvider};
    use api::method::*;
    use super::*;


    fn setup_test() -> (Runtime) {
        Runtime::new()
    }


    #[test]
    fn is_() {
        let rt = setup_test();
        let object = rt.object(native::None());
        let object2 = object.clone();
        let object3 = rt.object(native::None());

        let result = object.op_is(&rt, &object2).unwrap();
        assert_eq!(result, rt.bool(true));

        let result = object.op_is(&rt, &object3).unwrap();
        assert_eq!(result, rt.bool(false));
    }


    #[test]
    fn is_not() {
        let rt = setup_test();
        let object = rt.object(native::None());
        let object2 = object.clone();
        let object3 = rt.object(native::None());

        let result = object.op_is_not(&rt, &object2).unwrap();
        assert_eq!(result, rt.bool(false));

        let result = object.op_is_not(&rt, &object3).unwrap();
        assert_eq!(result, rt.bool(true));
    }

    #[test]
    fn __setattr__() {
        let rt = setup_test();
        let object = rt.object(native::None());

        let key = rt.str("hello");
        let value = rt.int(234);

        let result = object.op_setattr(&rt, &key, &value).unwrap();
        assert_eq!(result, rt.none())
    }

    #[cfg(test)]
    mod __getattr__ {
        use super::*;

        #[test]
        fn set_and_get() {
            let rt = setup_test();
            let object = rt.object(native::None());

            let key = rt.str("hello");
            let value = rt.int(234);

            let result = object.op_setattr(&rt, &key, &value).unwrap();
            assert_eq!(result, rt.none());

            let attr = object.op_getattr(&rt, &key).unwrap();
            assert_eq!(attr, value);
        }

        #[test]
        #[should_panic]
        fn get_missing_key() {
            let rt = setup_test();
            let object = rt.object(native::None());

            let key = rt.str("hello");
            let value = rt.int(234);

            let result = object.op_setattr(&rt, &key, &value).unwrap();
            assert_eq!(result, rt.none());

            let key = rt.str("baddie");
            object.op_getattr(&rt, &key).unwrap();
        }

    }


    /// Milestone v0.2.0
    ///
    /// Test setting the builtin len function as an attribute of the object.
    /// Retrieving that function by name, and calling it on tuple.
    #[test]
    fn function_setattr_getattr_call() {
        let rt = setup_test();
        let object = rt.object(native::None());

        let builtin_func = rt.get_builtin("len");
        let key = rt.str("test_function");
        let result = object.op_setattr(&rt, &key, &builtin_func).unwrap();
        assert_eq!(result, rt.none());

        let len = object.op_getattr(&rt, &key).unwrap();
        assert_eq!(len, builtin_func);

        let tuple = rt.tuple(vec![rt.none(), rt.int(3), rt.str("Potato!@!@")]);
        let args = rt.tuple(vec![tuple.clone()]);
        let starargs = rt.tuple(vec![]);
        let kwargs = rt.dict(native::Dict::new());

        let result = len.op_call(&rt, &args, &starargs, &kwargs).unwrap();
        assert_eq!(result, rt.int(3));
    }


}