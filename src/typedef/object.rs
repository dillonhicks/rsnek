use std::fmt;
use std::ops::Deref;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use error::Error;
use result::{RuntimeResult, NativeResult};
use runtime::{Runtime, NoneProvider, IntegerProvider};
use object::{self, RtValue, typing};
use object::method::{self, Id, GetItem, Hashed, SetItem};
use object::selfref::{self, SelfRef};
use object::typing::BuiltinType;

use typedef::dictionary::PyDictType;
use typedef::tuple::PyTupleType;
use typedef::builtin::Builtin;
use typedef::native::{self, DictKey};
use typedef::objectref::ObjectRef;


pub struct PyObjectType {
    pub object: ObjectRef,
    pub pytype: ObjectRef,
}

impl PyObjectType {
    pub fn init_type(typeref: &ObjectRef) -> Self {
        let typ = PyObjectType::inject_selfref(PyObjectType::alloc(native::Object {
                                                                       class: typeref.clone(),
                                                                       dict: PyDictType::inject_selfref(PyDictType::alloc(native::Dict::new())),
                                                                       bases: PyTupleType::inject_selfref(PyTupleType::alloc(native::Tuple::new())),
                                                                   }));


        let object = PyObjectType::inject_selfref(PyObjectType::alloc(native::Object {
                                                                          class: typeref.clone(),
                                                                          dict: PyDictType::inject_selfref(PyDictType::alloc(native::Dict::new())),
                                                                          bases:
                                                                              PyTupleType::inject_selfref(PyTupleType::alloc(native::Tuple::new())),
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


//// +-+-+-+-+-+-+-+-+-+-+-+-+-+
////    Python Object Traits
//// +-+-+-+-+-+-+-+-+-+-+-+-+-+


impl object::PyAPI for PyObject {}
impl method::New for PyObject {}
impl method::Init for PyObject {}
impl method::Delete for PyObject {}

impl method::GetAttr for PyObject {
    // TODO: Need to search the base classes dicts as well, maybe need MRO
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
impl method::GetAttribute for PyObject {}

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
            Err(_) => Err(Error::attribute()),
        }
    }
}

impl method::DelAttr for PyObject {}
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
impl method::StringCast for PyObject {}
impl method::BytesCast for PyObject {}
impl method::StringFormat for PyObject {}
impl method::StringRepresentation for PyObject {}
impl method::Equal for PyObject {}
impl method::NotEqual for PyObject {}
impl method::LessThan for PyObject {}
impl method::LessOrEqual for PyObject {}
impl method::GreaterOrEqual for PyObject {}
impl method::GreaterThan for PyObject {}
impl method::BooleanCast for PyObject {}
impl method::IntegerCast for PyObject {}
impl method::FloatCast for PyObject {}
impl method::ComplexCast for PyObject {}
impl method::Rounding for PyObject {}
impl method::Index for PyObject {}
impl method::NegateValue for PyObject {}
impl method::AbsValue for PyObject {}
impl method::PositiveValue for PyObject {}
impl method::InvertValue for PyObject {}
impl method::Add for PyObject {}
impl method::BitwiseAnd for PyObject {}
impl method::DivMod for PyObject {}
impl method::FloorDivision for PyObject {}
impl method::LeftShift for PyObject {}
impl method::Modulus for PyObject {}
impl method::Multiply for PyObject {}
impl method::MatrixMultiply for PyObject {}
impl method::BitwiseOr for PyObject {}
impl method::Pow for PyObject {}
impl method::RightShift for PyObject {}
impl method::Subtract for PyObject {}
impl method::TrueDivision for PyObject {}
impl method::XOr for PyObject {}
impl method::ReflectedAdd for PyObject {}
impl method::ReflectedBitwiseAnd for PyObject {}
impl method::ReflectedDivMod for PyObject {}
impl method::ReflectedFloorDivision for PyObject {}
impl method::ReflectedLeftShift for PyObject {}
impl method::ReflectedModulus for PyObject {}
impl method::ReflectedMultiply for PyObject {}
impl method::ReflectedMatrixMultiply for PyObject {}
impl method::ReflectedBitwiseOr for PyObject {}
impl method::ReflectedPow for PyObject {}
impl method::ReflectedRightShift for PyObject {}
impl method::ReflectedSubtract for PyObject {}
impl method::ReflectedTrueDivision for PyObject {}
impl method::ReflectedXOr for PyObject {}
impl method::InPlaceAdd for PyObject {}
impl method::InPlaceBitwiseAnd for PyObject {}
impl method::InPlaceDivMod for PyObject {}
impl method::InPlaceFloorDivision for PyObject {}
impl method::InPlaceLeftShift for PyObject {}
impl method::InPlaceModulus for PyObject {}
impl method::InPlaceMultiply for PyObject {}
impl method::InPlaceMatrixMultiply for PyObject {}
impl method::InPlaceBitwiseOr for PyObject {}
impl method::InPlacePow for PyObject {}
impl method::InPlaceRightShift for PyObject {}
impl method::InPlaceSubtract for PyObject {}
impl method::InPlaceTrueDivision for PyObject {}
impl method::InPlaceXOr for PyObject {}
impl method::Contains for PyObject {}
impl method::Iter for PyObject {}
impl method::Call for PyObject {}
impl method::Length for PyObject {}
impl method::LengthHint for PyObject {}
impl method::Next for PyObject {}
impl method::Reversed for PyObject {}
impl method::GetItem for PyObject {}
impl method::SetItem for PyObject {}
impl method::DeleteItem for PyObject {}
impl method::Count for PyObject {}
impl method::Append for PyObject {}
impl method::Extend for PyObject {}
impl method::Pop for PyObject {}
impl method::Remove for PyObject {}
impl method::IsDisjoint for PyObject {}
impl method::AddItem for PyObject {}
impl method::Discard for PyObject {}
impl method::Clear for PyObject {}
impl method::Get for PyObject {}
impl method::Keys for PyObject {}
impl method::Values for PyObject {}
impl method::Items for PyObject {}
impl method::PopItem for PyObject {}
impl method::Update for PyObject {}
impl method::SetDefault for PyObject {}
impl method::Await for PyObject {}
impl method::Send for PyObject {}
impl method::Throw for PyObject {}
impl method::Close for PyObject {}
impl method::Exit for PyObject {}
impl method::Enter for PyObject {}
impl method::DescriptorGet for PyObject {}
impl method::DescriptorSet for PyObject {}
impl method::DescriptorSetName for PyObject {}


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//        stdlib Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+


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



// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//          Tests
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

#[cfg(test)]
mod _api_method {
    use runtime::{BooleanProvider, NoneProvider, StringProvider, IntegerProvider, ObjectProvider};
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

    mod __getattr__ {
        use super::*;
        #[test]
        fn set_and_get() {
            let rt = setup_test();
            let object = rt.object(native::None());

            let boxed: &Box<Builtin> = object.0.borrow();
            let key = rt.str("hello");
            let value = rt.int(234);

            let result = boxed.op_setattr(&rt, &key, &value).unwrap();
            assert_eq!(result, rt.none());

            let result = boxed.op_getattr(&rt, &key).unwrap();
            assert_eq!(result, value);
        }

        #[test]
        #[should_panic]
        fn get_nonexistant_key() {
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

    #[test]
    fn debug() {
        let rt = setup_test();
        let object = rt.object(native::None());
        println!("{:?}", object);
    }
}
