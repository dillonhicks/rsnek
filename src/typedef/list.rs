use std;
use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::ops::DerefMut;
use std::fmt;
use std::ops::Deref;
use std::rc::{Weak, Rc};

use num::{BigInt, FromPrimitive};

use result::RuntimeResult;
use runtime::Runtime;
use error::{Error, ErrorType};

use super::objectref;
use super::objectref::{ObjectRef, WeakObjectRef};
use super::builtin::Builtin;
use super::float::FloatObject;


#[derive(Clone)]
pub struct List(RefCell<Vec<ObjectRef>>);


#[derive(Clone)]
pub struct ListObject {
    pub value: List,
}


impl objectref::ObjectBinaryOperations for ListObject {
    fn add(&self, runtime: &mut Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let borrowed: &RefCell<Builtin> = rhs.0.borrow();
        match borrowed.borrow_mut().deref() {
            &Builtin::List(ref obj) => {
                // TODO: Modify the new to allow runtime to give weakrefs back to self
                let lhs_cell: Ref<Vec<ObjectRef>> = self.value.0.borrow();
                let lhs_borrow: &Vec<ObjectRef> = lhs_cell.borrow().deref();

                // Inc refcounts for the objects transferred over to self's List
                let rhs_borrow = obj.value.0.borrow();

                let mut new_list: Vec<ObjectRef> = Vec::with_capacity(lhs_borrow.len() + rhs_borrow.len());
                lhs_borrow.iter().map(|objref| new_list.push(objref.clone()));
                rhs_borrow.iter().map(|objref| new_list.push(objref.clone()));

                /// DUMB DUMB DUMB THIS IS A COPY AND NOT THE REF TO THE ORIGINAL LIST!!!
                runtime.alloc(ListObject::new(&new_list).as_builtin().as_object_ref())
            },
            _ => Err(Error(ErrorType::Type, "TypeError cannot add to List"))
        }
    }

    fn subtract(&self, _: &mut Runtime, _: &ObjectRef) -> RuntimeResult {
        unimplemented!()
    }
}

impl objectref::TypeInfo for ListObject {}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0.borrow())
    }
}

impl fmt::Display for ListObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Debug for ListObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}


impl List {
    fn new(vector: Vec<ObjectRef>) -> List {
        List(RefCell::new(vector))
    }
}


impl ListObject {
    pub fn new(value: &Vec<ObjectRef>) -> ListObject {
        let tuple = ListObject {
            value: List::new(value.clone()),
        };

        return tuple
    }

    pub fn as_builtin(self) -> Builtin {
        return Builtin::List(self)
    }
}

impl objectref::ToRtWrapperType<Builtin> for ListObject {
    #[inline]
    fn to(self) -> Builtin {
        return Builtin::List(self)
    }
}

impl objectref::ToRtWrapperType<ObjectRef> for ListObject {
    #[inline]
    fn to(self) -> ObjectRef {
        ObjectRef::new(self.to())
    }
}


impl objectref::RtObject for ListObject {}

use object;

impl object::api::Identifiable for ListObject {}

impl object::api::Hashable for ListObject {}