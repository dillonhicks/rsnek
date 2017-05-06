use std::fmt;
use std::ops::Deref;
use std::borrow::Borrow;

use itertools::Itertools;
use num::ToPrimitive;

use ::resource::strings;
use error::Error;
use result::{RuntimeResult, NativeResult};
use runtime::Runtime;
use traits::{BooleanProvider, IntegerProvider, StringProvider,
             IteratorProvider, DefaultListProvider, ListProvider};
use object::{RtValue, typing, PyAPI};
use object::method::{self, Equal};
use object::selfref::{self, SelfRef};

use ::typedef::collection::sequence;
use ::typedef::builtin::Builtin;
use ::typedef::native::{self, List};
use ::object::RtObject as ObjectRef;


pub struct PyListType {
    pub empty: ObjectRef,
}


impl typing::BuiltinType for PyListType {
    type T = PyList;
    type V = native::List;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        PyListType::inject_selfref(PyListType::alloc(value))
    }

    fn init_type() -> Self {
        PyListType { empty: PyListType::inject_selfref(PyListType::alloc(native::List::new())) }
    }

    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::List(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::List(ref list) => {
                list.rc.set(&objref.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        PyList {
            value: ListValue(value),
            rc: selfref::RefCount::default(),
        }
    }
}

pub struct ListValue(pub native::List);
pub type PyList = RtValue<ListValue>;


impl fmt::Display for PyList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List({:?})", self.value.0)
    }
}

impl fmt::Debug for PyList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List({:?})", self.value.0)
    }
}


impl PyAPI for PyList {}

impl method::StringCast for PyList {
    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        let s = self.native_str()?;
        Ok(rt.str(s))
    }

    fn native_str(&self) -> NativeResult<native::String> {

        let result = self.value.0.iter()
            .map(|ref item| {
                let boxed: &Box<Builtin> = item.0.borrow();
                boxed.native_str()
            })
            .fold_results(Vec::new(), |mut acc, s| {acc.push(s); acc});

        match result {
            Ok(s) => Ok(format!("[{}]", s.join(", "))),
            Err(err) => Err(err)
        }
    }
}


impl method::Equal for PyList {

    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let boxed: &Box<Builtin> = rhs.0.borrow();
        let truth = self.native_eq(boxed)?;
        Ok(rt.bool(truth))
    }

    fn native_eq(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        match rhs {
            &Builtin::List(ref other) => {
                let left = &self.value.0;
                let right = &other.value.0;
                Ok(sequence::equals(left, right))
            }
            _ => Ok(false)
        }
    }
}

impl method::NotEqual for PyList {
    fn op_ne(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let boxed: &Box<Builtin> = rhs.0.borrow();

        let truth = self.native_ne(boxed)?;
        Ok(rt.bool(truth))
    }

    fn native_ne(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        let truth = self.native_eq(&rhs)?;
        Ok(!truth)
    } 
    
}

impl method::BooleanCast for PyList {
    fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_bool() {
            Ok(bool) => Ok(rt.bool(bool)),
            Err(err) => Err(err)
        }
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
        Ok(!self.value.0.is_empty())
    }
}


impl method::Multiply for PyList {

    fn op_mul(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match builtin.deref() {
            &Builtin::Int(ref int) => {
                match int.value.0.to_usize() {
                    Some(int) if int <= 0   => Ok(rt.default_list()),
                    Some(int) if int == 1   => self.rc.upgrade(),
                    Some(int)               => {
                        let list = sequence::multiply::<List>(&self.value.0, int);
                        Ok(rt.list(list))
                    },
                    None                    => {
                        Err(Error::overflow(strings::ERROR_NATIVE_INT_OVERFLOW))
                    },
                }
            }
            other => Err(Error::typerr(
                &strings_error_bad_operand!("*", "tuple", other.debug_name())))
        }
    }
}


impl method::Contains for PyList {
    fn op_contains(&self, rt: &Runtime, item: &ObjectRef) -> RuntimeResult {
        let boxed: &Box<Builtin> = item.0.borrow();
        let truth = self.native_contains(boxed)?;
        Ok(rt.bool(truth))
    }

    fn native_contains(&self, item: &Builtin) -> NativeResult<native::Boolean> {
        Ok(sequence::contains(&self.value.0, item))
    }
}
impl method::Iter for PyList {
    fn op_iter(&self, rt: &Runtime) -> RuntimeResult {
        let iter = self.native_iter()?;
        Ok(rt.iter(iter))
    }

