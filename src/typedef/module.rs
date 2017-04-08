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


//pub struct ModuleValue(pub ObjectRef);
//pub type PyModule = RtValue<ModuleValue>;
//
//
////// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//////    Python Object Traits
////// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//
//
//impl object::PyAPI for PyModule {}
//impl method::New for PyModule {}
//impl method::Init for PyModule {}
//impl method::Delete for PyModule {}
//
//impl method::GetAttr for PyModule {
//    // TODO: Need to search the base classes dicts as well, maybe need MRO
//    #[allow(unused_variables)]
//    fn op_getattr(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
//        let boxed: &Box<Builtin> = name.0.borrow();
//        self.native_getattr(&boxed)
//    }
//
//    fn native_getattr(&self, name: &Builtin) -> NativeResult<ObjectRef> {
//        match name {
//            &Builtin::Str(ref string) => {
//                let stringref = match string.rc.upgrade() {
//                    Ok(objref) => objref,
//                    Err(err) => return Err(err),
//                };
//
//                let key = DictKey(string.native_hash().unwrap(), stringref);
//                let dict: &Box<Builtin> = self.value
//                    .0
//                    .dict
//                    .0
//                    .borrow();
//                match dict.native_getitem(&Builtin::DictKey(key)) {
//                    Ok(objref) => Ok(objref),
//                    Err(err) => {
//                        let boxed: &Box<Builtin> = self.value
//                            .0
//                            .bases
//                            .0
//                            .borrow();
//
//                        match boxed.deref() {
//                            &Builtin::Tuple(ref tuple) => {
//                                for base in &tuple.value.0 {
//                                    println!("{:?}", base);
//                                }
//                            }
//                            _ => unreachable!(),
//                        }
//                        println!("NOOPE!");
//                        Err(err)
//                    }
//                }
//            }
//            _ => Err(Error::typerr("getattr(): attribute name must be string")),
//        }
//    }
//}
//impl method::GetAttribute for PyModule {}
//
//impl method::SetAttr for PyModule {
//    fn op_setattr(&self, rt: &Runtime, name: &ObjectRef, value: &ObjectRef) -> RuntimeResult {
//        let boxed_name: &Box<Builtin> = name.0.borrow();
//        let boxed_value: &Box<Builtin> = value.0.borrow();
//        match self.native_setattr(&boxed_name, boxed_value) {
//            Ok(_) => Ok(rt.none()),
//            Err(err) => Err(err),
//        }
//    }
//
//    fn native_setattr(&self, name: &Builtin, value: &Builtin) -> NativeResult<native::None> {
//
//        let hashid = match name.native_hash() {
//            Ok(hash) => hash,
//            Err(err) => return Err(err),
//        };
//
//        let key_ref = match name.upgrade() {
//            Ok(objref) => objref,
//            Err(err) => return Err(err),
//        };
//
//        let key = DictKey(hashid, key_ref);
//        let dict: &Box<Builtin> = self.value
//            .0
//            .dict
//            .0
//            .borrow();
//
//        match dict.native_setitem(&Builtin::DictKey(key), &value) {
//            Ok(_) => Ok(native::None()),
//            Err(_) => Err(Error::attribute()),
//        }
//    }
//}
//
//impl method::DelAttr for PyModule {}
//impl method::Id for PyModule {
//    fn native_id(&self) -> native::ObjectId {
//        match self.rc.upgrade() {
//            Ok(objref) => {
//                let boxed: &Box<Builtin> = objref.0.borrow();
//                boxed.native_id()
//            }
//            Err(_) => 0,
//        }
//    }
//}
//
//impl method::Hashed for PyModule {
//    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
//        match self.native_hash() {
//            Ok(hashid) => Ok(rt.int(hashid)),
//            Err(err) => Err(err),
//        }
//    }
//
//    fn native_hash(&self) -> NativeResult<native::HashId> {
//        let mut s = DefaultHasher::new();
//        self.native_id().hash(&mut s);
//        Ok(s.finish())
//    }
//}
//impl method::StringCast for PyModule {}
//impl method::BytesCast for PyModule {}
//impl method::StringFormat for PyModule {}
//impl method::StringRepresentation for PyModule {}
//impl method::Equal for PyModule {}
//impl method::NotEqual for PyModule {}
//impl method::LessThan for PyModule {}
//impl method::LessOrEqual for PyModule {}
//impl method::GreaterOrEqual for PyModule {}
//impl method::GreaterThan for PyModule {}
//impl method::BooleanCast for PyModule {}
//impl method::IntegerCast for PyModule {}
//impl method::FloatCast for PyModule {}
//impl method::ComplexCast for PyModule {}
//impl method::Rounding for PyModule {}
//impl method::Index for PyModule {}
//impl method::NegateValue for PyModule {}
//impl method::AbsValue for PyModule {}
//impl method::PositiveValue for PyModule {}
//impl method::InvertValue for PyModule {}
//impl method::Add for PyModule {}
//impl method::BitwiseAnd for PyModule {}
//impl method::DivMod for PyModule {}
//impl method::FloorDivision for PyModule {}
//impl method::LeftShift for PyModule {}
//impl method::Modulus for PyModule {}
//impl method::Multiply for PyModule {}
//impl method::MatrixMultiply for PyModule {}
//impl method::BitwiseOr for PyModule {}
//impl method::Pow for PyModule {}
//impl method::RightShift for PyModule {}
//impl method::Subtract for PyModule {}
//impl method::TrueDivision for PyModule {}
//impl method::XOr for PyModule {}
//impl method::ReflectedAdd for PyModule {}
//impl method::ReflectedBitwiseAnd for PyModule {}
//impl method::ReflectedDivMod for PyModule {}
//impl method::ReflectedFloorDivision for PyModule {}
//impl method::ReflectedLeftShift for PyModule {}
//impl method::ReflectedModulus for PyModule {}
//impl method::ReflectedMultiply for PyModule {}
//impl method::ReflectedMatrixMultiply for PyModule {}
//impl method::ReflectedBitwiseOr for PyModule {}
//impl method::ReflectedPow for PyModule {}
//impl method::ReflectedRightShift for PyModule {}
//impl method::ReflectedSubtract for PyModule {}
//impl method::ReflectedTrueDivision for PyModule {}
//impl method::ReflectedXOr for PyModule {}
//impl method::InPlaceAdd for PyModule {}
//impl method::InPlaceBitwiseAnd for PyModule {}
//impl method::InPlaceDivMod for PyModule {}
//impl method::InPlaceFloorDivision for PyModule {}
//impl method::InPlaceLeftShift for PyModule {}
//impl method::InPlaceModulus for PyModule {}
//impl method::InPlaceMultiply for PyModule {}
//impl method::InPlaceMatrixMultiply for PyModule {}
//impl method::InPlaceBitwiseOr for PyModule {}
//impl method::InPlacePow for PyModule {}
//impl method::InPlaceRightShift for PyModule {}
//impl method::InPlaceSubtract for PyModule {}
//impl method::InPlaceTrueDivision for PyModule {}
//impl method::InPlaceXOr for PyModule {}
//impl method::Contains for PyModule {}
//impl method::Iter for PyModule {}
//impl method::Call for PyModule {}
//impl method::Length for PyModule {}
//impl method::LengthHint for PyModule {}
//impl method::Next for PyModule {}
//impl method::Reversed for PyModule {}
//impl method::GetItem for PyModule {}
//impl method::SetItem for PyModule {}
//impl method::DeleteItem for PyModule {}
//impl method::Count for PyModule {}
//impl method::Append for PyModule {}
//impl method::Extend for PyModule {}
//impl method::Pop for PyModule {}
//impl method::Remove for PyModule {}
//impl method::IsDisjoint for PyModule {}
//impl method::AddItem for PyModule {}
//impl method::Discard for PyModule {}
//impl method::Clear for PyModule {}
//impl method::Get for PyModule {}
//impl method::Keys for PyModule {}
//impl method::Values for PyModule {}
//impl method::Items for PyModule {}
//impl method::PopItem for PyModule {}
//impl method::Update for PyModule {}
//impl method::SetDefault for PyModule {}
//impl method::Await for PyModule {}
//impl method::Send for PyModule {}
//impl method::Throw for PyModule {}
//impl method::Close for PyModule {}
//impl method::Exit for PyModule {}
//impl method::Enter for PyModule {}
//impl method::DescriptorGet for PyModule {}
//impl method::DescriptorSet for PyModule {}
//impl method::DescriptorSetName for PyModule {}
//
//
//// +-+-+-+-+-+-+-+-+-+-+-+-+-+
////        stdlib Traits
//// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//
//
//impl fmt::Display for PyModule {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{:?}", self.value.0)
//    }
//}
//
//impl fmt::Debug for PyModule {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{:?}", self.value.0)
//    }
//}
//
//
//
//// +-+-+-+-+-+-+-+-+-+-+-+-+-+
////          Tests
//// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//
//#[cfg(test)]
//mod _api_method {
//    use runtime::{BooleanProvider, NoneProvider, StringProvider, IntegerProvider, ModuleProvider};
//    use object::method::*;
//    use super::*;
//
//    fn setup_test() -> (Runtime) {
//        Runtime::new()
//    }
//
//    #[test]
//    fn is_() {
//        let rt = setup_test();
//        let module = rt.module(native::None());
//        let module2 = module.clone();
//        let module3 = rt.module(native::None());
//
//        let boxed: &Box<Builtin> = module.0.borrow();
//
//        let result = boxed.op_is(&rt, &module2).unwrap();
//        assert_eq!(result, rt.bool(true));
//
//        let result = boxed.op_is(&rt, &module3).unwrap();
//        assert_eq!(result, rt.bool(false));
//    }
//
//
//    #[test]
//    fn is_not() {
//        let rt = setup_test();
//        let module = rt.module(native::None());
//        let module2 = module.clone();
//        let module3 = rt.module(native::None());
//
//        let boxed: &Box<Builtin> = module.0.borrow();
//
//        let result = boxed.op_is_not(&rt, &module2).unwrap();
//        assert_eq!(result, rt.bool(false));
//
//        let result = boxed.op_is_not(&rt, &module3).unwrap();
//        assert_eq!(result, rt.bool(true));
//    }
//
//    #[test]
//    fn __setattr__() {
//        let rt = setup_test();
//        let module = rt.module(native::None());
//
//        let boxed: &Box<Builtin> = module.0.borrow();
//        let key = rt.str("hello");
//        let value = rt.int(234);
//
//        let result = boxed.op_setattr(&rt, &key, &value).unwrap();
//        assert_eq!(result, rt.none())
//    }
//
//    mod __getattr__ {
//        use super::*;
//        #[test]
//        fn set_and_get() {
//            let rt = setup_test();
//            let module = rt.module(native::None());
//
//            let boxed: &Box<Builtin> = module.0.borrow();
//            let key = rt.str("hello");
//            let value = rt.int(234);
//
//            let result = boxed.op_setattr(&rt, &key, &value).unwrap();
//            assert_eq!(result, rt.none());
//
//            let result = boxed.op_getattr(&rt, &key).unwrap();
//            assert_eq!(result, value);
//        }
//
//        #[test]
//        #[should_panic]
//        fn get_nonexistant_key() {
//            let rt = setup_test();
//            let module = rt.module(native::None());
//
//            let boxed: &Box<Builtin> = module.0.borrow();
//            let key = rt.str("hello");
//            let value = rt.int(234);
//
//            let result = boxed.op_setattr(&rt, &key, &value).unwrap();
//            assert_eq!(result, rt.none());
//
//            let key = rt.str("baddie");
//            boxed.op_getattr(&rt, &key).unwrap();
//        }
//
//    }
//
//    #[test]
//    fn debug() {
//        let rt = setup_test();
//        let module = rt.module(native::None());
//        println!("{:?}", module);
//    }
//}
//
//
//// +-+-+-+-+-+-+-+-+-+-+-+-+-+
////          Tests
//// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//
//#[cfg(test)]
//mod _integration {
//    use runtime::{DictProvider, TupleProvider, NoneProvider, StringProvider, IntegerProvider, ModuleProvider};
//    use object::method::*;
//    use super::*;
//
//    fn setup_test() -> (Runtime) {
//        Runtime::new()
//    }
//
//    /// Milestone v0.2.0
//    ///
//    /// Test setting the builtin len function as an attribute of the module.
//    /// Retrieving that function by name, and calling it on tuple.
//    #[test]
//    fn function_setattr_getattr_call() {
//        let rt = setup_test();
//        let module = rt.module(native::None());
//
//        let boxed: &Box<Builtin> = module.0.borrow();
//        let key = rt.str("test_function");
//
//        let func = rt.get_builtin("len");
//        let result = boxed.op_setattr(&rt, &key, &func).unwrap();
//        assert_eq!(result, rt.none());
//
//        let result = boxed.op_getattr(&rt, &key).unwrap();
//        assert_eq!(result, func);
//
//        let tuple = rt.tuple(vec![rt.none(), rt.int(3), rt.str("Potato!@!@")]);
//        let args = rt.tuple(vec![tuple.clone()]);
//        let starargs = rt.tuple(vec![]);
//        let kwargs = rt.dict(native::Dict::new());
//
//        let len: &Box<Builtin> = result.0.borrow();
//        let result = len.op_call(&rt, &args, &starargs, &kwargs).unwrap();
//        assert_eq!(result, rt.int(3));
//    }
//
//
//}