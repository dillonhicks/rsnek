use std;
use num;
use typedef;


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
pub type NoneValue = ();

//
// Collection Primitive Types
//
pub type List = Vec<typedef::objectref::ObjectRef>;
pub type Tuple = Vec<typedef::objectref::ObjectRef>;


#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct DictKey(pub HashId, pub typedef::objectref::ObjectRef);
//#[derive(Clone, Debug, Hash, Eq, PartialEq)]
//pub struct WeakKey(pub HashId, pub typedef::objectref::WeakObjectRef);

pub type Dict = std::collections::HashMap<DictKey, typedef::objectref::ObjectRef>;
/*struct {
    key_set: Set,
    mapping: std::collections::HashMap<Key, typedef::objectref::ObjectRef>
};
*/
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct SetElement(pub HashId, pub typedef::objectref::ObjectRef);
pub type Set = std::collections::HashSet<SetElement>;


pub type KWDictionary = std::collections::HashMap<String, typedef::objectref::ObjectRef>;


pub struct Type {
    name: String,
    dict: KWDictionary,
    bases: Tuple,
}