    fn native_iter(&self) -> NativeResult<native::Iterator> {
        match self.rc.upgrade() {
            Ok(selfref) => Ok(native::Iterator::new(&selfref)?),
            Err(err) => Err(err)
        }
    }

}

impl method::Length for PyList {
    fn op_len(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_len() {
            Ok(length) => Ok(rt.int(length)),
            Err(err) => Err(err),
        }
    }

    fn native_len(&self) -> NativeResult<native::Integer> {
        Ok(native::Integer::from(self.value.0.len()))
    }
}

impl method::GetItem for PyList {
    #[allow(unused_variables)]
    fn op_getitem(&self, rt: &Runtime, index: &ObjectRef) -> RuntimeResult {
        let boxed: &Box<Builtin> = index.0.borrow();
        self.native_getitem(boxed)
    }

    fn native_getitem(&self, index: &Builtin) -> RuntimeResult {
        match index {
            &Builtin::Int(ref int) => {
                sequence::get_index(&self.value.0, &int.value.0)
            }
            _ => Err(Error::typerr("list indices must be integers")),
        }
    }
}

method_not_implemented!(PyList,
    AbsValue   Add   AddItem   Append   
    Await   BitwiseAnd   BitwiseOr   BytesCast   
    Call   Clear   Close   ComplexCast   
    Count   DelAttr   Delete   DeleteItem   
    DescriptorGet   DescriptorSet   DescriptorSetName   Discard   
    DivMod   Enter   Exit   Extend   
    FloatCast   FloorDivision   Get   GetAttr   
    GetAttribute   GreaterOrEqual   GreaterThan   Hashed   
    Index   Init   InPlaceAdd   InPlaceBitwiseAnd   
    InPlaceBitwiseOr   InPlaceDivMod   InPlaceFloorDivision   InPlaceLeftShift   
    InPlaceMatrixMultiply   InPlaceModulus   InPlaceMultiply   InPlacePow   
    InPlaceRightShift   InPlaceSubtract   InPlaceTrueDivision   InPlaceXOr   
    IntegerCast   InvertValue   IsDisjoint   Items   
    Keys   LeftShift   LengthHint   LessOrEqual   
    LessThan   MatrixMultiply   Modulus   NegateValue   
    New   Next   Pop   PopItem   
    PositiveValue   Pow   ReflectedAdd   ReflectedBitwiseAnd   
    ReflectedBitwiseOr   ReflectedDivMod   ReflectedFloorDivision   ReflectedLeftShift   
    ReflectedMatrixMultiply   ReflectedModulus   ReflectedMultiply   ReflectedPow   
    ReflectedRightShift   ReflectedSubtract   ReflectedTrueDivision   ReflectedXOr   
    Remove   Reversed   RightShift   Rounding   
    Send   SetAttr   SetDefault   SetItem   
    StringFormat   StringRepresentation   Subtract   Throw   
    TrueDivision   Update   Values   XOr
);

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use num::Zero;

    use ::traits::{
        DefaultListProvider,
        NoneProvider,
        TupleProvider,
        FloatProvider
    };
    use ::object::method::{BooleanCast, GetItem, Multiply, NotEqual, Length, StringCast, Iter};
    use super::*;

    fn setup() -> (Runtime,) {
        (Runtime::new(),)
    }

    #[test]
    fn new_default() {
        let (rt,) = setup();
        rt.default_list();
    }
    
    #[test]
    fn __bool__() {
        let (rt,) = setup();

        // Empty
        let list = rt.default_list();

        let truth = list.op_bool(&rt).unwrap();
        assert_eq!(truth, rt.bool(false));
        let truth = list.native_bool().unwrap();
        assert_eq!(truth, false);

        // N Elements
        let list = rt.list(vec![rt.none(), rt.str("yup"), rt.float(1.324)]);

        let truth = list.op_bool(&rt).unwrap();
        assert_eq!(truth, rt.bool(true));
        let truth = list.native_bool().unwrap();
        assert_eq!(truth, true);
    }

    #[test]
    fn __eq__() {
        let (rt,) = setup();

        // Empty
        let list = rt.default_list();
        assert_eq!(list, list.clone());
        assert_eq!(list, rt.default_list());
        assert!(list != rt.list(vec![rt.int(1)]));

        // N Elements
        let list = rt.list(vec![rt.none(), rt.none(), rt.none()]);
        assert_eq!(list, list.clone());
        assert_eq!(list, rt.list(vec![rt.none(), rt.none(), rt.none()]));
        assert!(list != rt.list(vec![rt.int(1)]));
    }

    #[test]
    fn __ne__() {
        let (rt,) = setup();

        // Empty
        let list = rt.default_list();

        let truth = list.op_ne(&rt, &list.clone()).unwrap();
        assert_eq!(truth, rt.bool(false));
        let truth = list.op_ne(&rt, &rt.default_list()).unwrap();
        assert_eq!(truth, rt.bool(false));
        let truth = list.op_ne(&rt, &rt.list(vec![rt.int(1)])).unwrap();
        assert_eq!(truth, rt.bool(true));

        // N Elements
        let list = rt.list(vec![rt.int(1), rt.none(), rt.str("last")]);

        let truth = list.op_ne(&rt, &list.clone()).unwrap();
        assert_eq!(truth, rt.bool(false));
        let truth = list.op_ne(&rt, &rt.list(vec![rt.int(1), rt.none(), rt.str("last")])).unwrap();
        assert_eq!(truth, rt.bool(false));
        let truth = list.op_ne(&rt, &rt.list(vec![rt.str("first")])).unwrap();
        assert_eq!(truth, rt.bool(true));
    }

    #[test]
    fn __len__() {
        let (rt,) = setup();

        // Empty
        let list = rt.default_list();

        let len = list.op_len(&rt).unwrap();
        assert_eq!(len, rt.int(0));
        let len = list.native_len().unwrap();
        assert_eq!(len, native::Integer::zero());

        // N Elements
        let list = rt.list(vec![rt.none(), rt.none(), rt.none()]);

        let len = list.op_len(&rt).unwrap();
        assert_eq!(len, rt.int(3));
        let len = list.native_len().unwrap();
        assert_eq!(len, native::Integer::from(3));
    }

    #[test]
    fn __str__() {
        let (rt,) = setup();

        // Empty
        let list = rt.default_list();

        let s = list.op_str(&rt).unwrap();
        assert_eq!(s, rt.str("[]"));
        let s = list.native_str().unwrap();
        assert_eq!(&s, "[]");

        // N Elements
        let list = rt.list(vec![rt.none(), rt.bool(true), rt.bool(false), rt.int(1)]);

        let s = list.op_str(&rt).unwrap();
        assert_eq!(s, rt.str("[None, True, False, 1]"));
        let s = list.native_str().unwrap();
        assert_eq!(&s, "[None, True, False, 1]");
    }

    #[test]
    fn __getitem__() {
        let (rt,) = setup();

        // Empty
        let list = rt.default_list();

        let is_err = list.op_getitem(&rt, &rt.int(0)).is_err();
        assert_eq!(is_err, true);

        // N Elements
        let list = rt.list(vec![rt.int(1), rt.int(2), rt.str("three")]);

        let item = list.op_getitem(&rt, &rt.int(0)).unwrap();
        assert_eq!(item, rt.int(1));
        let item = list.op_getitem(&rt, &rt.int(1)).unwrap();
        assert_eq!(item, rt.int(2));
        let item = list.op_getitem(&rt, &rt.int(2)).unwrap();
        assert_eq!(item, rt.str("three"));

        // Out of bounds
        let is_err = list.op_getitem(&rt, &rt.int(3)).is_err();
        assert_eq!(is_err, true);

        // Negative indexing
        let item = list.op_getitem(&rt, &rt.int(-1)).unwrap();
        assert_eq!(item, rt.str("three"));

        // Out of bounds
        let is_err = list.op_getitem(&rt, &rt.int(-4)).is_err();
        assert_eq!(is_err, true);
    }
    
    #[test]
    fn __iter__() {
        let (rt,) = setup();

        // Empty
        let list = rt.default_list();

        let iter = list.op_iter(&rt).unwrap();
        assert_eq!(iter.count(), 0);

        // N Elements
        let list = rt.list(vec![
            rt.none(),
            rt.float(99433.000001),
            rt.str("asdf"),
            rt.tuple(vec![rt.default_list()])
        ]);

        let iter = list.op_iter(&rt).unwrap();
        assert_eq!(iter.count(), 4);
    }

    #[test]
    fn __mul__() {
        let (rt,) = setup();

        // Empty
        let list = rt.default_list();

        let new_list = list.op_mul(&rt, &rt.int(10)).unwrap();
        let len = new_list.op_len(&rt).unwrap();
        assert_eq!(len, rt.int(0));

        // N Elements
        let list = rt.list(vec![
            rt.none(),
            rt.float(99433.000001),
            rt.str("asdf"),
            rt.tuple(vec![rt.default_list()])
        ]);

        let new_list = list.op_mul(&rt, &rt.int(145)).unwrap();
        let len = new_list.op_len(&rt).unwrap();
        assert_eq!(len, rt.int(4 * 145));
    }

}
