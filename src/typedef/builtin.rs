use std;
use std::ops::Deref;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

use object;
use object::api::Identifiable;
use runtime::Runtime;
use result::{NativeResult, RuntimeResult};
use error::{Error, ErrorType};


use super::objectref;
use super::objectref::{RtObject, ObjectRef};
use super::boolean::BooleanObject;
use super::integer::IntegerObject;
use super::float::FloatObject;
use super::string::StringObject;
use super::tuple::TupleObject;
use super::list::ListObject;
use super::complex::ComplexObject;
use super::set::SetObject;
use super::frozenset::FrozenSetObject;
use super::dictionary::DictionaryObject;
use super::objectref::ObjectBinaryOperations;
use super::native;


#[macro_use]
use super::macros;

pub type CastResult<T: RtObject> = Result<T, Error>;


#[derive(Clone, Debug)]
pub enum Builtin {
    Integer(IntegerObject),
    Float(FloatObject),
    String(StringObject),
    Tuple(TupleObject),
    List(ListObject),
    Set(SetObject),
    FrozenSet(FrozenSetObject),
    Dictionary(DictionaryObject),
    Complex(ComplexObject),

    // Not yet implemented
    Boolean(BooleanObject),
    Object(()),
    Function(()),
    Method(()),
    None(()),
    Module(()), /*    Type(TypeObject),
                 *    Meta(MetaObject),
                 *    None(NoneObject) */
}


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///     Struct Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl Builtin {
    pub fn int(&self) -> CastResult<&IntegerObject> {
        match *self {
            Builtin::Integer(ref obj) => Ok(&obj),
            _ => Err(Error(ErrorType::Type, "Not an IntegerObject")),
        }
    }

    pub fn float(&self) -> CastResult<&FloatObject> {
        match *self {
            Builtin::Float(ref obj) => Ok(&obj),
            _ => Err(Error(ErrorType::Type, "Not a FloatObject")),
        }
    }

    pub fn tuple(&self) -> CastResult<&TupleObject> {
        match *self {
            Builtin::Tuple(ref obj) => Ok(&obj),
            _ => Err(Error(ErrorType::Type, "Not a TupleObject")),
        }
    }

    pub fn list(&self) -> CastResult<&ListObject> {
        match *self {
            Builtin::List(ref obj) => Ok(&obj),
            _ => Err(Error(ErrorType::Type, "Not a ListObject")),
        }
    }

    pub fn string(&self) -> CastResult<&StringObject> {
        match *self {
            Builtin::String(ref obj) => Ok(&obj),
            _ => Err(Error(ErrorType::Type, "Not a StringObject")),
        }
    }

    pub fn as_object_ref(self) -> ObjectRef {
        ObjectRef::new(self)
    }
}


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///     RtObject Traits
///
/// For the BuiltinObject this should mean just proxy dispatching the
/// underlying associated function using the foreach macros.
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl objectref::RtObject for Builtin {}

impl objectref::TypeInfo for Builtin {}

impl objectref::ObjectBinaryOperations for Builtin {
    fn add(&self, runtime: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, runtime, add, lhs, rhs)
    }

    fn subtract(&self, runtime: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, runtime, subtract, lhs, rhs)
    }
}

/// Builtin is an intermediate wrapper/union type and should not
/// expose identity
impl object::api::Identifiable for Builtin {
    fn identity(&self, rt: &mut Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, identity, lhs)
    }

    /// Short circuit the ident to hit the wrapper since
    /// the macro unwrapping causes an extra layer of indirection
    /// and makes comparing porinters at the Object level harder.
    //
    // fn native_identity(&self) -> native::ObjectId {
    //   return (&self as *const _) as native::ObjectId;
    // }

    fn op_is(&self, rt: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_is, lhs, rhs)
    }

    fn op_is_not(&self, rt: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_is_not, lhs, rhs)
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn op_equals(&self, rt: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_equals, lhs, rhs)
    }

    fn op_not_equals(&self, rt: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_not_equals, lhs, rhs)
    }

    fn native_is(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_is, lhs, rhs)
    }

    fn native_is_not(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_is_not, lhs, rhs)
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn native_equals(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_equals, lhs, rhs)
    }

    fn native_not_equals(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_not_equals, lhs, rhs)
    }
}


impl object::api::Hashable for Builtin {
    fn op_hash(&self, rt: &mut Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_hash, obj)
    }

    fn native_hash(&self) -> NativeResult<native::HashId>{
        native_foreach_builtin!(self, native_hash, obj)
    }

}


impl objectref::ToRtWrapperType<Builtin> for Builtin {
    #[inline]
    fn to(self) -> Builtin {
        self
    }
}

impl objectref::ToRtWrapperType<ObjectRef> for Builtin {
    #[inline]
    fn to(self) -> ObjectRef {
        ObjectRef::new(self)
    }
}

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///     stdlib Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl std::cmp::PartialEq for Builtin {
    fn eq(&self, rhs: &Builtin) -> bool {
        self.native_equals(rhs).unwrap()
    }
}


impl std::cmp::Eq for Builtin {}


impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Builtin::Integer(ref obj) => write!(f, "{}", obj),
            &Builtin::Float(ref obj) => write!(f, "{}", obj),
            &Builtin::String(ref obj) => write!(f, "{}", obj),
            &Builtin::Tuple(ref obj) => write!(f, "{}", obj),
            &Builtin::List(ref obj) => write!(f, "{}", obj),
            _ => write!(f, "BuiltinType"),
        }
    }
}

