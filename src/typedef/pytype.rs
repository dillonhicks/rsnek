use std::fmt;
use std::cell::RefCell;
use std::borrow::Borrow;
use std::ops::Deref;

use runtime::Runtime;
use object::{self, RtValue, method, typing};
use object::selfref::{self, SelfRef};

use typedef::native;
use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;


pub struct PyMeta {
    pub pytype: ObjectRef
}

pub type PyType = RtValue<TypeValue>;
pub struct TypeValue(pub native::Type);


impl typing::BuiltinType for PyMeta {
    type T = PyType;
    type V = native::Type;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        PyMeta::inject_selfref(PyMeta::alloc(value))
    }

    fn init_type() -> Self {
        let typeref = PyMeta::inject_selfref(PyMeta::alloc(native::Type {
                name: "type".to_string(),
                module: "builtins".to_string(),
                bases: Vec::new(),
                subclasses: RefCell::new(Vec::new()),
            }));

        PyMeta {
            pytype: typeref
        }
    }

    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::Type(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Type(ref pytype) => {
                pytype.rc.set(&objref.clone());
            },
            _ => unreachable!()
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        PyType {
            value: TypeValue(value),
            rc: selfref::RefCount::default(),
        }
    }
}



// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    Python Object Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl object::PyAPI for PyType {}
impl method::New for PyType {}
impl method::Init for PyType {}
impl method::Delete for PyType {}
impl method::GetAttr for PyType {}
impl method::GetAttribute for PyType {}
impl method::SetAttr for PyType {}
impl method::DelAttr for PyType {}
impl method::Id for PyType {}
impl method::Is for PyType {}
impl method::IsNot for PyType {}
impl method::Hashed for PyType {}
impl method::StringCast for PyType {}
impl method::BytesCast for PyType {}
impl method::StringFormat for PyType {}
impl method::StringRepresentation for PyType {}
impl method::Equal for PyType {}
impl method::NotEqual for PyType {}
impl method::LessThan for PyType {}
impl method::LessOrEqual for PyType {}
impl method::GreaterOrEqual for PyType {}
impl method::GreaterThan for PyType {}
impl method::BooleanCast for PyType {}
impl method::IntegerCast for PyType {}
impl method::FloatCast for PyType {}
impl method::ComplexCast for PyType {}
impl method::Rounding for PyType {}
impl method::Index for PyType {}
impl method::NegateValue for PyType {}
impl method::AbsValue for PyType {}
impl method::PositiveValue for PyType {}
impl method::InvertValue for PyType {}
impl method::Add for PyType {}
impl method::BitwiseAnd for PyType {}
impl method::DivMod for PyType {}
impl method::FloorDivision for PyType {}
impl method::LeftShift for PyType {}
impl method::Modulus for PyType {}
impl method::Multiply for PyType {}
impl method::MatrixMultiply for PyType {}
impl method::BitwiseOr for PyType {}
impl method::Pow for PyType {}
impl method::RightShift for PyType {}
impl method::Subtract for PyType {}
impl method::TrueDivision for PyType {}
impl method::XOr for PyType {}
impl method::ReflectedAdd for PyType {}
impl method::ReflectedBitwiseAnd for PyType {}
impl method::ReflectedDivMod for PyType {}
impl method::ReflectedFloorDivision for PyType {}
impl method::ReflectedLeftShift for PyType {}
impl method::ReflectedModulus for PyType {}
impl method::ReflectedMultiply for PyType {}
impl method::ReflectedMatrixMultiply for PyType {}
impl method::ReflectedBitwiseOr for PyType {}
impl method::ReflectedPow for PyType {}
impl method::ReflectedRightShift for PyType {}
impl method::ReflectedSubtract for PyType {}
impl method::ReflectedTrueDivision for PyType {}
impl method::ReflectedXOr for PyType {}
impl method::InPlaceAdd for PyType {}
impl method::InPlaceBitwiseAnd for PyType {}
impl method::InPlaceDivMod for PyType {}
impl method::InPlaceFloorDivision for PyType {}
impl method::InPlaceLeftShift for PyType {}
impl method::InPlaceModulus for PyType {}
impl method::InPlaceMultiply for PyType {}
impl method::InPlaceMatrixMultiply for PyType {}
impl method::InPlaceBitwiseOr for PyType {}
impl method::InPlacePow for PyType {}
impl method::InPlaceRightShift for PyType {}
impl method::InPlaceSubtract for PyType {}
impl method::InPlaceTrueDivision for PyType {}
impl method::InPlaceXOr for PyType {}
impl method::Contains for PyType {}
impl method::Iter for PyType {}
impl method::Call for PyType {}
impl method::Length for PyType {}
impl method::LengthHint for PyType {}
impl method::Next for PyType {}
impl method::Reversed for PyType {}
impl method::GetItem for PyType {}
impl method::SetItem for PyType {}
impl method::DeleteItem for PyType {}
impl method::Count for PyType {}
impl method::Append for PyType {}
impl method::Extend for PyType {}
impl method::Pop for PyType {}
impl method::Remove for PyType {}
impl method::IsDisjoint for PyType {}
impl method::AddItem for PyType {}
impl method::Discard for PyType {}
impl method::Clear for PyType {}
impl method::Get for PyType {}
impl method::Keys for PyType {}
impl method::Values for PyType {}
impl method::Items for PyType {}
impl method::PopItem for PyType {}
impl method::Update for PyType {}
impl method::SetDefault for PyType {}
impl method::Await for PyType {}
impl method::Send for PyType {}
impl method::Throw for PyType {}
impl method::Close for PyType {}
impl method::Exit for PyType {}
impl method::Enter for PyType {}
impl method::DescriptorGet for PyType {}
impl method::DescriptorSet for PyType {}
impl method::DescriptorSetName for PyType {}


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//      stdlib traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl fmt::Display for PyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type({:?})", self.value.0)
    }
}

impl fmt::Debug for PyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type({:?})", self.value.0)
    }
}

#[cfg(test)]
mod _api_methods {
    use super::*;
    use runtime::{PyTypeProvider};

    fn setup_test() -> (Runtime) {
        Runtime::new()
    }

    #[test]
    fn _fake() {
        let rt = setup_test();
        let t = rt.pytype(native::None());
        println!("{:?}", t);
    }

}