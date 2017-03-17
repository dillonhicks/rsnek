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

/// Native results bypass the heap.alloc_*() and are not reference
/// counted. There is nothing preventing the use of an ObjectRef
/// as a NativeResult type, however the intention is to use the more
/// platform native types in order to reduce overhead for native api cases.
///
/// When there is a case that a native result could be one of many, consider
/// use the Builtin wrapper.
pub type NativeResult<T> = result::Result<T, Error>;



//pub type IntegerResult = NativeResult<typedef::native::Integer>;
//pub type BooleanResult = NativeResult<typedef::native::Boolean>;
//pub type FloatResult = NativeResult<typedef::integer::Integer>;
//pub type IntegerResult = NativeResult<typedef::integer::Integer>;
//pub type IntegerResult = NativeResult<typedef::integer::Integer>;
//pub type IntegerResult = NativeResult<typedef::integer::Integer>;
//pub type IntegerResult = NativeResult<typedef::integer::Integer>;