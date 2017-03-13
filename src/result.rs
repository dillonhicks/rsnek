use std;
use std::result;
use std::fmt::{Debug,Formatter};
use std::rc::Rc;
use std::cell::RefCell;

use error::{Error};

use typedef::object::ObjectRef;
use typedef::builtin::Builtin;


pub type RuntimeResult = result::Result<ObjectRef, Error>;
