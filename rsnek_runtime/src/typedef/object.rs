use std::fmt;
use std::ops::Deref;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use error::Error;
use result::{RuntimeResult, NativeResult};
use runtime::Runtime;
use traits::{NoneProvider, IntegerProvider};
use object::{self, RtValue, typing};
use object::method::{self, Id, GetItem, Hashed, SetItem, Keys};
use object::selfref::{self, SelfRef};
use object::typing::BuiltinType;

use typedef::dictionary::PyDictType;
use typedef::tuple::PyTupleType;
use typedef::builtin::Builtin;
use typedef::native::{self, DictKey};
use ::object::RtObject as ObjectRef;


pub struct PyObjectType {
    pub object: ObjectRef,
    pub pytype: ObjectRef,
}

impl PyObjectType {
    pub fn init_type(typeref: &ObjectRef) -> Self {

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
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        PyObjectType::inject_selfref(PyObjectType::alloc(value))
    }

    fn init_type() -> Self {
        unimplemented!()
    }

    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::Object(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Object(ref object) => {
                object.rc.set(&objref.clone());
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
    pub fn dir(&self) -> NativeResult<native::Tuple> {
        let boxed: &Box<Builtin> = self.value.0.dict.0.borrow();
        boxed.native_meth_keys()
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



impl object::PyAPI for PyObject {}

impl method::GetAttr for PyObject {
    // TODO: {T63} Need to search the base classes dicts as well, maybe need MRO
    #[allow(unused_variables)]
    fn op_getattr(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        let boxed: &Box<Builtin> = name.0.borrow();
        self.native_getattr(&boxed)
    }

    fn native_getattr(&self, name: &Builtin) -> NativeResult<ObjectRef> {
        match name {
            &Builtin::Str(ref string) => {
                let stringref = match string.rc.upgrade() {
                    Ok(objref) => objref,
                    Err(err) => return Err(err),
                };

                let key = DictKey(string.native_hash().unwrap(), stringref);
                let dict: &Box<Builtin> = self.value
                    .0
                    .dict
                    .0
                    .borrow();
                match dict.native_getitem(&Builtin::DictKey(key)) {
                    Ok(objref) => Ok(objref),
                    Err(err) => {
                        let boxed: &Box<Builtin> = self.value
                            .0
                            .bases
                            .0
                            .borrow();

                        match boxed.deref() {
                            &Builtin::Tuple(ref tuple) => {
                                for base in &tuple.value.0 {
                                    println!("{:?}", base);
                                }
                            }
                            _ => unreachable!(),
                        }
                        println!("NOOPE!");
                        Err(err)
                    }
                }
            }
            _ => Err(Error::typerr("getattr(): attribute name must be string")),
        }
    }
}

impl method::SetAttr for PyObject {
    fn op_setattr(&self, rt: &Runtime, name: &ObjectRef, value: &ObjectRef) -> RuntimeResult {
        let boxed_name: &Box<Builtin> = name.0.borrow();
        let boxed_value: &Box<Builtin> = value.0.borrow();
        match self.native_setattr(&boxed_name, boxed_value) {
            Ok(_) => Ok(rt.none()),
            Err(err) => Err(err),
        }
    }

    fn native_setattr(&self, name: &Builtin, value: &Builtin) -> NativeResult<native::None> {

        let hashid = match name.native_hash() {
            Ok(hash) => hash,
            Err(err) => return Err(err),
        };

        let key_ref = match name.upgrade() {
            Ok(objref) => objref,
            Err(err) => return Err(err),
        };

        let key = DictKey(hashid, key_ref);
        let dict: &Box<Builtin> = self.value
            .0
            .dict
            .0
            .borrow();

        match dict.native_setitem(&Builtin::DictKey(key), &value) {
            Ok(_) => Ok(native::None()),
            Err(_) => Err(Error::attribute("Could not set attribute")),
        }
    }
}

impl method::Id for PyObject {
    fn native_id(&self) -> native::ObjectId {
        match self.rc.upgrade() {
            Ok(objref) => {
                let boxed: &Box<Builtin> = objref.0.borrow();
                boxed.native_id()
            }
            Err(_) => 0,
        }
    }
}

impl method::Hashed for PyObject {
    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_hash() {
            Ok(hashid) => Ok(rt.int(hashid)),
            Err(err) => Err(err),
        }
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
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
    use object::method::*;
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

        let boxed: &Box<Builtin> = object.0.borrow();

        let result = boxed.op_is(&rt, &object2).unwrap();
        assert_eq!(result, rt.bool(true));

        let result = boxed.op_is(&rt, &object3).unwrap();
        assert_eq!(result, rt.bool(false));
    }


    #[test]
    fn is_not() {
        let rt = setup_test();
        let object = rt.object(native::None());
        let object2 = object.clone();
        let object3 = rt.object(native::None());

        let boxed: &Box<Builtin> = object.0.borrow();

        let result = boxed.op_is_not(&rt, &object2).unwrap();
        assert_eq!(result, rt.bool(false));

        let result = boxed.op_is_not(&rt, &object3).unwrap();
        assert_eq!(result, rt.bool(true));
    }

    #[test]
    fn __setattr__() {
        let rt = setup_test();
        let object = rt.object(native::None());

        let boxed: &Box<Builtin> = object.0.borrow();
        let key = rt.str("hello");
        let value = rt.int(234);

        let result = boxed.op_setattr(&rt, &key, &value).unwrap();
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

            let boxed: &Box<Builtin> = object.0.borrow();
            let key = rt.str("hello");
            let value = rt.int(234);

            let result = boxed.op_setattr(&rt, &key, &value).unwrap();
            assert_eq!(result, rt.none());

            let key = rt.str("baddie");
            boxed.op_getattr(&rt, &key).unwrap();
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