use std::borrow::Borrow;
use std::hash::{Hash, Hasher, SipHasher};

use num::FromPrimitive;

use error::Error;
use runtime::Runtime;
use result::{RuntimeResult, NativeResult};
use typedef::builtin::Builtin;
use typedef::native;
use typedef::objectref::{ObjectRef, ToRtWrapperType};
use typedef::integer::IntegerObject;

use object::method;
use object::identity::DefaultIdentity;


pub trait DefaultHashed: DefaultIdentity + method::Hashed {
    // Called by built-in function hash() and for operations on members of hashed collections including
    // set, frozenset, and dict. __hash__() should return an integer. The only required property is
    // that objects which compare equal have the same hash value; it is advised to mix together
    // the hash values of the components of the object that also play a part in comparison
    // of objects by packing them into a tuple and hashing the tuple. Example:
    // api_method!(unary, self, __hash__, Hashable, op_hash, native_hash);
    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        match DefaultHashed::native_hash(self) {
            Ok(value) => rt.alloc(ObjectRef::new(Builtin::Integer(IntegerObject::new_u64(value)))),
            Err(err) => Err(err),
        }
    }

    /// Default implementation of the native hash is to
    /// use the ptr identity and hash that.
    /// Numerical types especially should override
    fn native_hash(&self) -> NativeResult<native::HashId> {
        let mut s = SipHasher::new();
        DefaultIdentity::native_id(self).hash(&mut s);
        Ok(s.finish())
    }
}
