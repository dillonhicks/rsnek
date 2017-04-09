use std::borrow::Borrow;

use error::Error;
use runtime::Runtime;
use result::{RuntimeResult, NativeResult};
use typedef::builtin::Builtin;
use typedef::native;
use typedef::objectref::ObjectRef;
use typedef::dictionary::PyDict;

use object::method;
use object::method::Hashed;
use object::method::GetItem;
use object::selfref::SelfRef;


pub trait HasDict {
    fn get_dict(&self) -> &PyDict;
}


pub trait DefaultGetAttr: method::GetAttr + HasDict {
    // TODO: Need to search the base classes dicts as well, maybe need MRO
    #[allow(unused_variables)]
    fn op_getattr(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        let boxed: &Box<Builtin> = name.0.borrow();
        DefaultGetAttr::native_getattr(self, &boxed)
    }

    fn native_getattr(&self, name: &Builtin) -> NativeResult<ObjectRef> {
        match name {
            &Builtin::Str(ref string) => {
                let stringref = match string.rc.upgrade() {
                    Ok(objref) => objref,
                    Err(err) => return Err(err),
                };

                let dict: &PyDict = self.get_dict();
                let key = native::DictKey(string.native_hash().unwrap(), stringref);
                match dict.native_getitem(&Builtin::DictKey(key)) {
                    // TODO: Fixme
                    Ok(builtin) => Ok(builtin.clone()),
                    Err(err) => Err(err),
                }
            }
            _ => Err(Error::typerr("getattr(): attribute name must be string")),
        }
    }
}