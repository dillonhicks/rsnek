use std::fmt;
use std::cell::Ref;
use std::ops::Deref;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use error::Error;
use result::{RuntimeResult, NativeResult};
use runtime::{Runtime, NoneProvider, IntegerProvider};
use object::{self, RtValue, typing};
use object::method::{self, Id};
use object::selfref::{self, SelfRef};
use object::typing::BuiltinType;

use typedef::dictionary::PyDictType;
use typedef::tuple::PyTupleType;
use typedef::builtin::Builtin;
use typedef::native::{self, Function, NativeFn, WrapperFn};
use typedef::object::PyObjectType;
use typedef::objectref::ObjectRef;


pub struct PyFunctionType {
    pub function_type: ObjectRef,
}

impl PyFunctionType {
    pub fn init_type(typeref: &ObjectRef, object: &ObjectRef) -> Self {

        let method = PyObjectType::inject_selfref(PyObjectType::alloc(
            native::Object {
                class: typeref.clone(),
                dict: PyDictType::inject_selfref(PyDictType::alloc(native::Dict::new())),
                bases: PyTupleType::inject_selfref(PyTupleType::alloc(vec![object.clone()])),
            }));

        PyFunctionType { function_type: method }
    }
}

impl typing::BuiltinType for PyFunctionType {
    type T = PyFunction;
    type V = native::Function;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        PyFunctionType::inject_selfref(PyFunctionType::alloc(value))
    }

    fn init_type() -> Self {
        unimplemented!()
    }

    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::Function(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Function(ref object) => {
                object.rc.set(&objref.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(object: Self::V) -> Self::T {
        PyFunction {
            value: FunctionValue(object),
            rc: selfref::RefCount::default(),
        }
    }
}


pub struct FunctionValue(pub native::Function);
pub type PyFunction = RtValue<FunctionValue>;

impl PyFunction {
    fn do_call_nativefn_rt(&self,
                           rt: &Runtime,
                           callable: &Box<NativeFn>,
                           pos_args: &ObjectRef,
                           star_args: &ObjectRef,
                           kwargs: &ObjectRef)
                           -> RuntimeResult {

        let boxed: &Box<Builtin> = pos_args.0.borrow();
        let arg0: native::Tuple = match boxed.deref() {
            &Builtin::Tuple(ref tuple) => {
                tuple.value
                    .0
                    .iter()
                    .map(|objref| objref.clone())
                    .collect()
            }
            _ => return Err(Error::typerr("Expected type tuple for pos_args")),
        };

        let boxed: &Box<Builtin> = star_args.0.borrow();
        let arg1: native::Tuple = match boxed.deref() {
            &Builtin::Tuple(ref tuple) => {
                tuple.value
                    .0
                    .iter()
                    .map(|objref| objref.clone())
                    .collect()
            }
            _ => return Err(Error::typerr("Expected type tuple for *args")),
        };

        let boxed: &Box<Builtin> = kwargs.0.borrow();
        let arg2: native::Dict = match boxed.deref() {
            &Builtin::Dict(ref dict) => {
                let borrowed: Ref<native::Dict> = dict.value.0.borrow();
                borrowed.iter().map(|(key, value)| (key.clone(), value.clone())).collect()
            }
            _ => return Err(Error::typerr("Expected type tuple for **args")),
        };

        match callable(&arg0, &arg1, &arg2) {
            Ok(_) => Ok(rt.none()),
            Err(err) => Err(err),
        }
    }

    fn do_call_wrapperfn(&self,
                         rt: &Runtime,
                         callable: &Box<WrapperFn>,
                         pos_args: &ObjectRef,
                         star_args: &ObjectRef,
                         kwargs: &ObjectRef)
                         -> RuntimeResult {

        let boxed: &Box<Builtin> = pos_args.0.borrow();
        match boxed.deref() {
            &Builtin::Tuple(_) => {}
            _ => return Err(Error::typerr("Expected type tuple for pos_args")),
        };

        let boxed: &Box<Builtin> = star_args.0.borrow();
        match boxed.deref() {
            &Builtin::Tuple(_) => {}
            _ => return Err(Error::typerr("Expected type tuple for *args")),
        };

        let boxed: &Box<Builtin> = kwargs.0.borrow();
        match boxed.deref() {
            &Builtin::Dict(_) => {}
            _ => return Err(Error::typerr("Expected type tuple for **args")),
        };

        callable(&rt, &pos_args, &star_args, &kwargs)
    }
}


//// +-+-+-+-+-+-+-+-+-+-+-+-+-+
////    Python Object Traits
//// +-+-+-+-+-+-+-+-+-+-+-+-+-+


impl object::PyAPI for PyFunction {}
impl method::New for PyFunction {}
impl method::Init for PyFunction {}
impl method::Delete for PyFunction {}

impl method::GetAttr for PyFunction {
    //
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
    //                    Err(err) => return Err(err)
    //                };
    //
    //                let key = DictKey(string.native_hash().unwrap(), stringref);
    //                let dict: &Box<Builtin> = self.value.0.dict.0.borrow();
    //                match dict.native_getitem(&Builtin::DictKey(key)) {
    //                    Ok(objref) => Ok(objref),
    //                    Err(err) => {
    //                        let boxed: &Box<Builtin> = self.value.0.bases.0.borrow();
    //
    //                        match boxed.deref() {
    //                            &Builtin::Tuple(ref tuple) => {
    //                                for base in &tuple.value.0 {
    //                                    println!("{:?}", base);
    //                                }
    //                            },
    //                            _ => unreachable!()
    //                        }
    //                        println!("NOOPE!");
    //                        Err(err)
    //                    }
    //                }
    //            },
    //            _ => Err(Error::typerr("getattr(): attribute name must be string"))
    //        }
    //    }
}
impl method::GetAttribute for PyFunction {}

impl method::SetAttr for PyFunction {
    //    fn op_setattr(&self, rt: &Runtime, name: &ObjectRef, value: &ObjectRef) -> RuntimeResult {
    //        let boxed_name: &Box<Builtin> = name.0.borrow();
    //        let boxed_value: &Box<Builtin> = value.0.borrow();
    //        match self.native_setattr(&boxed_name, boxed_value) {
    //            Ok(_) => Ok(rt.none()),
    //            Err(err) => Err(err)
    //        }
    //    }
    //
    //    fn native_setattr(&self, name: &Builtin, value: &Builtin) -> NativeResult<native::None> {
    //
    //        let hashid = match name.native_hash() {
    //            Ok(hash) => hash,
    //            Err(err) => return Err(err)
    //        };
    //
    //        let key_ref = match name.upgrade() {
    //            Ok(objref) => objref,
    //            Err(err) => return Err(err)
    //        };
    //
    //        let key = DictKey(hashid, key_ref);
    //        let dict: &Box<Builtin> = self.value.0.dict.0.borrow();
    //
    //        match dict.native_setitem(&Builtin::DictKey(key), &value) {
    //            Ok(_) => Ok(native::None()),
    //            Err(_) => Err(Error::attribute())
    //        }
    //    }
}

impl method::DelAttr for PyFunction {}

impl method::Id for PyFunction {
    // TODO: why do we have to go back through the builtin? Is there a good reason to
    //  special case this at the builtin.rs layer?
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

impl method::Hashed for PyFunction {
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
impl method::StringCast for PyFunction {}
impl method::BytesCast for PyFunction {}
impl method::StringFormat for PyFunction {}
impl method::StringRepresentation for PyFunction {}

impl method::Equal for PyFunction {
    fn native_eq(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        Ok(self.native_id() == other.native_id())
    }
}
impl method::NotEqual for PyFunction {}
impl method::LessThan for PyFunction {}
impl method::LessOrEqual for PyFunction {}
impl method::GreaterOrEqual for PyFunction {}
impl method::GreaterThan for PyFunction {}
impl method::BooleanCast for PyFunction {}
impl method::IntegerCast for PyFunction {}
impl method::FloatCast for PyFunction {}
impl method::ComplexCast for PyFunction {}
impl method::Rounding for PyFunction {}
impl method::Index for PyFunction {}
impl method::NegateValue for PyFunction {}
impl method::AbsValue for PyFunction {}
impl method::PositiveValue for PyFunction {}
impl method::InvertValue for PyFunction {}
impl method::Add for PyFunction {}
impl method::BitwiseAnd for PyFunction {}
impl method::DivMod for PyFunction {}
impl method::FloorDivision for PyFunction {}
impl method::LeftShift for PyFunction {}
impl method::Modulus for PyFunction {}
impl method::Multiply for PyFunction {}
impl method::MatrixMultiply for PyFunction {}
impl method::BitwiseOr for PyFunction {}
impl method::Pow for PyFunction {}
impl method::RightShift for PyFunction {}
impl method::Subtract for PyFunction {}
impl method::TrueDivision for PyFunction {}
impl method::XOr for PyFunction {}
impl method::ReflectedAdd for PyFunction {}
impl method::ReflectedBitwiseAnd for PyFunction {}
impl method::ReflectedDivMod for PyFunction {}
impl method::ReflectedFloorDivision for PyFunction {}
impl method::ReflectedLeftShift for PyFunction {}
impl method::ReflectedModulus for PyFunction {}
impl method::ReflectedMultiply for PyFunction {}
impl method::ReflectedMatrixMultiply for PyFunction {}
impl method::ReflectedBitwiseOr for PyFunction {}
impl method::ReflectedPow for PyFunction {}
impl method::ReflectedRightShift for PyFunction {}
impl method::ReflectedSubtract for PyFunction {}
impl method::ReflectedTrueDivision for PyFunction {}
impl method::ReflectedXOr for PyFunction {}
impl method::InPlaceAdd for PyFunction {}
impl method::InPlaceBitwiseAnd for PyFunction {}
impl method::InPlaceDivMod for PyFunction {}
impl method::InPlaceFloorDivision for PyFunction {}
impl method::InPlaceLeftShift for PyFunction {}
impl method::InPlaceModulus for PyFunction {}
impl method::InPlaceMultiply for PyFunction {}
impl method::InPlaceMatrixMultiply for PyFunction {}
impl method::InPlaceBitwiseOr for PyFunction {}
impl method::InPlacePow for PyFunction {}
impl method::InPlaceRightShift for PyFunction {}
impl method::InPlaceSubtract for PyFunction {}
impl method::InPlaceTrueDivision for PyFunction {}
impl method::InPlaceXOr for PyFunction {}
impl method::Contains for PyFunction {}
impl method::Iter for PyFunction {}
impl method::Call for PyFunction {
    fn op_call(&self, rt: &Runtime, pos_args: &ObjectRef, star_args: &ObjectRef, kwargs: &ObjectRef) -> RuntimeResult {
        match self.value.0 {
            Function::Native(ref func) => self.do_call_nativefn_rt(&rt, func, &pos_args, &star_args, &kwargs),
            Function::Wrapper(ref func) => self.do_call_wrapperfn(&rt, func, &pos_args, &star_args, &kwargs),
            _ => Err(Error::not_implemented()),
        }
    }

    #[allow(unused_variables)]
    fn native_call(&self, named_args: &Builtin, args: &Builtin, kwargs: &Builtin) -> NativeResult<Builtin> {
        Err(Error::not_implemented())
    }
}
impl method::Length for PyFunction {}
impl method::LengthHint for PyFunction {}
impl method::Next for PyFunction {}
impl method::Reversed for PyFunction {}
impl method::GetItem for PyFunction {}
impl method::SetItem for PyFunction {}
impl method::DeleteItem for PyFunction {}
impl method::Count for PyFunction {}
impl method::Append for PyFunction {}
impl method::Extend for PyFunction {}
impl method::Pop for PyFunction {}
impl method::Remove for PyFunction {}
impl method::IsDisjoint for PyFunction {}
impl method::AddItem for PyFunction {}
impl method::Discard for PyFunction {}
impl method::Clear for PyFunction {}
impl method::Get for PyFunction {}
impl method::Keys for PyFunction {}
impl method::Values for PyFunction {}
impl method::Items for PyFunction {}
impl method::PopItem for PyFunction {}
impl method::Update for PyFunction {}
impl method::SetDefault for PyFunction {}
impl method::Await for PyFunction {}
impl method::Send for PyFunction {}
impl method::Throw for PyFunction {}
impl method::Close for PyFunction {}
impl method::Exit for PyFunction {}
impl method::Enter for PyFunction {}
impl method::DescriptorGet for PyFunction {}
impl method::DescriptorSet for PyFunction {}
impl method::DescriptorSetName for PyFunction {}


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//        stdlib Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+


impl fmt::Display for PyFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Method")
    }
}

impl fmt::Debug for PyFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Method")
    }
}



// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//          Tests
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

#[cfg(test)]
mod _api_method {
    use runtime::{FunctionProvider, BooleanProvider, NoneProvider, DictProvider, TupleProvider};
    use object::method::*;
    use super::*;

    fn setup_test() -> (Runtime) {
        Runtime::new()
    }

    #[test]
    fn is_() {
        let rt = setup_test();
        let function = rt.function(native::None());
        let function2 = function.clone();
        let function3 = rt.function(native::None());

        let boxed: &Box<Builtin> = function.0.borrow();

        let result = boxed.op_is(&rt, &function2).unwrap();
        assert_eq!(result, rt.bool(true));

        let result = boxed.op_is(&rt, &function3).unwrap();
        assert_eq!(result, rt.bool(false));
    }


    #[test]
    fn is_not() {
        let rt = setup_test();
        let function = rt.function(native::None());
        let function2 = function.clone();
        let function3 = rt.function(native::None());

        let boxed: &Box<Builtin> = function.0.borrow();

        let result = boxed.op_is_not(&rt, &function2).unwrap();
        assert_eq!(result, rt.bool(false));

        let result = boxed.op_is_not(&rt, &function3).unwrap();
        assert_eq!(result, rt.bool(true));
    }


    #[test]
    fn __call__() {
        let rt = setup_test();
        let function = rt.function(native::None());

        let pos_args = rt.tuple(native::None());
        let starargs = rt.tuple(native::None());
        let kwargs = rt.dict(native::None());

        let boxed: &Box<Builtin> = function.0.borrow();

        let result = boxed.op_call(&rt, &pos_args, &starargs, &kwargs).unwrap();
        assert_eq!(result, rt.none());
    }


    #[test]
    fn debug() {
        let rt = setup_test();
        let object = rt.function(native::None());
        println!("{:?}", object);
    }
}
