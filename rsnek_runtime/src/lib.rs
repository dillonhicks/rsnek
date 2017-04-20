#![feature(associated_consts)]
#![feature(type_ascription)]
#![feature(const_fn)]
#![feature(exclusive_range_pattern)]
#![feature(test)]
#![feature(fn_traits)]

/// # Notes and TODOS
///
///  - TODO: Consider having objects implement an `pub fn alloc(self, rt: Runtime) -> ObjectRef`
///         - Generally cleanup the `as_builtin()` and `as_object_ref()` shit
///
/// - TODO: Determine if there is a generic way to increment rc's for adds to collections
///
///  - TODO: Some types need a weak ref back to their own RC in order to return back the proper
///          runtime result. We may also need an id or something else in order to look up
///          the in place modifyables down the chain.
///
///  - TODO: Consider a lighter weight NativeBuiltin union/enum for polymorphic native type cases
///
extern crate test;
extern crate num;
extern crate itertools;
extern crate fringe;
extern crate rustyline;

#[macro_use] extern crate serde_derive;
extern crate serde;

extern crate rsnek_compile;

#[macro_use]
mod macros;



mod builtin;
mod traits;
mod object;
mod result;
mod error;
mod typedef;

pub mod resource;

#[macro_use]
pub mod runtime;
