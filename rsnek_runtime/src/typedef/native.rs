use std;
use std::fmt;
use std::cell::Cell;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

use num;
#[allow(unused_imports)]
use num::ToPrimitive;
use num::Num as NumTrait;
use serde::ser::{Serializer};

use rsnek_compile::{Id, Tag, Num, OwnedTk};

use ::opcode::OpCode;
use ::result::{RuntimeResult, NativeResult};
use ::runtime::Runtime;
use ::typedef;
use ::object::RtObject as ObjectRef;
use ::typedef::builtin::Builtin;



// Implementation specific types
//
// The strong reference counting type used by the runtime
// to keep track of heap allocated object. The simple gc will
// look for exactly 1 strong ref (to the heap object itself).
// TODO: {T3088} The added Box<> here seems like a level of indirection that is not needed
//  and was probably because I was a rust noob at that point.
pub type RuntimeRef = std::rc::Rc<Box<typedef::builtin::Builtin>>;
pub type RuntimeWeakRef = std::rc::Weak<Box<typedef::builtin::Builtin>>;
pub type ObjectId = u64;
pub type HashId = u64;

// The well known primitive types that have a fairly direct
// 1:1 representation to rusts primitive types. The 'native api',
// along with not using the reference counting wrappers, will
// always return these types directly.
pub type Integer = num::BigInt;
pub type Count = usize;
pub type Float = f64;
pub type Boolean = bool;
pub type Complex = num::complex::Complex<Float>;

pub type Byte = u8;

pub type String = std::string::String;
pub type Bytes = Vec<Byte>;
pub struct None();


//
// Collection Primitive Types
//
pub type List = Vec<ObjectRef>;
pub type Tuple = Vec<ObjectRef>;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct DictKey(pub HashId, pub ObjectRef);
impl DictKey {
    pub fn hash(&self) -> HashId {
        self.0
    }

    pub fn value(&self) -> ObjectRef {
        self.1.clone()
    }
}

pub type Dict = std::collections::HashMap<DictKey, ObjectRef>;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct SetElement(pub HashId, pub ObjectRef);
pub type Set = std::collections::HashSet<SetElement>;

pub type NativeFnArgs = (Tuple, Tuple, Dict);
pub type FnArgs = (ObjectRef, ObjectRef, ObjectRef);

pub type NativeFn = Fn(&Tuple, &Tuple, &Dict) -> NativeResult<Builtin>;
pub type WrapperFn = Fn(&Runtime, &ObjectRef, &ObjectRef, &ObjectRef) -> RuntimeResult;

#[derive(Debug, Serialize)]
pub struct Func {
    pub name: String,
    pub signature: Signature,
    pub module: String,
    pub callable: FuncType,
}


// TODO: {127} Figure out how to the box<fn> types to clone/copy properly without
// the need for the none type.
#[derive(Serialize)]
pub enum FuncType {

    #[serde(skip_serializing)]
    Wrapper(Box<WrapperFn>),
    #[serde(skip_serializing)]
    MethodWrapper(ObjectRef, Box<WrapperFn>),
    Code(Code),
}


impl fmt::Debug for FuncType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let func_type = match self {
            &FuncType::Wrapper(ref func) => {
                format!("Wrapper(<function at {:?}>)", (func as *const _))
            },
            &FuncType::MethodWrapper(ref objref, ref func) => {
                format!(
                    "MethodWrapper(<method-wrapper of '{}' object at {:?}>)",
                    objref.debug_name(),
                    (func as *const _))
            }
            &FuncType::Code(_)  => format!("{:?}", self)
        };

        write!(f, "{}", func_type)
    }
}


#[derive(Debug)]
pub enum Iterator {
    Sequence {source: ObjectRef, idx_next: Cell<u64>},
    Empty,
}

impl Iterator {
    pub fn new(source: &ObjectRef) -> NativeResult<Self> {
        // TODO: {T101} Type assertions on new iterators or make it part of the `iter()`
        // builtin?
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


#[derive(Debug, Clone, Serialize)]
pub struct Code {
    pub co_name: String,
    pub co_names: Vec<String>,
    pub co_varnames: Vec<String>,
    pub co_code: Vec<Instr>,
    pub co_consts: Vec<Code>,
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

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize)]
pub struct Block {

}


#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize)]
pub struct Frame {
    pub f_back: ObjectRef,
    pub f_code: ObjectRef,
    pub f_builtins: ObjectRef,
    #[serde(serialize_with = "serialize::integer")]
    pub f_lasti: Integer,
    pub blocks: VecDeque<Block>,
}


#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Signature {
    args: Box<[String]>,
    required_kwargs: Box<[String]>,
    // vargs = "*name"
    vargs: Option<String>,

    default_kwargs: std::collections::HashMap<String, ObjectRef>,
    // kwargs = "**name"
    kwargs: Option<String>,
}


pub trait SignatureBuilder {
    fn as_args(&self) -> Signature;
}

impl Signature {
    pub fn new(args: &[&str],
               required_kwargs: &[&str],
               vargs: Option<&str>,
               kwargs: Option<&str>) -> Self {
        Signature {
            args: args.iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            required_kwargs: required_kwargs.iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            vargs: vargs.map(|s| s.to_string()),
            default_kwargs: HashMap::new(),
            kwargs: kwargs.map(|s| s.to_string()),
        }
    }
    
    pub fn args(&self) -> &[String] {
        &(*self.args)
    }

    pub fn has_vargs(&self) -> Boolean {
        self.vargs.is_some()
    }

    pub fn has_kwargs(&self) -> Boolean {
        self.kwargs.is_some()
    }

    pub fn vargs(&self) -> Option<&str> {
        match self.vargs {
            Some(ref string) => Some(&string),
            Option::None => Option::None
        }
    }

