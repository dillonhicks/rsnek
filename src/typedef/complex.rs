use std;
use std::fmt;
use std::ops::Deref;
use std::borrow::Borrow;
use result::RuntimeResult;
use runtime::Runtime;
use object::{self, RtValue, PyAPI, method, typing};
use object::selfref::{self, SelfRef};

use typedef::{native, objectref};
use typedef::builtin::Builtin;
use typedef::objectref::ObjectRef;


#[derive(Clone)]
pub struct PyComplexType {}


impl typing::BuiltinType for PyComplexType {
    type T = PyComplex;
    type V = native::Complex;

    fn init_type() -> Self {
        PyComplexType {}
    }

    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::Complex(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Complex(ref complex) => {
                complex.rc.set(&objref.clone());
            },
            _ => unreachable!()
        }
        new
    }

    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
       PyComplexType::inject_selfref(PyComplexType::alloc(value))
    }

    fn alloc(value: Self::V) -> Self::T {
        PyComplex {
            value: ComplexValue(value),
            rc: selfref::RefCount::default(),
        }
    }

}



#[derive(Clone)]
pub struct ComplexValue(native::Complex);
pub type PyComplex = RtValue<ComplexValue>;


impl PyAPI for PyComplex {}
impl method::New for PyComplex {}
impl method::Init for PyComplex {}
impl method::Delete for PyComplex {}
impl method::GetAttr for PyComplex {}
impl method::GetAttribute for PyComplex {}
impl method::SetAttr for PyComplex {}
impl method::DelAttr for PyComplex {}
impl method::Id for PyComplex {}
impl method::Is for PyComplex {}
impl method::IsNot for PyComplex {}
impl method::Hashed for PyComplex {}
impl method::StringCast for PyComplex {}
impl method::BytesCast for PyComplex {}
impl method::StringFormat for PyComplex {}
impl method::StringRepresentation for PyComplex {}
impl method::Equal for PyComplex {}
impl method::NotEqual for PyComplex {}
impl method::LessThan for PyComplex {}
impl method::LessOrEqual for PyComplex {}
impl method::GreaterOrEqual for PyComplex {}
impl method::GreaterThan for PyComplex {}
impl method::BooleanCast for PyComplex {}
impl method::IntegerCast for PyComplex {}
impl method::FloatCast for PyComplex {}
impl method::ComplexCast for PyComplex {}
impl method::Rounding for PyComplex {}
impl method::Index for PyComplex {}
impl method::NegateValue for PyComplex {}
impl method::AbsValue for PyComplex {}
impl method::PositiveValue for PyComplex {}
impl method::InvertValue for PyComplex {}
impl method::Add for PyComplex {}
impl method::BitwiseAnd for PyComplex {}
impl method::DivMod for PyComplex {}
impl method::FloorDivision for PyComplex {}
impl method::LeftShift for PyComplex {}
impl method::Modulus for PyComplex {}
impl method::Multiply for PyComplex {}
impl method::MatrixMultiply for PyComplex {}
impl method::BitwiseOr for PyComplex {}
impl method::Pow for PyComplex {}
impl method::RightShift for PyComplex {}
impl method::Subtract for PyComplex {}
impl method::TrueDivision for PyComplex {}
impl method::XOr for PyComplex {}
impl method::ReflectedAdd for PyComplex {}
impl method::ReflectedBitwiseAnd for PyComplex {}
impl method::ReflectedDivMod for PyComplex {}
impl method::ReflectedFloorDivision for PyComplex {}
impl method::ReflectedLeftShift for PyComplex {}
impl method::ReflectedModulus for PyComplex {}
impl method::ReflectedMultiply for PyComplex {}
impl method::ReflectedMatrixMultiply for PyComplex {}
impl method::ReflectedBitwiseOr for PyComplex {}
impl method::ReflectedPow for PyComplex {}
impl method::ReflectedRightShift for PyComplex {}
impl method::ReflectedSubtract for PyComplex {}
impl method::ReflectedTrueDivision for PyComplex {}
impl method::ReflectedXOr for PyComplex {}
impl method::InPlaceAdd for PyComplex {}
impl method::InPlaceBitwiseAnd for PyComplex {}
impl method::InPlaceDivMod for PyComplex {}
impl method::InPlaceFloorDivision for PyComplex {}
impl method::InPlaceLeftShift for PyComplex {}
impl method::InPlaceModulus for PyComplex {}
impl method::InPlaceMultiply for PyComplex {}
impl method::InPlaceMatrixMultiply for PyComplex {}
impl method::InPlaceBitwiseOr for PyComplex {}
impl method::InPlacePow for PyComplex {}
impl method::InPlaceRightShift for PyComplex {}
impl method::InPlaceSubtract for PyComplex {}
impl method::InPlaceTrueDivision for PyComplex {}
impl method::InPlaceXOr for PyComplex {}
impl method::Contains for PyComplex {}
impl method::Iter for PyComplex {}
impl method::Call for PyComplex {}
impl method::Length for PyComplex {}
impl method::LengthHint for PyComplex {}
impl method::Next for PyComplex {}
impl method::Reversed for PyComplex {}
impl method::GetItem for PyComplex {}
impl method::SetItem for PyComplex {}
impl method::DeleteItem for PyComplex {}
impl method::Count for PyComplex {}
impl method::Append for PyComplex {}
impl method::Extend for PyComplex {}
impl method::Pop for PyComplex {}
impl method::Remove for PyComplex {}
impl method::IsDisjoint for PyComplex {}
impl method::AddItem for PyComplex {}
impl method::Discard for PyComplex {}
impl method::Clear for PyComplex {}
impl method::Get for PyComplex {}
impl method::Keys for PyComplex {}
impl method::Values for PyComplex {}
impl method::Items for PyComplex {}
impl method::PopItem for PyComplex {}
impl method::Update for PyComplex {}
impl method::SetDefault for PyComplex {}
impl method::Await for PyComplex {}
impl method::Send for PyComplex {}
impl method::Throw for PyComplex {}
impl method::Close for PyComplex {}
impl method::Exit for PyComplex {}
impl method::Enter for PyComplex {}
impl method::DescriptorGet for PyComplex {}
impl method::DescriptorSet for PyComplex {}
impl method::DescriptorSetName for PyComplex {}


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//        stdlib Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+


