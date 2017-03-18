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

use super::builtin;
use super::builtin::Builtin;
use super::objectref;
use super::objectref::ObjectRef;
use super::native;

use num::ToPrimitive;




pub type Boolean = native::Integer;


#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct BooleanObject {
    value: Boolean,
}


pub const SINGLETON_TRUE: BooleanObject = BooleanObject { value: 1 };
pub const SINGLETON_FALSE: BooleanObject = BooleanObject { value: 0 };
pub const SINGLETON_TRUE_BUILTIN: builtin::Builtin = builtin::Builtin::Boolean(SINGLETON_TRUE);
pub const SINGLETON_FALSE_BUILTIN: builtin::Builtin = builtin::Builtin::Boolean(SINGLETON_FALSE);

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///       Struct Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl BooleanObject {
    pub fn new_u64(value: u64) -> BooleanObject {
        match value {
            0 => SINGLETON_FALSE,
            _ => SINGLETON_TRUE,
        }
    }

    pub fn new_f64(value: f64) -> BooleanObject {
        match value {
            0.0 => SINGLETON_FALSE,
            _ => SINGLETON_TRUE,
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
        let equality = match rhs {
            &Builtin::Boolean(ref obj) => self.value == obj.value,
            &Builtin::Integer(ref obj) => *self == BooleanObject::new_u64(obj.value.to_u64().unwrap_or_default()),
            &Builtin::Float(ref obj) =>  *self == BooleanObject::new_f64(obj.value),
            _ => self.value == 1,
        };

        Ok(equality)
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
mod tests {
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
    use super::{SINGLETON_FALSE_BUILTIN, SINGLETON_TRUE_BUILTIN};
    use typedef::objectref::ToRtWrapperType;

    use num::ToPrimitive;
    use std::cmp::PartialEq;
    use std::borrow::Borrow;

    /// Reference equality
    ///  True is True
    #[test]
    fn test_boolean_identity() {
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
    fn test_boolean_equality() {
        let mut runtime = Runtime::new(None);
        assert_eq!(runtime.heap_size(), 0);

        let False = ObjectRef::new(SINGLETON_FALSE_BUILTIN);
        let True = ObjectRef::new(SINGLETON_TRUE_BUILTIN);

        let thing1 = runtime.alloc(False.clone()).unwrap();
        let False2 = runtime.alloc(False.clone()).unwrap();
        let thing3 = runtime.alloc(True.clone()).unwrap();

        let False_ref: &Box<Builtin> = False.0.borrow();

        let result = False_ref.op_eq(&mut runtime, &False2.clone()).unwrap();
        assert_eq!(result, True, "BooleanObject equality (op_equals)");

        let result = False_ref.native_eq(False_ref).unwrap();
        assert_eq!(result, true);
    }




}