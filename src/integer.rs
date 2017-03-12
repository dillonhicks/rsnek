extern crate num;

use self::num::BigInt;
use self::num::FromPrimitive;

use object;
use snektype::{BuiltinType, SnekInteger};
use result::Result;
use std::rc::{Weak,Rc};
use runtime::Runtime;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::fmt;

#[derive(Clone)]
pub struct IntegerObject {
    runtime: RefCell<Runtime>,
    value: SnekInteger
}


impl object::ObjectMethods<IntegerObject> for IntegerObject {
    fn add(&self, rhs: Rc<IntegerObject>) -> Result<Rc<BuiltinType>> {
        // If this fails the interpreter is fucked anyways because the runtime has been dealloc'd


        let borrowed: &IntegerObject = rhs.borrow();
        let new_value = &self.value + &borrowed.value;
        let integer = BuiltinType::Integer(IntegerObject {
            runtime: self.runtime.clone(),
            value: new_value,
        });


        self.runtime.borrow_mut().reserve(Rc::new(integer))

    }
}

impl object::TypeInfo for IntegerObject {
    fn snek_type(&self) -> BuiltinType {
        BuiltinType::Object
    }
}


impl fmt::Display for IntegerObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl IntegerObject {
    pub fn new(rt: RefCell<Runtime>, value: i64) -> IntegerObject {
        return IntegerObject {
            runtime: rt,
            value: BigInt::from_i64(value).unwrap()
        }
    }
}

impl object::Object for IntegerObject {
}