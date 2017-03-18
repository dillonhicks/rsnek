use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::fmt;
use std::ops::Deref;
use std::rc::{Weak, Rc};
use std::hash::{Hash, SipHasher, Hasher};

use num::{FromPrimitive, BigInt};

use object;
use error::{Error, ErrorType};
use result::{NativeResult, RuntimeResult};
use runtime::Runtime;
use typedef::objectref::ToRtWrapperType;

use super::native;
pub use typedef::native::Integer;
use super::objectref;
use super::objectref::ObjectRef;
use super::builtin::Builtin;
use super::float::FloatObject;


#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct IntegerObject {
    pub value: Integer,
}

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///       Struct Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl IntegerObject {
    #[inline]
    pub fn new_i64(value: i64) -> IntegerObject {
        let integer = IntegerObject { value: Integer::from(value) };

        return integer;
    }

    #[inline]
    pub fn new_u64(value: u64) -> IntegerObject {
        let integer = IntegerObject { value: Integer::from(value) };

        return integer;
    }

    pub fn new_bigint(value: Integer) -> IntegerObject {
        let integer = IntegerObject { value: Integer::from(value) };

        return integer;
    }

}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    Python Object Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl objectref::RtObject for IntegerObject {}
impl object::model::PyObject for IntegerObject {}
impl object::model::PyBehavior for IntegerObject {

    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match self.native_eq(builtin.deref()) {
            Ok(value) => {
                if value {
                    Ok(rt.True())
                } else {
                    Ok(rt.False())
                }
            }
            Err(err) => Err(err),
        }
    }

    fn native_eq(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        match other {
            &Builtin::Integer(ref obj) => Ok(self.value == obj.value),
            _ => Ok(false),
        }
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
        let mut s = SipHasher::new();
        self.hash(&mut s);
        Ok(s.finish())
    }

    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_hash() {
            Ok(value) => rt.alloc(ObjectRef::new(Builtin::Integer(IntegerObject::new_u64(value)))),
            Err(err) => Err(err)
        }
    }

    fn op_add(&self, runtime: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        // If this fails the interpreter is fucked anyways because the runtime has been dealloc'd
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match builtin.deref() {
            &Builtin::Integer(ref obj) => {
                let new_number: ObjectRef = IntegerObject::new_bigint(&self.value + &obj.value).to();
                runtime.alloc(new_number)
            }
            &Builtin::Float(ref obj) => {
                let new_number: ObjectRef = FloatObject::add_integer(obj, &self)?.to();
                runtime.alloc(new_number)
            }
            _ => Err(Error(ErrorType::Type, "TypeError cannot add to int")),
        }
    }
}


impl objectref::ToRtWrapperType<Builtin> for IntegerObject {
    #[inline]
    fn to(self) -> Builtin {
        return Builtin::Integer(self);
    }
}

impl objectref::ToRtWrapperType<ObjectRef> for IntegerObject {
    #[inline]
    fn to(self) -> ObjectRef {
        ObjectRef::new(self.to())
    }
}


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///      stdlib Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl fmt::Display for IntegerObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
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

    use runtime::{Runtime, DEFAULT_HEAP_CAPACITY};
    use typedef::integer;
    use typedef::builtin::Builtin;
    use super::{Integer, IntegerObject};
    use typedef::float::FloatObject;
    use typedef::string::StringObject;
    use typedef::tuple::TupleObject;
    use typedef::list::ListObject;
    use typedef::objectref::ToRtWrapperType;
    use object::model::PyBehavior;

    use num::ToPrimitive;
    use std::cmp::PartialEq;
    use std::borrow::Borrow;

    #[test]
    fn test_integer_alloc() {
        let mut runtime = Runtime::new(None);
        assert_eq!(runtime.heap_size(), 0);

        let one_object = ObjectRef::new(Builtin::Integer(IntegerObject::new_i64(1)));
        let one: ObjectRef = runtime.alloc(one_object.clone()).unwrap();

        let one_clone = one.clone();
        assert_eq!(Rc::strong_count(&one.0), 5);

    }


    /// Create integer object on the stack and try to allocate it
    /// in the runtime.
    ///
    #[test]
    fn test_alloc_integer() {
        let mut runtime = Runtime::new(None);
        assert_eq!(runtime.heap_size(), 0);

        let one_object: ObjectRef = IntegerObject::new_i64(1).to();
        let one: ObjectRef = runtime.alloc(one_object.clone()).unwrap();

        /// A new integer should only alloc one spot on the heap
        assert_eq!(runtime.heap_size(), 1);
        println!("{:?}", runtime);
    }

    /// int+int => int
    /// api::BinaryOperation::op_add
    #[test]
    fn test_int_add_int_equals_int() {
        let mut runtime = Runtime::new(None);
        assert_eq!(runtime.heap_size(), 0);

        let one_object: ObjectRef = IntegerObject::new_i64(1).to();
        let one: ObjectRef = runtime.alloc(one_object.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 1);

        let another_one: ObjectRef = IntegerObject::new_i64(1).to();
        let one2: ObjectRef = runtime.alloc(another_one.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 2);

        let one_ref: &Box<Builtin> = one.0.borrow();
        let two = one_ref.op_add(&mut runtime, &another_one).unwrap();
        assert_eq!(runtime.heap_size(), 3);

        println!("{:?}", runtime);
    }

    /// Just try to init the runtime
    #[test]
    fn integer_equality() {
        let mut rt = Runtime::new(None);
        assert_eq!(rt.heap_size(), 0);


        let one_object: ObjectRef = IntegerObject::new_i64(1).to();
        let one: ObjectRef = rt.alloc(one_object.clone()).unwrap();
        assert_eq!(rt.heap_size(), 1);

        let another_one: ObjectRef= IntegerObject::new_i64(1).to();
        let one2: ObjectRef = rt.alloc(another_one.clone()).unwrap();
        assert_eq!(rt.heap_size(), 2);

        println!("{:?}", rt);

        let one_builtin: &Box<Builtin> = one.0.borrow();
        let result = one_builtin.op_eq(&mut rt, &one2).unwrap();

        assert_eq!(result, rt.True())
    }
}
