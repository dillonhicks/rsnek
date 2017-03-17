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

    #[deprecated]
    #[inline]
    pub fn to_builtin(self) -> Builtin {
        return Builtin::Integer(self);
    }

    #[deprecated]
    #[inline]
    pub fn as_builtin(self) -> Builtin {
        return Builtin::Integer(self);
    }

    #[deprecated]
    #[inline]
    pub fn as_object_ref(self) -> ObjectRef {
        return Builtin::Integer(self).as_object_ref();
    }
}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    Python Object Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl object::model::PythonObject for IntegerObject {}
impl objectref::RtObject for IntegerObject {}
impl objectref::TypeInfo for IntegerObject {}

impl object::api::Identifiable for IntegerObject {
    fn op_equals(&self, rt: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match self.native_equals(builtin.deref()) {
            Ok(value) => {
                if value {
                    rt.alloc(ObjectRef::new(SINGLETON_TRUE_BUILTIN))
                } else {
                    rt.alloc(ObjectRef::new(SINGLETON_FALSE_BUILTIN))
                }
            }
            Err(err) => Err(err),
        }
    }

    fn native_equals(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        match other {
            &Builtin::Integer(ref obj) => Ok(self.value == obj.value),
            _ => Ok(false),
        }
    }
}


impl object::api::Hashable for IntegerObject {
    fn native_hash(&self) -> NativeResult<native::HashId> {
        let mut s = SipHasher::new();
        self.hash(&mut s);
        Ok(s.finish())
    }

    fn op_hash(&self, rt: &mut Runtime) -> RuntimeResult {
        match self.native_hash() {
            Ok(value) => rt.alloc(ObjectRef::new(Builtin::Integer(IntegerObject::new_u64(value)))),
            Err(err) => Err(err)
        }
    }
}


impl objectref::ObjectBinaryOperations for IntegerObject {
    fn add(&self, runtime: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        // If this fails the interpreter is fucked anyways because the runtime has been dealloc'd
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match builtin.deref() {
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
    use typedef::objectref::{self, ObjectBinaryOperations, ObjectRef};

    use runtime::{Runtime, DEFAULT_HEAP_CAPACITY};
    use typedef::integer;
    use typedef::builtin::Builtin;
    use super::{Integer, IntegerObject};
    use typedef::float::FloatObject;
    use typedef::string::StringObject;
    use typedef::tuple::TupleObject;
    use typedef::list::ListObject;
    use typedef::boolean::{SINGLETON_FALSE_BUILTIN, SINGLETON_TRUE_BUILTIN};
    use typedef::objectref::ToRtWrapperType;

    use num::ToPrimitive;
    use std::cmp::PartialEq;
    use object::api::Identifiable;
    use std::borrow::Borrow;



    #[test]
    fn test_integer_identity() {
        let mut runtime = Runtime::new(None);
        assert_eq!(runtime.heap_size(), 0);

        let one_object = ObjectRef::new(Builtin::Integer(IntegerObject::new_i64(1)));
        let one: ObjectRef = runtime.alloc(one_object.clone()).unwrap();

        let one_clone = one.clone();
        assert_eq!(Rc::strong_count(&one.0), 4);

    }


    /// Create integer object on the stack and try to allocate it
    /// in the runtime.
    ///
    #[test]
    fn test_alloc_integer() {
        let mut runtime = Runtime::new(None);
        assert_eq!(runtime.heap_size(), 0);

        let one_object = IntegerObject::new_i64(1).as_builtin().as_object_ref();
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

        let one_object = IntegerObject::new_i64(1).as_builtin().as_object_ref();
        let one: ObjectRef = runtime.alloc(one_object.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 1);

        let another_one = IntegerObject::new_i64(1).as_builtin().as_object_ref();
        let one2: ObjectRef = runtime.alloc(another_one.clone()).unwrap();
        assert_eq!(runtime.heap_size(), 2);

        let one_ref: &Box<Builtin> = one.0.borrow();
        let two = one_ref.add(&mut runtime, &another_one).unwrap();
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
        let result = one_builtin.op_equals(&mut rt, &one2).unwrap();

        assert_eq!(result, rt.True())
    }
}
