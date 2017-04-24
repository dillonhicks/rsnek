use std;
use std::cell::Cell;
use num;

use rsnek_compile::Instr;

use runtime::Runtime;
use typedef;
use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;
use result::{RuntimeResult, NativeResult};

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

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct SetElement(pub HashId, pub ObjectRef);
#[allow(dead_code)]
pub type Set = std::collections::HashSet<SetElement>;

pub type NativeFnArgs = (Tuple, Tuple, Dict);
pub type FnArgs = (ObjectRef, ObjectRef, ObjectRef);

pub type NativeFn = Fn(&Tuple, &Tuple, &Dict) -> NativeResult<Builtin>;
pub type WrapperFn = Fn(&Runtime, &ObjectRef, &ObjectRef, &ObjectRef) -> RuntimeResult;

pub enum Function {
    Native(Box<NativeFn>),
    Wrapper(Box<WrapperFn>),
    ByteCode(),
}

#[derive(Debug)]
pub enum Iterator {
    Sequence {source: ObjectRef, idx_next: Cell<u64>},
    Empty,
}

impl Iterator {
    pub fn new(source: &ObjectRef) -> NativeResult<Self> {
        // TODO: {T101} Type assertions on new iterators or make it part of the `iter()`
        // builtin
        Ok(Iterator::Sequence {source: source.clone(), idx_next: Cell::new(0)})
    }
}

#[derive(Debug)]
pub struct Object {
    pub class: ObjectRef,
    pub dict: ObjectRef,
    pub bases: ObjectRef,
}

#[allow(dead_code)]
pub struct Module {
    pub name: ObjectRef,

}

/// Enum for numeric types
#[allow(dead_code)]
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

pub struct Code {
    pub co_name: String,
    pub co_names: Vec<String>,
    pub co_varnames: Tuple,
    pub co_code: Vec<Instr>,
    pub co_consts: Tuple,

    //pub co_argcount: Int,
    //pub co_cellvars: Tuple,
    //pub co_filename: Str,
    //pub co_firstlineno: Int,
    //pub co_flags: Int,
    //pub co_freevars: Tuple,
    //pub co_kwonlyargcount: Int,
    //pub co_lnotab: Bytes,

    //pub co_nlocals: Int,
    //pub co_stacksize: Int,
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

//#[derive(Debug)]
//pub enum Sequence {
//    Tuple(Tuple),
//    List(List),
//    Str(String),
//    Bytes(Bytes),
//}
