use std::fmt;
use std::ops::{Add, Deref};
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use itertools::Itertools;
use num::{ToPrimitive, Zero};

use ::resource::strings;
use error::Error;
use result::{RuntimeResult, NativeResult};
use runtime::Runtime;
use traits::{BooleanProvider, IntegerProvider, StringProvider, IteratorProvider, DefaultTupleProvider, TupleProvider};
use object::{self, RtValue, typing};
use object::method::{self, Id, Length};
use object::selfref::{self, SelfRef};

use ::typedef::builtin::Builtin;
use ::typedef::native::{self, Tuple};
use ::object::RtObject;
use ::typedef::collection::sequence;


pub struct PyTupleType {
    pub empty: RtObject,
}


impl typing::BuiltinType for PyTupleType {
    type T = PyTuple;
    type V = native::Tuple;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> RtObject {
        PyTupleType::inject_selfref(PyTupleType::alloc(value))
    }

    fn init_type() -> Self {
        PyTupleType { empty: PyTupleType::inject_selfref(PyTupleType::alloc(native::Tuple::new())) }
    }

    fn inject_selfref(value: Self::T) -> RtObject {
        let objref = RtObject::new(Builtin::Tuple(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Tuple(ref tuple) => {
                tuple.rc.set(&objref.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        PyTuple {
            value: TupleValue(value),
            rc: selfref::RefCount::default(),
        }
    }
}


pub struct TupleValue(pub native::Tuple);
pub type PyTuple = RtValue<TupleValue>;


impl fmt::Display for PyTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tuple({:?})", self.value.0)
    }
}

impl fmt::Debug for PyTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tuple({:?})", self.value.0)
    }
}


impl object::PyAPI for PyTuple {}

impl method::Hashed for PyTuple {
    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_hash() {
            Ok(hashid) => Ok(rt.int(hashid)),
            Err(err) => Err(err),
        }
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
        if self.native_len().unwrap().is_zero() {
            let mut s = DefaultHasher::new();
            match self.rc.upgrade() {
                Ok(objref) => {
                    let boxed: &Box<Builtin> = objref.0.borrow();
                    boxed.native_id().hash(&mut s);
                    return Ok(s.finish());
                }
                Err(err) => return Err(err),
            }
        }

        self.value
            .0
            .iter()
            .map(|ref item| {
                     let boxed: &Box<Builtin> = item.0.borrow();
                     boxed.native_hash()
                 })
            .fold_results(0, Add::add)
    }
}

impl method::StringCast for PyTuple {
    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        let string = self.native_str()?;
        Ok(rt.str(string))
    }

    fn native_str(&self) -> NativeResult<native::String> {
        let elems = self.value.0.iter()
                .map(|ref item| {
                     let boxed: &Box<Builtin> = item.0.borrow();
                     boxed.native_str()
                 })
                .fold_results(
                    Vec::with_capacity(self.value.0.len()),
                    |mut acc, s| {acc.push(s); acc})?
                .join(", ");

        Ok(format!("({})", elems))
    }
}


impl method::Equal for PyTuple {
    fn op_eq(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let boxed: &Box<Builtin> = rhs.0.borrow();

        let truth = self.native_eq(boxed)?;
        Ok(rt.bool(truth))
    }

    fn native_eq(&self, rhs: &Builtin) -> NativeResult<native::Boolean> {
        match rhs {
            &Builtin::Tuple(ref other) => {
                let left = &self.value.0;
                let right = &other.value.0;
                Ok(sequence::equals(left, right))
            }
            _ => Ok(false)
        }
    }
}



