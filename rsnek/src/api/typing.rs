//! Skeleton definitions for traits and properties required for
//! a flexible type system.
use ::api::method;
use ::api::RtObject as ObjectRef;
use ::runtime::Runtime;
use ::system::primitives as rs;


pub trait HasName {
    fn get_name(&self) -> rs::String;
}


// TODO: {T49} Investigate an actual type object. Things to ponder: Allow arguments to new and
// init, inheritance? Missing bases, mro, etc.
/// __builtins__.type: Defines how types are created
pub trait Type: method::New + method::Init + HasName {}


/// The primordial trait from which all builtin `Py*` types are
/// created.
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
