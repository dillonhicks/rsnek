use error::Error;
use runtime::Runtime;
use result::{RuntimeResult, NativeResult};
use typedef::builtin::Builtin;
use typedef::native;


// TODO: Allow arguments to new and init
// TODO: Inheritance? Missing bases, mro, etc.
/// __builtins__.type: Defines how types are created
pub trait Type {
    type T;

    /// Create the type and register it with the runtime
    fn create(&Runtime) -> Self;

    /// __new__
    fn op_new(&self) -> RuntimeResult;
    fn native_new(&self) -> NativeResult<Self::T>;

    /// __init___
    fn op_init(&self) -> RuntimeResult;
    fn native_init(&self) -> NativeResult<Self::T>;

    /// __name__ (e.g. self.__class__.__name__)
    fn op_name(&self) -> RuntimeResult;
    fn native_name(&self) -> NativeResult<native::String>;

    api_method!(unary, self, __bases__, HasBases, op_bases, native_bases);
    api_method!(unary, self, __del__, Delete, op_del, native_del);
}