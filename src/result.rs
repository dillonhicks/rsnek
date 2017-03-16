use std;
use std::result;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::cell::RefCell;

use error::{Error};

use typedef;
use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;


pub type RuntimeResult = result::Result<ObjectRef, Error>;

pub type NativeResult<T> = result::Result<T, Error>;

//
//pub type IntegerResult = NativeResult<typedef::native::Integer>;
//pub type BooleanResult = NativeResult<typedef::native::Boolean>;
//
//pub type FloatResult = NativeResult<typedef::integer::Integer>;
//pub type IntegerResult = NativeResult<typedef::integer::Integer>;
//pub type IntegerResult = NativeResult<typedef::integer::Integer>;
//pub type IntegerResult = NativeResult<typedef::integer::Integer>;
//pub type IntegerResult = NativeResult<typedef::integer::Integer>;