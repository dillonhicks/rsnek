use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use result::{NativeResult, RuntimeResult};
use runtime::{Runtime, IntegerProvider, BooleanProvider};

use object::{self, RtValue};
use object::selfref::{self, SelfRef};
use object::typing;
use object::method;

use typedef::native;
use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;


pub struct PyBytesType {
    pub empty: ObjectRef,
}


impl typing::BuiltinType for PyBytesType {
    type T = PyBytes;
    type V = native::Bytes;

    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        PyBytesType::inject_selfref(PyBytesType::alloc(value))
    }

    fn init_type() -> Self {
        PyBytesType { empty: PyBytesType::inject_selfref(PyBytesType::alloc(native::Bytes::new())) }
    }


    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::Bytes(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Bytes(ref string) => {
                string.rc.set(&objref.clone());
            }
            _ => unreachable!(),
        }
        new
    }


    fn alloc(value: Self::V) -> Self::T {
        PyBytes {
            value: StringValue(value),
            rc: selfref::RefCount::default(),
        }
    }
}


pub struct StringValue(pub native::Bytes);


pub type PyBytes = RtValue<StringValue>;


impl fmt::Debug for PyBytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "String {{ {:?} }}", self.value.0)
    }
}




impl object::PyAPI for PyBytes {}
impl method::New for PyBytes {}
impl method::Init for PyBytes {}
impl method::Delete for PyBytes {}
impl method::GetAttr for PyBytes {}
impl method::GetAttribute for PyBytes {}
impl method::SetAttr for PyBytes {}
impl method::DelAttr for PyBytes {}
impl method::Id for PyBytes {}
impl method::Is for PyBytes {}
impl method::IsNot for PyBytes {}
impl method::Hashed for PyBytes {
    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_hash() {
            Ok(value) => Ok(rt.int(native::Integer::from(value))),
            Err(err) => Err(err),
        }
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
        let mut s = DefaultHasher::new();
        self.value.0.hash(&mut s);
        Ok(s.finish())
    }
}
impl method::StringCast for PyBytes {}
impl method::BytesCast for PyBytes {}
impl method::StringFormat for PyBytes {}
impl method::StringRepresentation for PyBytes {}
impl method::Equal for PyBytes {
    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let boxed: &Box<Builtin> = rhs.0.borrow();

        match self.native_eq(boxed) {
            Ok(value) => Ok(rt.bool(value)),
            _ => unreachable!(),
        }
    }

    fn native_eq(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        match rhs {
            &Builtin::Bytes(ref bytes) => Ok(self.value.0 == bytes.value.0),
            _ => Ok(false),
        }
    }
}
impl method::NotEqual for PyBytes {}
impl method::LessThan for PyBytes {}
impl method::LessOrEqual for PyBytes {}
impl method::GreaterOrEqual for PyBytes {}
impl method::GreaterThan for PyBytes {}
impl method::BooleanCast for PyBytes {}
impl method::IntegerCast for PyBytes {}
impl method::FloatCast for PyBytes {}
impl method::ComplexCast for PyBytes {}
impl method::Rounding for PyBytes {}
impl method::Index for PyBytes {}
impl method::NegateValue for PyBytes {}
impl method::AbsValue for PyBytes {}
impl method::PositiveValue for PyBytes {}
impl method::InvertValue for PyBytes {}
impl method::Add for PyBytes {}
impl method::BitwiseAnd for PyBytes {}
impl method::DivMod for PyBytes {}
impl method::FloorDivision for PyBytes {}
impl method::LeftShift for PyBytes {}
impl method::Modulus for PyBytes {}
impl method::Multiply for PyBytes {}
impl method::MatrixMultiply for PyBytes {}
impl method::BitwiseOr for PyBytes {}
impl method::Pow for PyBytes {}
impl method::RightShift for PyBytes {}
impl method::Subtract for PyBytes {}
impl method::TrueDivision for PyBytes {}
impl method::XOr for PyBytes {}
impl method::ReflectedAdd for PyBytes {}
impl method::ReflectedBitwiseAnd for PyBytes {}
impl method::ReflectedDivMod for PyBytes {}
impl method::ReflectedFloorDivision for PyBytes {}
impl method::ReflectedLeftShift for PyBytes {}
impl method::ReflectedModulus for PyBytes {}
impl method::ReflectedMultiply for PyBytes {}
impl method::ReflectedMatrixMultiply for PyBytes {}
impl method::ReflectedBitwiseOr for PyBytes {}
impl method::ReflectedPow for PyBytes {}
impl method::ReflectedRightShift for PyBytes {}
impl method::ReflectedSubtract for PyBytes {}
impl method::ReflectedTrueDivision for PyBytes {}
impl method::ReflectedXOr for PyBytes {}
impl method::InPlaceAdd for PyBytes {}
impl method::InPlaceBitwiseAnd for PyBytes {}
impl method::InPlaceDivMod for PyBytes {}
impl method::InPlaceFloorDivision for PyBytes {}
impl method::InPlaceLeftShift for PyBytes {}
impl method::InPlaceModulus for PyBytes {}
impl method::InPlaceMultiply for PyBytes {}
impl method::InPlaceMatrixMultiply for PyBytes {}
impl method::InPlaceBitwiseOr for PyBytes {}
impl method::InPlacePow for PyBytes {}
impl method::InPlaceRightShift for PyBytes {}
impl method::InPlaceSubtract for PyBytes {}
impl method::InPlaceTrueDivision for PyBytes {}
impl method::InPlaceXOr for PyBytes {}
impl method::Contains for PyBytes {}
impl method::Iter for PyBytes {}
impl method::Call for PyBytes {}
impl method::Length for PyBytes {}
impl method::LengthHint for PyBytes {}
impl method::Next for PyBytes {}
impl method::Reversed for PyBytes {}
impl method::GetItem for PyBytes {}
impl method::SetItem for PyBytes {}
impl method::DeleteItem for PyBytes {}
impl method::Count for PyBytes {}
impl method::Append for PyBytes {}
impl method::Extend for PyBytes {}
impl method::Pop for PyBytes {}
impl method::Remove for PyBytes {}
impl method::IsDisjoint for PyBytes {}
impl method::AddItem for PyBytes {}
impl method::Discard for PyBytes {}
impl method::Clear for PyBytes {}
impl method::Get for PyBytes {}
impl method::Keys for PyBytes {}
impl method::Values for PyBytes {}
impl method::Items for PyBytes {}
impl method::PopItem for PyBytes {}
impl method::Update for PyBytes {}
impl method::SetDefault for PyBytes {}
impl method::Await for PyBytes {}
impl method::Send for PyBytes {}
impl method::Throw for PyBytes {}
impl method::Close for PyBytes {}
impl method::Exit for PyBytes {}
impl method::Enter for PyBytes {}
impl method::DescriptorGet for PyBytes {}
impl method::DescriptorSet for PyBytes {}
impl method::DescriptorSetName for PyBytes {}
