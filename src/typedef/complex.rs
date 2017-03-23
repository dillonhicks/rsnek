use std;
use std::fmt;

use result::RuntimeResult;
use runtime::Runtime;
use object;

use typedef::native;
use typedef::objectref;
use typedef::builtin::Builtin;
use typedef::objectref::RtObject;
use typedef::objectref::ObjectRef;
use object::model::PyBehavior;


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

        let False = rt.False();
        let False2 = False.clone();

        let False_ref: &Box<Builtin> = False.0.borrow();

        let result = False_ref.native_is(False_ref.deref()).unwrap();
        assert_eq!(result, true, "BooleanObject native is(native_is)");

        let result = False_ref.op_is(&mut rt, &False2).unwrap();
        assert_eq!(result, rt.True(), "BooleanObject is(op_is)");

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
        assert_eq!(result, rt.True());

        let result = boxed.op_ne(&rt, &rhs_eq).unwrap();
        assert_eq!(result, rt.False());

        let result = boxed.op_eq(&rt, &rhs_ne).unwrap();
        assert_eq!(result, rt.False());

        let result = boxed.op_ne(&rt, &rhs_ne).unwrap();
        assert_eq!(result, rt.True());
    }

    #[test]
    #[allow(non_snake_case)]
    fn __bool__() {
        let mut rt = Runtime::new(None);

        let lhs = rt.alloc(ComplexObject::from_f64(1.0, 2.0).to()).unwrap();

        let boxed: &Box<Builtin> = lhs.0.borrow();
        let result = boxed.op_bool(&rt).unwrap();

        assert_eq!(result, rt.True());
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
