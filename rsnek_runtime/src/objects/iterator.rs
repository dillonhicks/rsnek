use std::fmt;
use std::default::Default;
use std::ops::{Deref};
use std::borrow::Borrow;

use ::api::result::{Error, ErrorType};
use runtime::Runtime;
use ::runtime::traits::IntegerProvider;
use ::api::result::{ObjectResult, RtResult};
use api::{self, RtValue, typing};
use api::selfref::{self, SelfRef};
use api::method::{self, GetItem, Next};
use api::typing::BuiltinType;

use objects::builtin::Builtin;
use objects::native;
use ::api::RtObject;


const TYPE_NAME: &'static str = "iter";


pub struct PyIteratorType {}


impl PyIteratorType {

    pub fn empty(&self, rt: &Runtime) -> RtObject {
        let value = IteratorValue(native::Iterator::Empty, rt.clone());
        self.new(&rt, value)
    }
}

impl typing::BuiltinType for PyIteratorType {
    type T = PyIterator;
    type V = IteratorValue;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> RtObject {
        PyIteratorType::inject_selfref(PyIteratorType::alloc(value))
    }

    fn init_type() -> Self {
        PyIteratorType {}
    }

    fn inject_selfref(value: Self::T) -> RtObject {
        let object = RtObject::new(Builtin::Iter(value));
        let new = object.clone();

        match object.as_ref() {
            &Builtin::Iter(ref tuple) => {
                tuple.rc.set(&object.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        PyIterator {
            value: value,
            rc: selfref::RefCount::default(),
        }
    }
}


pub struct IteratorValue(pub native::Iterator, pub Runtime);
pub type PyIterator = RtValue<IteratorValue>;


impl fmt::Display for PyIterator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Iterator({:?})", self.value.0)
    }
}

impl fmt::Debug for PyIterator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Iterator({:?})", self.value.0)
    }
}


impl Iterator for PyIterator {
    type Item = RtObject;

    fn next(&mut self) -> Option<Self::Item> {
        match self.native_next() {
            Ok(objref) => Some(objref),
            Err(Error(ErrorType::StopIteration, _)) => None,
            Err(_) => panic!("Iterator logic fault")
        }
    }
}


impl api::PyAPI for PyIterator {}

impl method::Next for PyIterator {

    #[allow(unused_variables)]
    fn op_next(&self, rt: &Runtime) -> ObjectResult {
        match self.value.0 {
            // TODO: {T82} Use weakref or some other mechanism to not keep a handle to source forever?
            _ => self.native_next()
        }
    }

    fn native_next(&self) -> RtResult<RtObject> {
        let ref rt = self.value.1;

        match self.value.0 {
            // TODO: {T82} Use weakref or some other mechanism to not keep a handle to source forever?
            native::Iterator::Sequence {ref source, ref idx_next} => {
                let mut idx = idx_next.get();;

                match source.native_getitem(rt.int(idx).as_ref()) {
                    Ok(objref) => {
                        idx += 1;
                        idx_next.set(idx);
                        Ok(objref)
                    },
                    Err(_) => Err(Error::stop_iteration())
                }
            }

            native::Iterator::Empty => Err(Error::stop_iteration())
        }
    }
}

