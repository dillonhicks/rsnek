use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::fmt;
use std::ops::Deref;
use std::rc::{Weak, Rc};

use num::{FromPrimitive, BigInt};

use object;
use error::{Error, ErrorType};
use result::{NativeResult, RuntimeResult};
use runtime::Runtime;

use super::native;
use super::objectref;
use super::objectref::ObjectRef;
use super::builtin::Builtin;
use super::float::FloatObject;
use super::boolean::{SINGLETON_FALSE_BUILTIN, SINGLETON_TRUE_BUILTIN};


pub type Integer = BigInt;


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
        let integer = IntegerObject { value: BigInt::from(value) };

        return integer;
    }

    #[inline]
    pub fn new_u64(value: u64) -> IntegerObject {
        let integer = IntegerObject { value: BigInt::from(value) };

        return integer;
    }

    pub fn new_bigint(value: BigInt) -> IntegerObject {
        let integer = IntegerObject { value: BigInt::from(value) };

        return integer;
    }

    #[inline]
    pub fn to_builtin(self) -> Builtin {
        return Builtin::Integer(self);
    }

    #[inline]
    pub fn as_builtin(self) -> Builtin {
        return Builtin::Integer(self);
    }


    #[inline]
    pub fn as_object_ref(self) -> ObjectRef {
        return Builtin::Integer(self).as_object_ref();
    }
}

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///      Rt Object Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+


impl objectref::RtObject for IntegerObject {}
impl objectref::TypeInfo for IntegerObject {}

impl object::api::Identifiable for IntegerObject {
    fn op_equals(&self, rt: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let borrowed: &RefCell<Builtin> = rhs.0.borrow();
        let builtin = borrowed.borrow_mut();

        match self.native_equals(builtin.deref()) {
            Ok(value) => {
                if value {
                    rt.alloc(ObjectRef::new(SINGLETON_TRUE_BUILTIN))
                } else {
                    rt.alloc(ObjectRef::new(SINGLETON_FALSE_BUILTIN))
                }
            },
            Err(err) => Err(err)
        }
    }

    fn native_equals(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        match other {
            &Builtin::Integer(ref obj) => Ok(self.value == obj.value),
            _ => Ok(false)
        }
    }
}


impl object::api::Hashable for IntegerObject {

}


impl objectref::ObjectBinaryOperations for IntegerObject {
    fn add(&self, runtime: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        // If this fails the interpreter is fucked anyways because the runtime has been dealloc'd

        let borrowed: &RefCell<Builtin> = rhs.0.borrow();
        match borrowed.borrow_mut().deref() {
            &Builtin::Integer(ref obj) => {
                let new_number = IntegerObject::new_bigint(&self.value + &obj.value).as_builtin();
                runtime.alloc(new_number.as_object_ref())
            }
            &Builtin::Float(ref obj) => {
                let new_number = FloatObject::add_integer(obj, &self)?.as_builtin();
                runtime.alloc(new_number.as_object_ref())
            }
            _ => Err(Error(ErrorType::Type, "TypeError cannot add to int")),
        }
    }

    fn subtract(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
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
///      Rt Object Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl fmt::Display for IntegerObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