    pub fn kwargs(&self) -> Option<&str> {
        match self.kwargs {
            Some(ref string) => Some(&string),
            Option::None => Option::None
        }
    }

    pub fn min_arg_count(&self) -> Integer {
        Integer::from(self.args.len() + self.required_kwargs.len())
    }

    pub fn max_arg_count(&self) -> Option<Integer> {
        if self.vargs.is_some() || self.kwargs.is_some() {
            return Option::None
        }

        Some(Integer::from(
                    self.args.len() +
                    self.required_kwargs.len() +
                    self.default_kwargs.len()))
    }

}

macro_rules! signature_impls {
  ($i:ty, $($N:expr)+) => {
    $(
        impl<'a> SignatureBuilder for [$i; $N] {
            fn as_args(&self) -> Signature {
                Signature::new(&self[..], &[], Option::None, Option::None)
            }
        }

        impl<'a> SignatureBuilder for &'a [$i; $N] {
            fn as_args(&self) -> Signature {
                Signature::new(&self[..], &[], Option::None, Option::None)
            }
        }
    )+
  };
}


signature_impls!(&'a str, 0 1 2 3 4 5 6);


impl<'a> SignatureBuilder for &'a [String] {
    fn as_args(&self) -> Signature {
        let arr = self.iter().map(String::as_str).collect::<Vec<&str>>();
        Signature::new(&arr[..], &[], Option::None, Option::None)
    }
}


impl SignatureBuilder for Vec<String> {
    fn as_args(&self) -> Signature {
        let arr: Vec<&str> = self.iter().map(String::as_str).collect::<Vec<&str>>();
        Signature::new(&arr[..], &[], Option::None, Option::None)
    }
}

// Compiler Types

#[derive(Debug, Clone, Serialize)]
pub struct Instr(pub OpCode, pub Option<Native>);

impl Instr {
    pub fn tuple(&self) -> (OpCode, Option<Native>) {
        (self.0.clone(), self.1.clone())
    }

    pub fn code(&self) -> OpCode {
        return self.0.clone()
    }

    pub fn value(&self) -> Option<Native> {
        return self.1.clone()
    }
}


#[derive(Debug, Clone, Serialize)]
pub enum Native {
    Str(String),
    Int(
        #[serde(serialize_with = "serialize::integer")]
        Integer
    ),
    Float(Float),
    Bool(Boolean),
    Complex(
        #[serde(with = "ComplexSerdeDef")]
        Complex
    ),
    Count(Count),
    Code(Code),
    List(List),
    None,
}



impl<'a> From<&'a OwnedTk> for Native {
    // TODO: {T96} Refactor to use stdlib traits From / TryFrom if possible
    // TODO: {T96} unwrap() can cause panics, make this able to return a result

    fn from(tk: &'a OwnedTk) -> Self {
        let parsed = String::from_utf8(tk.bytes().to_vec()).unwrap();
        let content = parsed.as_str();

        match (tk.id(), tk.tag()) {
            (Id::Name, _)     => Native::Str(parsed.clone()),
            (Id::String, _)         |
            (Id::RawString, _)      |
            (Id::FormatString, _)   |
            (Id::ByteString, _)     => {
                // TODO: {T96} This is a hack to get the " or ' off of quoted strings
                Native::Str(parsed[1..parsed.len()-1].to_string())
            },
            (Id::Number, Tag::N(Num::Int))   => Native::Int(Integer::from_str(&parsed).unwrap()),
            (Id::Number, Tag::N(Num::Binary))=> Native::Int(Integer::from_str_radix(&parsed[2..], 2).unwrap()),
            (Id::Number, Tag::N(Num::Octal)) => Native::Int(Integer::from_str_radix(&parsed[2..], 8).unwrap()),
            (Id::Number, Tag::N(Num::Hex))   => Native::Int(Integer::from_str_radix(&parsed[2..], 16).unwrap()),
            (Id::Number, Tag::N(Num::Float)) => Native::Float(content.parse::<f64>().unwrap()),
            (Id::Number, Tag::N(Num::Complex)) => {
                let real: Float = 0.0;
                let img: Float =  content[..content.len()-1].parse::<f64>().unwrap();
                Native::Complex(Complex::new(real, img))
            },
            (Id::True, _) => Native::Bool(true),
            (Id::False, _) => Native::Bool(false),
            (Id::None, _) => Native::None,
            _ => unimplemented!()
        }
    }
}

impl<'a> From<&'a str> for Native {
    fn from(s: &'a str) -> Self {
        Native::Str(s.to_string())
    }
}


// Serialization for native Rust and external types

// Serde calls this the definition of the remote type. It is just a copy of the
// remote type. The `remote` attribute gives the path to the actual type.
#[derive(Serialize)]
#[serde(remote = "Complex")]
struct ComplexSerdeDef {
    /// Real portion of the complex number
    pub re: Float,
    /// Imaginary portion of the complex number
    pub im: Float
}

pub mod serialize {
    use super::*;

    const JSON_MAX_INT_BITS_LOSSLESS: usize = 54;


    /// Serialize a native::Integer type as a string if it will fit in a JSON double
    /// otherwise a string.
    pub fn integer<S>(int: &Integer, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        if int.bits() > JSON_MAX_INT_BITS_LOSSLESS {
            serializer.serialize_str(&format!("{}", int))
        } else {
            serializer.serialize_i64(int.to_i64().unwrap())
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use rsnek_compile::fmt::json;

    #[test]
    fn value_serialization() {
        println!("{}", json(&Native::Int(
            Integer::from_str("12341234124312423143214132432145932958392853094543214324").unwrap())));
        println!("{}", json(&Native::Complex(Complex::new(234.345, 622.9900000000001))));
    }


}