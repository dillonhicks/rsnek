//! Type aliases and rust defined types used to back the higher level objects
//!
//! The well known primitive types that have a fairly direct
//! 1:1 representation to rusts primitive types. The 'native api',
//! along with not using the reference counting wrappers, will
//! should return these types directly.
use std;
use std::fmt;
use std::cell::Cell;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

use num;
#[allow(unused_imports)]
use num::{ToPrimitive};
use num::Num as NumTrait;
use serde::ser::{Serializer};

use python_ast::{Id, Tag, Num, OwnedTk};

use ::api::result::{ObjectResult, RtResult, Error};
use ::api::RtObject;
use ::modules::builtins::Type as BuiltinType;
use ::objects::collection::sequence::is_sequence;
use ::objects;
use ::runtime::{OpCode, Runtime};

/// The representation of the Id of an object as the cast of its memory address to
/// the platform size. Note that this mimics the CPython way and may change. The only
/// guarantee is that Object ids must be unique for the life of an object. This is an
/// implementation detail and is subject to change.
pub type ObjectId = usize;

/// Hashes are currently computed using the rust std machinery that computes hashes
/// as u64. This is considered an implementation detail and is subject to change.
pub type HashId = u64;

/// All integer values are aliased to use `BigInt` by default
/// to support python's idea of unbounded integer types.
pub type Integer = num::BigInt;

/// Not exposed as type per se. However, the maximum value of `Count` (`usize`) is
/// an upper limit for operations such as vector indexing and integer exponentiation.
pub type Count = usize;

/// Floats are doubles (`f64`) no surprise there, which is the same as CPython.
pub type Float = f64;

/// Booleans are `bool` because why do something special like make them an int?
pub type Boolean = bool;


/// Complex is represented as
pub type Complex = num::complex::Complex<Float>;

/// Used the own String type for most string value representations
/// in order to prevent descending into reference lifetime hell.
pub type String = std::string::String;

pub type Byte = u8;
pub type Bytes = Vec<Byte>;

/// None is a alias to the unit struct.
pub struct None();

/// `List` is backed by a `Vec` type which is an array list.
pub type List = Vec<RtObject>;

/// List `List`, `Tuple` is backed by a `Vec` type which is an array list.
/// The type alias does not prevent resizing that is done where the tuple is used
/// since rust has a strong idea about interior mutability.
pub type Tuple = Vec<RtObject>;


/// Necessary to hold the computed value of the hash since RtObject cannot call
/// `op_hash` without a reference to the `Runtime`. So the `DictKey::hash` should
/// should be the value returned from `op_hash` or `native_hash`.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct DictKey(HashId, RtObject);

impl DictKey {
    pub fn new(id: HashId, object: &RtObject) -> Self {
        DictKey(id, object.clone())
    }

    pub fn hash(&self) -> HashId {
        self.0
    }

    pub fn value(&self) -> RtObject {
        self.1.clone()
    }
}


/// Dictionaries use the standard rust HashMap from collections that map
/// RtObject => RtObject. They are keyed using the `DictKey` instead in order
/// to store the hash with the object since the runtime is needed to compute
/// the hash.
pub type Dict = std::collections::HashMap<DictKey, RtObject>;

/// See: `DictKey`
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct SetElement(pub HashId, pub RtObject);

/// Just a hash set of RtObject.
pub type Set = std::collections::HashSet<SetElement>;

/// Defines a function ptr to a function with a python like signature of
/// `fn (rt: &Runtime, args: &RtObject, stargs: &RtObject, kwargs: &RtObject) -> ObjectResult`
///
/// Especially useful for creating method wrappers or exposing rust closures as python objects.
pub type WrapperFn = Fn(&Runtime, &RtObject, &RtObject, &RtObject) -> ObjectResult;


/// Struct defining the data needed for a Python function including the name, signature, module,
/// and callable. Note that `FuncType` allows for both native functions and a bytecode object
/// to be stored in the same `Func` struct.
#[derive(Debug, Serialize)]
pub struct Func {
    pub name: String,
    pub signature: Signature,
    pub module: String,
    pub callable: FuncType,
}

// TODO: {127} Figure out how to the box<fn> types to clone/copy properly
#[derive(Serialize)]
pub enum FuncType {

    #[serde(skip_serializing)]
    Wrapper(Box<WrapperFn>),
    #[serde(skip_serializing)]
    MethodWrapper(RtObject, Box<WrapperFn>),
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


/// Wrapper to hold iterators that come from difference sources.
/// Currently, the only supported iterators are `Empty` and well known
/// `BuiltinType` variants.
#[derive(Debug)]
pub enum Iterator {
    Sequence {source: RtObject, idx_next: Cell<Count>},
    Empty,
}

impl Iterator {
    pub fn new(source: &RtObject) -> RtResult<Self> {
        if is_sequence(source) {
            return Ok(Iterator::Sequence {source: source.clone(), idx_next: Cell::new(0)})
        }

        Err(Error::typerr(
            &format!("'{}' is not a sequence", source.debug_name())))
    }
}

/// Work in progress to define the properties required to a generic python defined
/// class object.
#[derive(Debug)]
pub struct Object {
    pub class: RtObject,
    pub dict: RtObject,
    pub bases: RtObject,
}

/// WIP
#[allow(dead_code)]
pub struct Module {
    pub name: RtObject,

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

/// Defines the bytecode object.
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


/// Placeholder for try/catch block accounting
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize)]
pub struct Block {}


