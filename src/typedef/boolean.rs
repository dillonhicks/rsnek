use std;
use std::fmt;
use std::cell::RefCell;
use std::ops::Deref;
use std::borrow::Borrow;

use object;
use object::api;
use object::model::PyBehavior;
use runtime::Runtime;
use result::{RuntimeResult, NativeResult};
use typedef::integer::IntegerObject;
use typedef::objectref::ToRtWrapperType;

use super::builtin;
use super::builtin::Builtin;
use super::objectref;
use super::objectref::ObjectRef;
use super::native;

use num::Zero;
use num::FromPrimitive;
use num::ToPrimitive;



pub type Boolean = native::Integer;


#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct BooleanObject {
    value: Boolean,
}


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///       Struct Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl BooleanObject {
    pub fn new_true() -> BooleanObject {
        return BooleanObject{
            value: native::Integer::from(1)
        }
    }

    pub fn new_false() -> BooleanObject {
        return BooleanObject{
            value: native::Integer::from(0)
        }
    }
}

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///    Python Object Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl objectref::RtObject for BooleanObject {}
impl object::model::PyObject for BooleanObject {}
impl object::model::PyBehavior for BooleanObject {

    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match self.native_eq(builtin.deref()) {
            Ok(value) => {
                if value {
                    Ok(rt.True())
                } else {
                    Ok(rt.False())
                }
            },
            Err(err) => Err(err)
        }
    }

    fn native_eq(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        match rhs.native_bool() {
            Ok(value) => Ok(self.native_bool().unwrap() == value),
            Err(err) => Err(err)
        }
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
        return Ok(!self.value.is_zero())
    }

    fn op_int(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_int() {
            Ok(int) => {
                let int_obj: ObjectRef = IntegerObject::new_bigint(int).to();
                rt.alloc(int_obj)
            },
            Err(err) => Err(err)
        }

    }

    fn native_int(&self) -> NativeResult<native::Integer> {
        return Ok(self.value.clone())
    }
}


impl objectref::ToRtWrapperType<builtin::Builtin> for BooleanObject {
    fn to(self) -> builtin::Builtin {
        builtin::Builtin::Boolean(self)
    }
}

impl objectref::ToRtWrapperType<objectref::ObjectRef> for BooleanObject {
    fn to(self) -> ObjectRef {
        ObjectRef::new(builtin::Builtin::Boolean(self))
    }
}


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///        stdlib Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl fmt::Display for BooleanObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///          Tests
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

#[cfg(test)]
#[allow(non_snake_case)]
mod test_pybehavior {
    use std;
    use std::rc::Rc;
    use std::ops::Deref;
    use typedef::objectref::{self, ObjectRef};
    use object::model::PyBehavior;

    use runtime::{Runtime, DEFAULT_HEAP_CAPACITY};
    use typedef::integer;
    use typedef::builtin::Builtin;
    use typedef::integer::{Integer, IntegerObject};
    use typedef::float::FloatObject;
    use typedef::string::StringObject;
    use typedef::tuple::TupleObject;
    use typedef::list::ListObject;
    use typedef::objectref::ToRtWrapperType;

    use num::ToPrimitive;
    use std::cmp::PartialEq;
    use std::borrow::Borrow;

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
        assert_eq!(rt.heap_size(), 0);

        let False = rt.False();
        let True = rt.True();

        let thing1 = rt.alloc(False.clone()).unwrap();
        let False2 = rt.alloc(False.clone()).unwrap();
        let thing3 = rt.alloc(True.clone()).unwrap();

        let False_ref: &Box<Builtin> = False.0.borrow();

        let result = False_ref.op_eq(&rt, &False2.clone()).unwrap();
        assert_eq!(result, True, "BooleanObject equality (op_equals)");

        let result = False_ref.native_eq(False_ref).unwrap();
        assert_eq!(result, true);
    }


    #[test]
    #[allow(non_snake_case)]
    fn __bool__() {
        let mut rt = Runtime::new(None);

        let True = rt.True();
        let True_ref: &Box<Builtin> = True.0.borrow();

        let result = True_ref.op_bool(&rt).unwrap();
        assert_eq!(rt.True(), result);

        let result = True_ref.native_bool().unwrap();
        assert_eq!(result, true);

    }

    #[test]
    fn __int__() {
        let mut rt = Runtime::new(None);

        let one: ObjectRef = IntegerObject::new_u64(1).to();

        let True = rt.True();
        let True_ref: &Box<Builtin> = True.0.borrow();

        let result = True_ref.op_int(&rt).unwrap();
        assert_eq!(result, one);
    }


}