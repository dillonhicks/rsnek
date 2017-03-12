use std::result;
use std::fmt::{Debug,Formatter};
use error::{Error};
use object::ObjectRef;
use std::rc::Rc;
use std::cell::RefCell;
use builtin::Builtin;
use std;

pub type RuntimeResult = result::Result<ObjectRef, Error>;
