use runtime::Runtime;
use typedef::native;
use typedef::objectref::ObjectRef;

use object::method;


pub trait HasName {
    fn get_name(&self) -> native::String;
}


// TODO: {T49} Investigate an actual type opbject. Things to pinder: Allow arguments to new and
// init, inheritance? Missing bases, mro, etc.
/// __builtins__.type: Defines how types are created
pub trait Type: method::New + method::Init + HasName {}


pub trait BuiltinType {
    type T;
    type V;

    //fn name() -> &'static str;

    /// Create the type and do any static initialization that may be needed
    fn init_type() -> Self;

    fn inject_selfref(Self::T) -> ObjectRef;

    /// Create an instance of the type ane return the struct that contains
    /// the state but is not yet reference counted.
    fn alloc(value: Self::V) -> Self::T;

    /// Create a new instance of the primitve type that his reference counted
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef;

}