#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize)]
pub struct Frame {
    pub f_back: RtObject,
    pub f_code: RtObject,
    pub f_builtins: RtObject,
    #[serde(serialize_with = "serialize::integer")]
    pub f_lasti: Integer,
    pub blocks: VecDeque<Block>,
}


/// Represents the canonical python function signature of
/// `args`, `*args`, `kw_only_args`, and `**kwargs`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Signature {
    /// Good ol' "business in the front" positional named arguments
    args: Box<[String]>,

    /// `def(*, named,...)` required keyword only arguments
    required_kwargs: Box<[String]>,

    /// vargs = "*name"
    vargs: Option<String>,

    /// name="value"
    default_kwargs: std::collections::HashMap<String, RtObject>,

    /// and the "party in the back" `**kwargs` arguments.
    kwargs: Option<String>,
}


/// Trait to allow conversions of any implementer into a `Signature`
/// type. Primary use case is `[String]` arrays into a signature of
/// of named positional arguments.
pub trait SignatureBuilder {
    fn as_args(&self) -> Signature;
}


impl Signature {
    /// By default, creating a sigunature
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

    /// Return the variable `Some(name)` for `*kwargs` if present, otherwise `None`
    pub fn vargs(&self) -> Option<&str> {
        match self.vargs {
            Some(ref string) => Some(&string),
            Option::None => Option::None
        }
    }

    /// Return the variable `Some(name)` for `**kwargs` if present, otherwise `None`
    pub fn kwargs(&self) -> Option<&str> {
        match self.kwargs {
            Some(ref string) => Some(&string),
            Option::None => Option::None
        }
    }

    /// Return the minimum number of arguments allowed by this `Signature`.
    pub fn min_arg_count(&self) -> Integer {
        Integer::from(self.args.len() + self.required_kwargs.len())
    }

    /// Return the maximum number of arguments allowed by the `Signature` as
    /// `Some(count)`. In the variadic case where `*args` or `**args` is defined
    /// return `None` since there is no practical upper limit.
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


impl Default for Signature {
    /// Create an empty signature
    fn default() -> Signature {
        Signature::new(&[][..], &[], Option::None, Option::None)
    }
}


macro_rules! signature_impls {
  ($i:ty, $($N:expr)+) => {
    $(
        impl<'a> SignatureBuilder for [$i; $N] {
            /// Create a `Signature` of named positional arguments from the array
            fn as_args(&self) -> Signature {
                Signature::new(&self[..], &[], Option::None, Option::None)
            }
        }

        impl<'a> SignatureBuilder for &'a [$i; $N] {
            /// Create a `Signature` of named positional arguments from the array reference
            fn as_args(&self) -> Signature {
                Signature::new(&self[..], &[], Option::None, Option::None)
            }
        }
    )+
  };
}


signature_impls!(&'a str, 0 1 2 3 4 5 6);


impl<'a> SignatureBuilder for &'a [String] {
    /// Create a `Signature` of named positional arguments from a slice of strings.
    fn as_args(&self) -> Signature {
        let arr = self.iter().map(String::as_str).collect::<Vec<&str>>();
        Signature::new(&arr[..], &[], Option::None, Option::None)
    }
}


impl SignatureBuilder for Vec<String> {
    /// Create a `Signature` of named positional arguments from a vector of strings.
    fn as_args(&self) -> Signature {
        let arr: Vec<&str> = self.iter().map(String::as_str).collect::<Vec<&str>>();
        Signature::new(&arr[..], &[], Option::None, Option::None)
    }
}

/// Instruction type used by the compiler and interpreter
#[derive(Debug, Clone, Serialize)]
pub struct Instr(pub OpCode, pub Option<Native>);


impl Instr {
    pub fn to_tuple(&self) -> (OpCode, Option<Native>) {
        (self.0.clone(), self.1.clone())
    }

    pub fn code(&self) -> OpCode {
        return self.0.clone()
    }

    pub fn value(&self) -> Option<Native> {
        return self.1.clone()
    }
}


/// Enum of well known primitive types similar to `modules::builtins::type::Type`.
/// for uses where appropriate such as return values of native api methods,
/// the compiler, etc.
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
    /// Convert an `&OwnedTk` into a `Native` type by token id.
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
    /// Convert a `str` into `Native::Str` as s convenience.
    fn from(s: &'a str) -> Self {
        Native::Str(s.to_string())
    }
}


/// Serde calls this the definition of the remote type. It is just a copy of the
/// remote type. The `remote` attribute gives the path to the actual type.
#[derive(Serialize)]
#[serde(remote = "Complex")]
#[doc(hidden)]
struct ComplexSerdeDef {
    /// Real portion of the complex number
    pub re: Float,
    /// Imaginary portion of the complex number
    pub im: Float
}


pub mod serialize {
    use super::*;

    /// The epsilon for doubles (the only JS number type)
    /// becomes nonzero after 2**54 and integer values
    /// start to loose precision.
    const JSON_MAX_INT_BITS_LOSSLESS: usize = 54;


    /// Serialize a rs::Integer type as a string if it will fit in a JSON double
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
    use python_ast::fmt::json;

    #[test]
    fn value_serialization() {
        info!("{}", json(&Native::Int(
            Integer::from_str("12341234124312423143214132432145932958392853094543214324").unwrap())));
        info!("{}", json(&Native::Complex(Complex::new(234.345, 622.9900000000001))));
    }


}