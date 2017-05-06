use std::fmt;
use std::default::Default;
use std::ops::{Deref};
use std::borrow::Borrow;

use error::{Error, ErrorType};
use runtime::Runtime;
use traits::IntegerProvider;
use result::{RuntimeResult, NativeResult};
use object::{self, RtValue, typing};
use object::selfref::{self, SelfRef};
use object::method::{self, GetItem, Next};
use object::typing::BuiltinType;

use typedef::builtin::Builtin;
use typedef::native;
use ::object::RtObject as ObjectRef;


pub struct PyIteratorType {
}

impl PyIteratorType {

    pub fn empty(&self, rt: &Runtime) -> ObjectRef {
        let value = IteratorValue(native::Iterator::Empty, rt.clone());
        self.new(&rt, value)
    }
}

impl typing::BuiltinType for PyIteratorType {
    type T = PyIterator;
    type V = IteratorValue;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        PyIteratorType::inject_selfref(PyIteratorType::alloc(value))
    }

    fn init_type() -> Self {
        PyIteratorType {}
    }

    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::Iter(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Iter(ref tuple) => {
                tuple.rc.set(&objref.clone());
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
        write!(f, "Tuple({:?})", self.value.0)
    }
}

impl fmt::Debug for PyIterator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tuple({:?})", self.value.0)
    }
}


impl Iterator for PyIterator {
    type Item = ObjectRef;

    fn next(&mut self) -> Option<Self::Item> {
        match self.native_next() {
            Ok(objref) => Some(objref),
            Err(Error(ErrorType::StopIteration, _)) => None,
            Err(_) => panic!("Iterator logic fault")
        }
    }
}


impl object::PyAPI for PyIterator {}

impl method::Next for PyIterator {

    #[allow(unused_variables)]
    fn op_next(&self, rt: &Runtime) -> RuntimeResult {
        match self.value.0 {
            // TODO: {T82} Use weakref or some other mechanism to not keep a handle to source forever?
            _ => self.native_next()
        }
    }

    fn native_next(&self) -> NativeResult<ObjectRef> {
        let ref rt = self.value.1;

        match self.value.0 {
            // TODO: {T82} Use weakref or some other mechanism to not keep a handle to source forever?
            native::Iterator::Sequence {ref source, ref idx_next} => {
                let boxed: &Box<Builtin> = source.0.borrow();
                let mut idx = idx_next.get();
                let idx_obj = rt.int(idx);

                let boxed_idx: &Box<Builtin> = idx_obj.0.borrow();

                match boxed.native_getitem(&boxed_idx) {
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
    use traits::{IteratorProvider, BooleanProvider, IntegerProvider, NoneProvider, TupleProvider};
    use object::method::*;
    use test::Bencher;
    use super::*;

    fn setup_test() -> (Runtime) {
        Runtime::new()
    }

    #[test]
    fn is_() {
        let rt = setup_test();
        let iterator = rt.iter(native::None());
        let iterator2 = iterator.clone();

        let boxed: &Box<Builtin> = iterator.0.borrow();

        let result = boxed.op_is(&rt, &iterator2).unwrap();
        assert_eq!(result, rt.bool(true));

        let result = boxed.op_is(&rt, &rt.int(1)).unwrap();
        assert_eq!(result, rt.bool(false));
    }

    mod __next__ {
        use super::*;

        #[test]
        #[should_panic]
        fn empty() {
            let rt = setup_test();
            let iterator = rt.iter(native::None());

            let boxed: &Box<Builtin> = iterator.0.borrow();
            // Should raise an StopIteration error
            boxed.op_next(&rt).unwrap();
        }

        #[test]
        fn len3_tuple() {
            let rt = setup_test();
            let tuple = rt.tuple(vec![rt.none(), rt.int(1), rt.bool(true)]);
            let iterator = rt.iter(native::Iterator::new(&tuple).unwrap());

            let boxed: &Box<Builtin> = iterator.0.borrow();
            let result = boxed.op_next(&rt).unwrap();
            assert_eq!(result, rt.none());

            let result = boxed.op_next(&rt).unwrap();
            assert_eq!(result, rt.int(1));

            let result = boxed.op_next(&rt).unwrap();
            assert_eq!(result, rt.bool(true));
        }

        /// See the time it takes to fully consume an iterator backed by a reasonably sized
        /// tuple.
        #[bench]
        fn iter_objref(b: &mut Bencher) {
            let rt = setup_test();
            let tuple = rt.tuple(vec![rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.none(), rt.int(1), rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true),rt.bool(true)]);
            b.iter(|| {
                let iterator = rt.iter(native::Iterator::new(&tuple).unwrap());
                let boxed: &Box<Builtin> = iterator.0.borrow();
                match boxed.deref() {
                    &Builtin::Iter(ref iterator) => {
                        loop {
                            match iterator.native_next() {
                                Ok(_) => continue,
                                Err(Error(ErrorType::StopIteration, _)) => break,
                                Err(_) => panic!("Iterator logic fault")
                            };
                        }

                    },
                    _ => {}
                }
            });

            let iterator = rt.iter(native::Iterator::new(&tuple).unwrap());

            let mut results: Vec<ObjectRef> = vec![];

            for i in iterator {
                results.push(i)
            }

            println!("Total: {}", results.len());
        }

    }

}