method_not_implemented!(PyIterator,
    AbsValue   Add   AddItem   Append
    Await   BitwiseAnd   BitwiseOr   BooleanCast
    BytesCast   Call   Clear   Close
    ComplexCast   Contains   Count   DelAttr
    Delete   DeleteItem   DescriptorGet   DescriptorSet
    DescriptorSetName   Discard   DivMod   Enter
    Equal   Exit   Extend   FloatCast
    FloorDivision   Get   GetAttr   GetAttribute
    GetItem   GreaterOrEqual   GreaterThan   Hashed
    Id   InPlaceAdd   InPlaceBitwiseAnd   InPlaceBitwiseOr
    InPlaceDivMod   InPlaceFloorDivision   InPlaceLeftShift   InPlaceMatrixMultiply
    InPlaceModulus   InPlaceMultiply   InPlacePow   InPlaceRightShift
    InPlaceSubtract   InPlaceTrueDivision   InPlaceXOr   Index
    Init   IntegerCast   InvertValue   Is
    IsDisjoint   IsNot   Items   Iter
    Keys   LeftShift   Length   LengthHint
    LessOrEqual   LessThan   MatrixMultiply   Modulus
    Multiply   NegateValue   New
    NotEqual   Pop   PopItem   PositiveValue
    Pow   ReflectedAdd   ReflectedBitwiseAnd   ReflectedBitwiseOr
    ReflectedDivMod   ReflectedFloorDivision   ReflectedLeftShift   ReflectedMatrixMultiply
    ReflectedModulus   ReflectedMultiply   ReflectedPow   ReflectedRightShift
    ReflectedSubtract   ReflectedTrueDivision   ReflectedXOr   Remove
    Reversed   RightShift   Rounding   Send
    SetAttr   SetDefault   SetItem   StringCast
    StringFormat   StringRepresentation   Subtract   Throw
    TrueDivision   Update   Values   XOr
);


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use ::runtime::traits::{IteratorProvider, BooleanProvider, IntegerProvider,
                 StringProvider, NoneProvider, TupleProvider};
    use api::method::*;
    use test::Bencher;
    use super::*;

    fn setup_test() -> (Runtime) {
        Runtime::new()
    }

    #[test]
    fn is_() {
        let rt = setup_test();
        let iter = rt.iter(native::None());
        let iter2 = iter.clone();

        let result = iter.op_is(&rt, &iter2).unwrap();
        assert_eq!(result, rt.bool(true));

        let result = iter.op_is(&rt, &rt.int(1)).unwrap();
        assert_eq!(result, rt.bool(false));
    }

    mod __next__ {
        use super::*;

        #[test]
        #[should_panic]
        fn empty() {
            let rt = setup_test();
            let iterator = rt.iter(native::None());
            // Should raise an StopIteration error
            iterator.op_next(&rt).unwrap();
        }

        #[test]
        fn len3_tuple() {
            let rt = setup_test();
            let tuple = rt.tuple(vec![rt.none(), rt.int(1), rt.bool(true)]);
            let iter = rt.iter(native::Iterator::new(&tuple).unwrap());

            let result = iter.op_next(&rt).unwrap();
            assert_eq!(result, rt.none());

            let result = iter.op_next(&rt).unwrap();
            assert_eq!(result, rt.int(1));

            let result = iter.op_next(&rt).unwrap();
            assert_eq!(result, rt.bool(true));
        }

        macro_rules! iter_bench (
        ($name:ident, $N:expr) => (
        #[bench]
        fn $name(b: &mut Bencher) {
            let rt = setup_test();
            let elems = (0..$N).map(|i|
                {
                    match i % 5 {
                        0 => rt.bool(false),
                        1 => rt.bool(true),
                        2 => rt.int(i),
                        3 => rt.str(format!("{}", i)),
                        4 |
                        _ => rt.none()

                    }
                })
                .collect::<Vec<_>>();

            let tuple = rt.tuple(elems);

            b.iter(|| {
                let iter = rt.iter(native::Iterator::new(&tuple).unwrap());
                loop {
                    match iter.op_next(&rt) {
                        Ok(_) => continue,
                        Err(Error(ErrorType::StopIteration, _)) => break,
                        Err(_) => panic!("Iterator logic fault")
                    };
                }
            });
        }

        );
    );

        iter_bench!(iter_list_elems_0,      0);
        iter_bench!(iter_list_elems_1,      1);
        iter_bench!(iter_list_elems_4,      4);
        iter_bench!(iter_list_elems_16,     16);
        iter_bench!(iter_list_elems_64,     64);
        iter_bench!(iter_list_elems_256,    256);
        iter_bench!(iter_list_elems_1024,   1024);
        iter_bench!(iter_list_elems_4096,   4095);
        iter_bench!(iter_list_elems_16384,  16384);
    }

}

