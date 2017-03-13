use std;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::borrow::Borrow;
use std::fmt::Display;
use std::ops::Deref;

use runtime::Runtime;
use result::RuntimeResult;

use super::builtin;
use super::builtin::Builtin;
use super::integer::IntegerObject;
use super::float::FloatObject;


type _ObjectRef = Rc<RefCell<Builtin>>;
pub struct ObjectRef(pub _ObjectRef);

type RefCount = Weak<RefCell<Builtin>>;


impl ObjectRef {
    #[inline]
    pub fn new(value: Builtin) -> ObjectRef{
        ObjectRef(Rc::new(RefCell::new(value)))
    }

}


impl Clone for ObjectRef {
    fn clone(&self) -> Self {
        ObjectRef((self.0).clone())
    }

}

impl Borrow<RefCell<Builtin>> for ObjectRef {
    fn borrow(&self) -> &RefCell<Builtin> {
        self.0.borrow()
    }
}

impl std::fmt::Display for ObjectRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let borrowed: &RefCell<Builtin> = (self.0.borrow());

        match borrowed.borrow_mut().deref() {
            &Builtin::Integer(ref obj) => write!(f, "<Integer({}): {:?}>", obj, obj as *const IntegerObject),
            &Builtin::Float(ref obj) => write!(f, "<Float({}) {:?}>", obj, obj as *const FloatObject),
            &Builtin::String(ref obj) => write!(f, "<String({}) {:?}>", obj, obj as *const StringObject),
            &Builtin::Tuple(ref obj) => write!(f, "<Tuple({}) {:?}>", obj, obj as *const TupleObject),
            other => write!(f, "<{:?} {:?}>", other, other as *const _)
        }
    }
}

impl std::fmt::Debug for ObjectRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let borrowed: &RefCell<Builtin> = (self.0.borrow());

        match borrowed.borrow_mut().deref() {
            &Builtin::Integer(ref obj) => write!(f, "<Integer({}): {:?}>", obj, obj as *const IntegerObject),
            &Builtin::Float(ref obj) => write!(f, "<Float({}) {:?}>", obj, obj as *const FloatObject),
            &Builtin::String(ref obj) => write!(f, "<String({}) {:?}>", obj, obj as *const StringObject),
            &Builtin::Tuple(ref obj) => write!(f, "<Tuple({}) {:?}>", obj, obj as *const TupleObject),
            other => write!(f, "<{:?} {:?}>", other, other as *const _)
        }
    }
}

pub trait ObjectMethods {

    //    binaryfunc nb_add;
    //    binaryfunc nb_subtract;
    //    binaryfunc nb_multiply;
    //    binaryfunc nb_remainder;
    //    binaryfunc nb_divmod;
    //    ternaryfunc nb_power;
    //    unaryfunc nb_negative;
    //    unaryfunc nb_positive;
    //    unaryfunc nb_absolute;
    fn add(&self, &mut Runtime, &ObjectRef) -> RuntimeResult;
//    fn subtract(&self, &Object) -> Result<&Object>;
//    fn multiply(&self, &Object) -> Result<&Object>;
//    fn remainder(&self, &Object) -> Result<&Object>;
//    fn divmod(&self, &Object) -> Result<&Object>;
//    fn power(&self, &Object, &Object) -> Result<&Object>;
//    fn negative(&self, &Object) -> Result<&Object>;
//    fn positive(&self) -> Result<&Object>;
//    fn absolute(&self) -> Result<&Object>;
//
//    //    inquiry nb_bool;
//    fn inquery(&self) -> Result<&Object>;
//
//    //    unaryfunc nb_invert;
//    //    binaryfunc nb_lshift;
//    //    binaryfunc nb_rshift;
//    //    binaryfunc nb_and;
//    //    binaryfunc nb_xor;
//    //    binaryfunc nb_or;
//    //    unaryfunc nb_int;
//    //    void *nb_reserved;  /* the slot formerly known as nb_long */
//    //    unaryfunc nb_float;
//    fn invert(&self) -> Result<&Object>;
//    fn lshift(&self, &Object) -> Result<&Object>;
//    fn rshift(&self, &Object) -> Result<&Object>;
//    fn and(&self, &Object) -> Result<&Object>;
//    fn xor(&self, &Object) -> Result<&Object>;
//    fn or(&self, &Object) -> Result<&Object>;
//    fn float(&self) -> Result<&Object>;
//
//    //    binaryfunc nb_inplace_add;
//    //    binaryfunc nb_inplace_subtract;
//    //    binaryfunc nb_inplace_multiply;
//    //    binaryfunc nb_inplace_remainder;
//    //    ternaryfunc nb_inplace_power;
//    //    binaryfunc nb_inplace_lshift;
//    //    binaryfunc nb_inplace_rshift;
//    //    binaryfunc nb_inplace_and;
//    //    binaryfunc nb_inplace_xor;
//    //    binaryfunc nb_inplace_or;
//
//    fn inplace_add(&self, &Object) -> Result<&Object>;
//    fn inplace_subtract(&self, &Object) -> Result<&Object>;
//    fn inplace_multiply(&self, &Object) -> Result<&Object>;
//    fn inplace_remainder(&self, &Object) -> Result<&Object>;
//    fn inplace_power(&self, &Object, &Object) -> Result<&Object>;
//    fn inplace_invert(&self, &Object) -> Result<&Object>;
//    fn inplace_lshift(&self, &Object) -> Result<&Object>;
//    fn inplace_rshift(&self, &Object) -> Result<&Object>;
//    fn inplace_and(&self, &Object) -> Result<&Object>;
//    fn inplace_xor(&self, &Object) -> Result<&Object>;
//    fn inplace_or(&self, &Object) -> Result<&Object>;
//
//
//    //    binaryfunc nb_floor_divide;
//    //    binaryfunc nb_true_divide;
//    //    binaryfunc nb_inplace_floor_divide;
//    //    binaryfunc nb_inplace_true_divide;
//
//    fn floor_divide(&self, &Object) -> Result<&Object>;
//    fn true_divide(&self, &Object) -> Result<&Object>;
//    fn inplace_floor_divide(&self, &Object) -> Result<&Object>;
//    fn inplace_true_divide(&self, &Object) -> Result<&Object>;
//
//    //    unaryfunc nb_index;
//    fn index(&self, &Object) -> Result<&Object>;
//
//    //    binaryfunc nb_matrix_multiply;
//    //    binaryfunc nb_inplace_matrix_multiply;
//    fn matrix_multiply(&self, &Object) -> Result<&Object>;
//    fn inplace_matrix_multiply(&self, &Object) -> Result<&Object>;
}


pub trait TypeInfo {

}


pub trait Object: ObjectMethods + TypeInfo + Display where Self: std::marker::Sized {

}

//
//
//trait TypeInfo {
//
//}