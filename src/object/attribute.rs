use std::borrow::Borrow;
use num::FromPrimitive;

use error::{Error, ErrorType};
use runtime::Runtime;
use result::{RuntimeResult, NativeResult};
use typedef::builtin::Builtin;
use typedef::native;
use typedef::objectref::ObjectRef;
use typedef::dictionary::DictionaryObject;

use object::method;
use object::method::Hashed;
use object::selfref::SelfRef;
use object::model::PyBehavior;


pub trait InternalDictAccess {
    fn intern_dict(&self) -> &DictionaryObject;
}


pub trait DefaultGetAttr: method::GetAttr + InternalDictAccess {
    // TODO: Need to search the base classes dicts as well, maybe need MRO
    fn op_getattr(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        let boxed: &Box<Builtin> = name.0.borrow();
        DefaultGetAttr::native_getattr(self, &boxed)
    }

    fn native_getattr(&self, name: &Builtin) -> NativeResult<ObjectRef> {
        match name {
            &Builtin::Str(ref string) => {
                let stringref = match string.rc.upgrade() {
                    Ok(objref) => objref,
                    Err(err) => return Err(err)
                };

                let dict: &DictionaryObject = self.intern_dict();
                let key = native::Key(string.native_hash().unwrap(), stringref);
                match dict.native_getitem(&key) {
                    // TODO: Fixme
                    Ok(objref) => Ok(objref.clone()),
                    Err(Error(0: ErrorType::KeyError, 1: _)) => Err(Error::attribute()),
                    Err(err) => Err(err)
                }
            },
            _ => Err(Error::typerr("getattr(): attribute name must be string"))
        }
    }
}