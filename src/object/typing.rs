use error::Error;
use runtime::Runtime;
use result::{RuntimeResult, NativeResult};
use typedef::builtin::Builtin;
use typedef::native;

use object::method;


pub trait HasName {
    fn get_name(&self) -> NativeResult<native::String>;
}

// TODO: Allow arguments to new and init
// TODO: Inheritance? Missing bases, mro, etc.
/// __builtins__.type: Defines how types are created
pub trait Type: method::New + method::Init + HasName {}
