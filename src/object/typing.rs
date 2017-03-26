use error::Error;
use runtime::Runtime;
use result::{RuntimeResult, NativeResult};
use typedef::builtin::Builtin;
use typedef::native;

use object::method;


// TODO: Allow arguments to new and init
// TODO: Inheritance? Missing bases, mro, etc.
/// __builtins__.type: Defines how types are created
pub trait Type {
    type T;

    /// __new__
    fn op_new(&self, &Runtime) -> RuntimeResult;
    fn native_new(&self) -> NativeResult<Self::T>;

    /// __init___
    fn op_init(&mut self, &Runtime) -> RuntimeResult;
    fn native_init(&mut self) -> NativeResult<native::NoneValue>;

    /// __name__ (e.g. self.__class__.__name__)
    fn op_name(&self, &Runtime) -> RuntimeResult;
    fn native_name(&self) -> NativeResult<native::String>;

    api_method!(unary, self, __bases__, HasBases, op_bases, native_bases);
    api_method!(unary, self, __del__, Delete, op_del, native_del);
}

//pub trait TypeAPI: method::
