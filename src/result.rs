use std::result;
use error::Error;
use typedef::objectref::ObjectRef;


pub type RuntimeResult = result::Result<ObjectRef, Error>;

/// Native results bypass the heap.alloc_*() and are not reference
/// counted. There is nothing preventing the use of an ObjectRef
/// as a NativeResult type, however the intention is to use the more
/// platform native types in order to reduce overhead for native api cases.
///
/// When there is a case that a native result could be one of many, consider
/// use the Builtin wrapper.
pub type NativeResult<T> = result::Result<T, Error>;