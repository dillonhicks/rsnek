use result::Result;
use snektype;
use std;
use std::rc::Rc;
use snektype::BuiltinType;
use std::fmt::Display;

pub trait ObjectMethods<T: Object> {

    //    binaryfunc nb_add;
    //    binaryfunc nb_subtract;
    //    binaryfunc nb_multiply;
    //    binaryfunc nb_remainder;
    //    binaryfunc nb_divmod;
    //    ternaryfunc nb_power;
    //    unaryfunc nb_negative;
    //    unaryfunc nb_positive;
    //    unaryfunc nb_absolute;
    fn add(&self, Rc<T>) -> Result<Rc<BuiltinType>>;
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
    fn snek_type(&self) -> snektype::BuiltinType;
}


pub trait Object: ObjectMethods<Self> + TypeInfo + Display where Self: std::marker::Sized {

}

//
//
//trait TypeInfo {
//
//}