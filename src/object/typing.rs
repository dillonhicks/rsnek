use error::Error;
use runtime::Runtime;
use result::{RuntimeResult, NativeResult};
use typedef::builtin::Builtin;
use typedef::native;
use typedef::objectref::ObjectRef;

use object::method;


pub trait HasName {
    fn get_name(&self) -> native::String;
}

// TODO: Allow arguments to new and init
// TODO: Inheritance? Missing bases, mro, etc.
/// __builtins__.type: Defines how types are created
pub trait Type: method::New + method::Init + HasName {}
pub trait BuiltinTypeAPI {
    type T;
    type V;

    /// Create a new instance of the primitve type that his reference counted
    fn new(rt: &Runtime, value: Self::V) -> ObjectRef;

    /// Create an instance of the type ane return the struct that contains
    /// the state but is not yet reference counted.
    fn alloc(value: Self::V) -> Self::T;
}
