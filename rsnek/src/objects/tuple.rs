//! PyTuple - statically sized, immutable, sequence of `RtObject` elements
//!
//! ```ignore
//! tuple()
//! (1,3,3,5)
//! ```
//!
use std::fmt;
use std::ops::{Add, Deref};
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use itertools::Itertools;
use num::{ToPrimitive, Zero};

use ::api::result::Error;
use ::api::method::{self, Id, Length, StringRepresentation};
use ::api::RtObject;
use ::api::selfref::{self, SelfRef};
use ::api::{self, RtValue, typing};
use ::resources::strings;
use ::api::result::{ObjectResult, RtResult};
use ::runtime::Runtime;
use ::runtime::traits::{BooleanProvider, IntegerProvider, StringProvider,
               IteratorProvider, DefaultTupleProvider, TupleProvider};
use ::modules::builtins::Type;
use ::objects::collection::sequence;
use ::system::primitives::{Tuple};
use ::system::primitives as rs;


pub struct PyTupleType {
    pub empty: RtObject,
}


impl typing::BuiltinType for PyTupleType {
    type T = PyTuple;
    type V = rs::Tuple;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> RtObject {
        PyTupleType::inject_selfref(PyTupleType::alloc(value))
    }

    fn init_type() -> Self {
        PyTupleType { empty: PyTupleType::inject_selfref(PyTupleType::alloc(rs::Tuple::new())) }
    }

    fn inject_selfref(value: Self::T) -> RtObject {
        let object = RtObject::new(Type::Tuple(value));
        let new = object.clone();

        match object.as_ref() {
            &Type::Tuple(ref tuple) => {
                tuple.rc.set(&object.clone());
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


pub struct TupleValue(pub rs::Tuple);
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


impl api::PyAPI for PyTuple {}

impl method::Hashed for PyTuple {
    fn op_hash(&self, rt: &Runtime) -> ObjectResult {
        let value = self.native_hash()?;
        Ok(rt.int(value))
    }

    fn native_hash(&self) -> RtResult<rs::HashId> {
        if self.native_len().unwrap().is_zero() {
            let mut s = DefaultHasher::new();
            let this_object = self.rc.upgrade()?;
            this_object.native_id().hash(&mut s);

            return Ok(s.finish());
        }

        self.value.0.iter()
            .map(RtObject::native_hash)
            .fold_results(0, Add::add)
    }
}

impl method::StringCast for PyTuple {
    fn op_str(&self, rt: &Runtime) -> ObjectResult {
        let string = self.native_str()?;
        Ok(rt.str(string))
    }

    fn native_str(&self) -> RtResult<rs::String> {
        let elems = self.value.0.iter()
                .map(RtObject::native_repr)
                .fold_results(
                    Vec::with_capacity(self.value.0.len()),
                    |mut acc, s| {acc.push(s); acc})
                ?.join(", ");

        Ok(format!("({})", elems))
    }
}


impl method::Equal for PyTuple {
    fn op_eq(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        let truth = self.native_eq(rhs.as_ref())?;
        Ok(rt.bool(truth))
    }

    fn native_eq(&self, rhs: &Type) -> RtResult<rs::Boolean> {
        match rhs {
            &Type::Tuple(ref other) => {
                let left = &self.value.0;
                let right = &other.value.0;
                Ok(sequence::equals(left, right))
            }
            _ => Ok(false)
        }
    }
}


impl method::BooleanCast for PyTuple {
    fn op_bool(&self, rt: &Runtime) -> ObjectResult {
        let truth = self.native_bool()?;
        Ok(rt.bool(truth))
    }

    fn native_bool(&self) -> RtResult<rs::Boolean> {
        Ok(!self.value.0.is_empty())
    }
}


impl method::Multiply for PyTuple {
    fn op_mul(&self, rt: &Runtime, rhs: &RtObject) -> ObjectResult {
        match rhs.as_ref() {
            &Type::Int(ref int) => {
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
    fn op_contains(&self, rt: &Runtime, item: &RtObject) -> ObjectResult {
        let truth = self.native_contains(item.as_ref())?;
        Ok(rt.bool(truth))
    }

    fn native_contains(&self, item: &Type) -> RtResult<rs::Boolean> {
        Ok(sequence::contains(&self.value.0, item))
    }
}

impl method::Iter for PyTuple {
    fn op_iter(&self, rt: &Runtime) -> ObjectResult {
        let iter = self.native_iter()?;
        Ok(rt.iter(iter))
    }

    fn native_iter(&self) -> RtResult<rs::Iterator> {
        let this_object = self.rc.upgrade()?;
        Ok(rs::Iterator::new(&this_object)?)
    }

}


impl method::Length for PyTuple {
    fn op_len(&self, rt: &Runtime) -> ObjectResult {
        let value = self.native_len()?;
        Ok(rt.int(value))
    }

    fn native_len(&self) -> RtResult<rs::Integer> {
        Ok(rs::Integer::from(self.value.0.len()))
    }
}


impl method::GetItem for PyTuple {
    #[allow(unused_variables)]
    #[inline(always)]
    fn op_getitem(&self, rt: &Runtime, index: &RtObject) -> ObjectResult {
        self.native_getitem(index.as_ref())
    }

    #[inline(always)]
    fn native_getitem(&self, index: &Type) -> ObjectResult {
        match index {
            &Type::Int(ref int) => {
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
    use ::runtime::traits::{TupleProvider, BooleanProvider};
    use api::method::*;
    use super::*;

    fn setup_test() -> (Runtime) {
        Runtime::new()
    }

    #[test]
    fn is_() {
        let rt = setup_test();
        let tuple = rt.tuple(rs::None());
        let tuple2 = tuple.clone();
        let tuple3 = rt.tuple(vec![rt.tuple(rs::None())]);

        let result = tuple.op_is(&rt, &tuple2).unwrap();
        assert_eq!(result, rt.bool(true));

        let result = tuple.op_is(&rt, &tuple3).unwrap();
        assert_eq!(result, rt.bool(false));
    }

    mod __hash__ {
        use ::runtime::traits::{StringProvider, IntegerProvider, DictProvider};
        use super::*;

        #[test]
        fn empty_stable() {
            let rt = setup_test();
            let tuple = rt.tuple(rs::None());
            let tuple2 = tuple.clone();

            let r1 = tuple.op_hash(&rt).unwrap();
            let r2 = tuple2.op_hash(&rt).unwrap();

            assert_eq!(r1, r2);
        }

        #[test]
        fn hashable_items() {
            let rt = setup_test();
            let empty = rt.tuple(rs::None());

            let tuple = rt.tuple(vec![rt.int(1), rt.int(2), rt.str("3")]);
            let tuple2 = rt.tuple(vec![rt.int(1), rt.int(2), rt.str("3")]);

            let r1 = tuple.op_hash(&rt).unwrap();
            let r2 = tuple2.op_hash(&rt).unwrap();
            let r3 = empty.op_hash(&rt).unwrap();

            assert_eq!(r1, r2);
            assert!(r1 != r3);
        }

        #[test]
        #[should_panic]
        fn unhashable_items_causes_error() {
            let rt = setup_test();

            let tuple = rt.tuple(vec![rt.dict(rs::None())]);
            tuple.op_hash(&rt).unwrap();
        }
    }
}
