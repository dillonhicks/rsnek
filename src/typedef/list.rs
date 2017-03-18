use std;
use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, Ref, RefCell};
use std::ops::DerefMut;
use std::fmt;
use std::ops::Deref;
use std::rc::{Weak, Rc};
use std::marker::Copy;

use num::{BigInt, FromPrimitive};

use object;
use result::RuntimeResult;
use runtime::Runtime;
use error::{Error, ErrorType};
use typedef::objectref::ToRtWrapperType;
use typedef::native;

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


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    Struct Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

impl List {
    fn new(vector: Vec<ObjectRef>) -> List {
        List(RefCell::new(vector))
    }
}


impl ListObject {
    pub fn new(value: &Vec<ObjectRef>) -> ListObject {
        let tuple = ListObject {
            value: List::new(value.clone())
        };

        return tuple
    }

}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    Python Object Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl objectref::RtObject for ListObject {}
impl object::model::PyObject for ListObject {}
impl object::model::PyBehavior for ListObject {

    fn op_add(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match builtin.deref() {
            &Builtin::List(ref obj) => {
                // TODO: Modify the new to allow runtime to give weakrefs back to self
                let lhs_cell: Ref<Vec<ObjectRef>> = self.value.0.borrow();
                let lhs_borrow: &Vec<ObjectRef> = lhs_cell.borrow().deref();

                // Inc refcounts for the objects transferred over to self's List
                let rhs_borrow = obj.value.0.borrow();

                let mut new_list: Vec<ObjectRef> = Vec::with_capacity(lhs_borrow.len() + rhs_borrow.len());
                lhs_borrow.iter().map(|objref| new_list.push(objref.clone()));
                rhs_borrow.iter().map(|objref| new_list.push(objref.clone()));


                rt.find_object((&(*self) as *const _) as native::ObjectId).unwrap();

                /// DUMB DUMB DUMB THIS IS A COPY AND NOT THE REF TO THE ORIGINAL LIST!!!
                let l: ObjectRef = ListObject::new(&new_list).to();
                rt.alloc(l)
            },
            _ => Err(Error(ErrorType::Type, "TypeError cannot add to List"))
        }
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



// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    stdlib Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

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
