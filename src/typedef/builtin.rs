use std;
use std::ops::Deref;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

use object;
use runtime::Runtime;
use result::{NativeResult, RuntimeResult};
use error::{Error, ErrorType};
use object::model::PyBehavior;

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

}


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///     Python Object Traits
///
/// For the BuiltinObject this should mean just proxy dispatching the
/// underlying associated function using the foreach macros.
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl objectref::RtObject for Builtin {}
impl object::model::PyObject for Builtin {}
impl object::model::PyBehavior for Builtin {

    //
    // Identity and Equality
    //

    fn identity(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, identity, lhs)
    }

    /// Short circuit the ident to hit the wrapper since
    /// the macro unwrapping causes an extra layer of indirection
    /// and makes comparing porinters at the Object level harder.
    //
    //     fn native_identity(&self) -> native::ObjectId {
    //        native_foreach_builtin!(self, native_identity, lhs)
    //        //return (&self as *const _) as native::ObjectId;
    //     }

    fn op_is(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_is, lhs, rhs)
    }

    fn op_is_not(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_is_not, lhs, rhs)
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_eq, lhs, rhs)
    }

    fn op_ne(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_ne, lhs, rhs)
    }

    fn native_is(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_is, lhs, rhs)
    }

    fn native_is_not(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_is_not, lhs, rhs)
    }

    /// Default implementation of equals fallsbacks to op_is.
    fn native_eq(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_eq, lhs, rhs)
    }

    fn native_ne(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        native_foreach_builtin!(self, native_ne, lhs, rhs)
    }



    //
    // Hash
    //
    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_hash, obj)
    }

    fn native_hash(&self) -> NativeResult<native::HashId>{
        native_foreach_builtin!(self, native_hash, obj)
    }

    //
    // Numeric Conversions
    //
    fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_bool, obj)
    }

    fn native_bool(&self) -> NativeResult<native::Boolean>{
        native_foreach_builtin!(self, native_bool, obj)
    }
    
    fn op_int(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_int, obj)
    }

    fn native_int(&self) -> NativeResult<native::Integer>{
        native_foreach_builtin!(self, native_int, obj)
    }

    fn op_float(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_float, obj)
    }

    fn native_float(&self) -> NativeResult<native::Float>{
        native_foreach_builtin!(self, native_float, obj)
    }

    fn op_complex(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_complex, obj)
    }

    fn native_complex(&self) -> NativeResult<native::Complex>{
        native_foreach_builtin!(self, native_complex, obj)
    }

    fn op_index(&self, rt: &Runtime) -> RuntimeResult {
        foreach_builtin!(self, rt, op_index, obj)
    }

    fn native_index(&self) -> NativeResult<native::Integer>{
        native_foreach_builtin!(self, native_index, obj)
    }

    // Numeric operators
    fn op_add(&self, rt: &Runtime, rhs:&ObjectRef) -> RuntimeResult {
        foreach_builtin!(self, rt, op_add, lhs, rhs)
    }

    fn native_add(&self, rhs: &Builtin) -> NativeResult<Builtin>{
        native_foreach_builtin!(self, native_add, lhs, rhs)
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
        self.native_eq(rhs).unwrap()
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