impl method::BooleanCast for PyTuple {
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


impl method::Multiply for PyTuple {
    fn op_mul(&self, rt: &Runtime, rhs: &RtObject) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match builtin.deref() {
            &Builtin::Int(ref int) => {
                match int.value.0.to_usize() {
                    Some(int) if int <= 0   => Ok(rt.default_tuple()),
                    Some(int) if int == 1   => self.rc.upgrade(),
                    Some(int)               => {
                        let value = sequence::multiply::<Tuple>(&self.value.0, int);
                        Ok(rt.tuple(value))
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


impl method::Contains for PyTuple {
    fn op_contains(&self, rt: &Runtime, item: &RtObject) -> RuntimeResult {
        let boxed: &Box<Builtin> = item.0.borrow();
        let truth = self.native_contains(boxed)?;
        Ok(rt.bool(truth))
    }

    fn native_contains(&self, item: &Builtin) -> NativeResult<native::Boolean> {
        Ok(sequence::contains(&self.value.0, item))
    }
}

impl method::Iter for PyTuple {
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


impl method::Length for PyTuple {
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


impl method::GetItem for PyTuple {
    #[allow(unused_variables)]
    fn op_getitem(&self, rt: &Runtime, index: &RtObject) -> RuntimeResult {
        let boxed: &Box<Builtin> = index.0.borrow();
        self.native_getitem(boxed)
    }

    fn native_getitem(&self, index: &Builtin) -> RuntimeResult {
        match index {
            &Builtin::Int(ref int) => {
                sequence::get_index(&self.value.0, &int.value.0)
            }
            _ => Err(Error::typerr("list index was not int")),
        }
    }
}


method_not_implemented!(PyTuple,
    AbsValue   Add   AddItem   Append   Await   BitwiseAnd   
    BitwiseOr   BytesCast   Call   Clear   Close   ComplexCast   
    Count   DelAttr   Delete   DeleteItem   DescriptorGet   DescriptorSet   
    DescriptorSetName   Discard   DivMod   Enter   Exit   Extend   
    FloatCast   FloorDivision   Get   GetAttr   GetAttribute   GreaterOrEqual   
    GreaterThan   Index   Init   InPlaceAdd   InPlaceBitwiseAnd   InPlaceBitwiseOr   
    InPlaceDivMod   InPlaceFloorDivision   InPlaceLeftShift   InPlaceMatrixMultiply
    InPlaceModulus   InPlaceMultiply InPlacePow   InPlaceRightShift   InPlaceSubtract
    InPlaceTrueDivision   InPlaceXOr   IntegerCast InvertValue   IsDisjoint   Items   Keys
    LeftShift   LengthHint LessOrEqual   LessThan   MatrixMultiply   Modulus   NegateValue   New
    Next   NotEqual   Pop   PopItem   PositiveValue   Pow   ReflectedAdd   ReflectedBitwiseAnd
    ReflectedBitwiseOr   ReflectedDivMod   ReflectedFloorDivision   ReflectedLeftShift
    ReflectedMatrixMultiply   ReflectedModulus   ReflectedMultiply   ReflectedPow
    ReflectedRightShift   ReflectedSubtract ReflectedTrueDivision   ReflectedXOr   Remove
    Reversed   RightShift   Rounding Send   SetAttr   SetDefault   SetItem   StringFormat
    StringRepresentation Subtract   Throw   TrueDivision   Update   Values   XOr
);


#[cfg(test)]
mod tests {
    use traits::{TupleProvider, BooleanProvider};
    use object::method::*;
    use super::*;

    fn setup_test() -> (Runtime) {
        Runtime::new()
    }

    #[test]
    fn is_() {
        let rt = setup_test();
        let tuple = rt.tuple(native::None());
        let tuple2 = tuple.clone();
        let tuple3 = rt.tuple(vec![rt.tuple(native::None())]);

        let boxed: &Box<Builtin> = tuple.0.borrow();

        let result = boxed.op_is(&rt, &tuple2).unwrap();
        assert_eq!(result, rt.bool(true));

        let result = boxed.op_is(&rt, &tuple3).unwrap();
        assert_eq!(result, rt.bool(false));
    }

    mod __hash__ {
        use traits::{StringProvider, IntegerProvider, DictProvider};
        use super::*;

        #[test]
        fn empty_stable() {
            let rt = setup_test();
            let tuple = rt.tuple(native::None());
            let tuple2 = tuple.clone();

            let boxed: &Box<Builtin> = tuple.0.borrow();
            let r1 = boxed.op_hash(&rt).unwrap();
            let boxed: &Box<Builtin> = tuple2.0.borrow();
            let r2 = boxed.op_hash(&rt).unwrap();

            assert_eq!(r1, r2);
        }

        #[test]
        fn hashable_items() {
            let rt = setup_test();
            let empty = rt.tuple(native::None());

            let tuple = rt.tuple(vec![rt.int(1), rt.int(2), rt.str("3")]);
            let tuple2 = rt.tuple(vec![rt.int(1), rt.int(2), rt.str("3")]);

            let boxed: &Box<Builtin> = tuple.0.borrow();
            let r1 = boxed.op_hash(&rt).unwrap();
            let boxed: &Box<Builtin> = tuple2.0.borrow();
            let r2 = boxed.op_hash(&rt).unwrap();
            let boxed: &Box<Builtin> = empty.0.borrow();
            let r3 = boxed.op_hash(&rt).unwrap();

            assert_eq!(r1, r2);
            assert!(r1 != r3);
        }

        #[test]
        #[should_panic]
        fn unhashable_items_causes_error() {
            let rt = setup_test();

            let tuple = rt.tuple(vec![rt.dict(native::None())]);
            let boxed: &Box<Builtin> = tuple.0.borrow();
            boxed.op_hash(&rt).unwrap();
        }
    }
}
