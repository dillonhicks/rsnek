use std;
use num;

use runtime::{self, Runtime};
use typedef;
use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;
use result::{RuntimeResult, NativeResult};
use traits::New;

// Implementation specific types
//
// The strong reference counting type used by the runtime
// to keep track of heap allocated object. The simple gc will
// look for exactly 1 strong ref (to the heap object itself).
pub type RuntimeRef = std::rc::Rc<Box<typedef::builtin::Builtin>>;
pub type RuntimeWeakRef = std::rc::Weak<Box<typedef::builtin::Builtin>>;
pub type ObjectId = u64;
pub type HashId = u64;

// The well known primitive types that have a fairly direct
// 1:1 representation to rusts primitive types. The 'native api',
// along with not using the reference counting wrappers, will
// always return these types directly.
pub type Integer = num::BigInt;
pub type Float = f64;
pub type Boolean = bool;
pub type Complex = num::Complex<f64>;

pub type String = std::string::String;
pub type Bytes = Vec<u8>;
pub struct None();

//
// Collection Primitive Types
//
pub type List = Vec<ObjectRef>;
pub type Tuple = Vec<ObjectRef>;


#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct DictKey(pub HashId, pub ObjectRef);
pub type Dict = std::collections::HashMap<DictKey, ObjectRef>;
pub type KWDict = std::collections::HashMap<String, ObjectRef>;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct SetElement(pub HashId, pub ObjectRef);
pub type Set = std::collections::HashSet<SetElement>;

pub type NativeFn = Fn(&Tuple, &Tuple, &Dict) -> NativeResult<Builtin>;
pub type WrapperFn = Fn(&Runtime, &ObjectRef, &ObjectRef, &ObjectRef) -> RuntimeResult;

pub enum Function {
    Native(Box<NativeFn>),
    Wrapper(Box<WrapperFn>),
    ByteCode(),
}


#[derive(Debug)]
pub struct Object {
    pub class: ObjectRef,
    pub dict: ObjectRef,
    pub bases: ObjectRef,
}


/// Enum for numeric types
pub enum Number {
    Int(Integer),
    Float(Float),
    Bool(Boolean),
    Complex(Complex),
}


#[derive(Debug)]
pub struct Type {
    pub name: String,
    pub module: String,
    pub bases: Tuple,
    pub subclasses: std::cell::RefCell<List>,
}


//
//
//pub enum Collection {
//    Dict(Dict),
//    Tuple(Tuple),
//    List(List),
//    Str(String),
//    Bytes(Bytes),
//}
//
//pub enum Sequence {
//    Tuple(Tuple),
//    List(List),
//    Str(String),
//    Bytes(Bytes),
//}
