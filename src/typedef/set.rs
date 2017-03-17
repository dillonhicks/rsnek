use std::fmt;
use std::collections::HashSet;
use std::borrow::Borrow;
use result::RuntimeResult;
use runtime::Runtime;
use std::cell::RefCell;

use object;
use typedef::builtin::Builtin;
use object::api::Hashable;
use super::objectref::{self, ObjectRef};
use super::builtin;
use super::native::{HashId};


#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct SetElement(HashId, ObjectRef);
pub type Set = HashSet<SetElement>;


#[derive(Clone, Debug)]
pub struct SetObject {
    value: RefCell<Set>,
}


impl SetObject {
    #[inline]
    pub fn new() -> SetObject {
        SetObject { value: RefCell::new(Set::new()) }
    }
}


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///     RtObject Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl objectref::RtObject for SetObject {}
impl objectref::TypeInfo for SetObject {}
impl object::api::Identifiable for SetObject {}
impl object::api::Hashable for SetObject {}

impl objectref::ToRtWrapperType<builtin::Builtin> for SetObject {
    fn to(self) -> builtin::Builtin {
        builtin::Builtin::Set(self)
    }
}

impl objectref::ToRtWrapperType<ObjectRef> for SetObject {
    fn to(self) -> ObjectRef {
        ObjectRef::new(builtin::Builtin::Set(self))
    }
}

impl objectref::ObjectBinaryOperations for SetObject {
    fn add(&self, rt: &mut Runtime, item: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = item.0.borrow();
        match builtin.native_hash() {
            Ok(hash_id) => {
                self.value.borrow_mut().insert(SetElement(hash_id, item.clone()));
                Ok(rt.None())
            },
            // TODO: When objects are around we will need to match
            // against builtin enum variants.
            Err(err) => Err(err)
        }
    }

    fn subtract(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }
}

/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///        stdlib Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl fmt::Display for SetObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}


/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///        Tests
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

#[cfg(test)]
mod tests {

    use std;
    use std::ops::Deref;
    use typedef::objectref::{self, ObjectBinaryOperations, ObjectRef};

    use runtime::{Runtime, DEFAULT_HEAP_CAPACITY};
    use typedef::integer;
    use typedef::builtin::Builtin;
    use typedef::integer::{Integer, IntegerObject};
    use typedef::float::FloatObject;
    use typedef::string::StringObject;
    use typedef::tuple::TupleObject;
    use typedef::list::ListObject;
    use typedef::boolean::{SINGLETON_FALSE_BUILTIN, SINGLETON_TRUE_BUILTIN};
    use typedef::objectref::ToRtWrapperType;

    use super::SetObject;

    use num::ToPrimitive;
    use std::cmp::PartialEq;
    use object::api::Identifiable;
    use std::borrow::Borrow;


    #[test]
    fn test_add_item_to_set() {
        let mut rt = Runtime::new(None);

        let mut t1: Vec<ObjectRef> = 
                    vec![IntegerObject::new_i64(0).to(),
                         IntegerObject::new_i64(0).to(),
                         IntegerObject::new_i64(1).to(),
                         IntegerObject::new_i64(1).to(),
                         IntegerObject::new_i64(2).to(),
                         IntegerObject::new_i64(2).to()];
        
        t1 = t1.iter().map(|objref| rt.alloc(objref.clone()).unwrap()).collect();
        

        let set = rt.alloc(ObjectRef::new(Builtin::Set(SetObject::new()))).unwrap();
        let set_bi: &Box<Builtin> = set.0.borrow();

        for obj in &t1 {
            set_bi.add(&mut rt, &obj).unwrap();
        }

        println!("{:?}", set_bi)
    }
}