impl fmt::Display for PyComplex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.0)
    }
}

impl fmt::Debug for PyComplex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value.0)
    }
}


#[cfg(test)]
mod old {
    pub type Complex = native::Complex;


    #[derive(Clone, Debug)]
    pub struct ComplexObject {
        value: Complex,
    }


    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    //        stdlib Traits
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+

    impl ComplexObject {
        pub fn from_f64(real: f64, img: f64) -> Self {
            ComplexObject { value: Complex::new(real, img) }
        }

        pub fn from_native(native: Complex) -> Self {
            ComplexObject { value: native }
        }
    }

    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    //        stdlib Traits
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+

    impl objectref::RtObject for ComplexObject {}

    impl object::model::PyObject for ComplexObject {}

    impl object::model::PyBehavior for ComplexObject {}

    impl objectref::ToRtWrapperType<Builtin> for ComplexObject {
        fn to(self) -> Builtin {
            Builtin::Complex(self)
        }
    }

    impl objectref::ToRtWrapperType<ObjectRef> for ComplexObject {
        fn to(self) -> ObjectRef {
            ObjectRef::new(Builtin::Complex(self))
        }
    }


    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    //        stdlib Traits
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    impl fmt::Display for ComplexObject {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }
}
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//        Tests
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

#[cfg(test)]
#[allow(non_snake_case)]
mod impl_pybehavior {
    use std;
    use std::rc::Rc;
    use std::ops::Deref;
    use std::cmp::PartialEq;
    use std::borrow::Borrow;

    use num::ToPrimitive;

    use object::model::PyBehavior;
    use runtime::{Runtime, DEFAULT_HEAP_CAPACITY};
    use typedef::integer::IntegerObject;
    use typedef::builtin::Builtin;
    use typedef::float::FloatObject;
    use typedef::string::StringObject;
    use typedef::tuple::TupleObject;
    use typedef::list::ListObject;
    use typedef::objectref::ToRtWrapperType;
    use typedef::objectref::ObjectRef;

    use super::*;



    /// Reference equality
    ///  True is True
    #[test]
    fn is() {
        let mut rt = Runtime::new(None);
        assert_eq!(rt.heap_size(), 0);

        let False = rt.OldFalse();
        let False2 = False.clone();

        let False_ref: &Box<Builtin> = False.0.borrow();

        let result = False_ref.native_is(False_ref.deref()).unwrap();
        assert_eq!(result, true, "BooleanObject native is(native_is)");

        let result = False_ref.op_is(&mut rt, &False2).unwrap();
        assert_eq!(result, rt.OldTrue(), "BooleanObject is(op_is)");

    }

    ///
    /// True == True
    #[test]
    fn __eq__() {

        let mut rt = Runtime::new(None);

        let lhs = rt.alloc(ComplexObject::from_f64(1.0, 2.0).to()).unwrap();
        let rhs_eq = rt.alloc(ComplexObject::from_f64(1.0, 2.0).to()).unwrap();
        let rhs_ne = rt.alloc(ComplexObject::from_f64(0.0, 0.0).to()).unwrap();

        let boxed: &Box<Builtin> = lhs.0.borrow();

        let result = boxed.op_eq(&rt, &rhs_eq).unwrap();
        assert_eq!(result, rt.OldTrue());

        let result = boxed.op_ne(&rt, &rhs_eq).unwrap();
        assert_eq!(result, rt.OldFalse());

        let result = boxed.op_eq(&rt, &rhs_ne).unwrap();
        assert_eq!(result, rt.OldFalse());

        let result = boxed.op_ne(&rt, &rhs_ne).unwrap();
        assert_eq!(result, rt.OldTrue());
    }

    #[test]
    #[allow(non_snake_case)]
    fn __bool__() {
        let mut rt = Runtime::new(None);

        let lhs = rt.alloc(ComplexObject::from_f64(1.0, 2.0).to()).unwrap();

        let boxed: &Box<Builtin> = lhs.0.borrow();
        let result = boxed.op_bool(&rt).unwrap();

        assert_eq!(result, rt.OldTrue());
    }

    #[test]
    #[should_panic]
    fn __int__() {
        let mut rt = Runtime::new(None);

        let mut complex: ObjectRef = ComplexObject::from_f64(1.0, 2.0).to();
        complex = rt.alloc(complex).unwrap();

        let boxed: &Box<Builtin> = complex.0.borrow();
        let result = boxed.op_int(&rt).unwrap();
    }

    #[test]
    #[should_panic]
    fn __index__() {
        let mut rt = Runtime::new(None);

        let lhs = rt.alloc(ComplexObject::from_f64(1.0, 2.0).to()).unwrap();
        let boxed: &Box<Builtin> = lhs.0.borrow();

        boxed.op_index(&rt).unwrap();
    }
}
