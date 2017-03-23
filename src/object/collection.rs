use std::borrow::Borrow;
use num::FromPrimitive;

use error::Error;
use runtime::Runtime;
use result::{RuntimeResult, NativeResult};
use typedef::builtin::Builtin;
use typedef::native;
use typedef::objectref::ObjectRef;

use object::method;


pub trait Iterator: method::Iter + method::Next {}

pub trait Reverse: method::Iter + method::Reversed {}

pub trait Generator: Iterator + method::Close + method::Send + method::Throw {}

pub trait Collection: method::Length + method::Iter + method::Contains {}

pub trait Sequence
    : Reverse + Collection + method::GetItem + method::Length + method::Index + method::Count {
}

pub trait ByteString: Sequence {}

pub trait MutableSequence
    : Sequence + method::Append + method::Extend + method::Pop + method::Remove + method::InPlaceAdd {
}

pub trait Set:
    Collection +
    method::GetItem +
    method::LessOrEqual +
    method::LessThan +
    method::Equal +
    method::NotEqual +
    method::GreaterThan +
    method::GreaterOrEqual +
    method::BitwiseAnd +
    method::BitwiseOr +
    method::Subtract +
    method::IsDisjoint {}


pub trait MutableSet:
    Set +
    method::AddItem +
    method::Discard +
    method::Clear +
    method::Pop +
    method::Remove +
    method::InPlaceBitwiseOr +
    method::InPlaceBitwiseAnd +
    method::InPlaceXOr +
    method::InPlaceSubtract {}

pub trait Mapping
    : Collection + method::GetItem + method::Keys + method::Values + method::Items + method::Get + method::Equal + method::NotEqual
    {
}

pub trait MutableMapping
    : Mapping + method::SetItem + method::DeleteItem + method::Pop + method::PopItem + method::Clear + method::Update + method::SetDefault
    {
}
