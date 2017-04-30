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
use typedef::objectref::ObjectRef;


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


impl object::PyAPI for PyIterator {}
impl method::New for PyIterator {}
impl method::Init for PyIterator {}
impl method::Delete for PyIterator {}
impl method::GetAttr for PyIterator {}
impl method::GetAttribute for PyIterator {}
impl method::SetAttr for PyIterator {}
impl method::DelAttr for PyIterator {}
impl method::Hashed for PyIterator {}
impl method::StringCast for PyIterator {}
impl method::BytesCast for PyIterator {}
impl method::StringFormat for PyIterator {}
impl method::StringRepresentation for PyIterator {}
impl method::Equal for PyIterator {}
impl method::NotEqual for PyIterator {}
impl method::LessThan for PyIterator {}
impl method::LessOrEqual for PyIterator {}
impl method::GreaterOrEqual for PyIterator {}
impl method::GreaterThan for PyIterator {}
impl method::BooleanCast for PyIterator {}
impl method::IntegerCast for PyIterator {}
impl method::FloatCast for PyIterator {}
impl method::ComplexCast for PyIterator {}
impl method::Rounding for PyIterator {}
impl method::Index for PyIterator {}
impl method::NegateValue for PyIterator {}
impl method::AbsValue for PyIterator {}
impl method::PositiveValue for PyIterator {}
impl method::InvertValue for PyIterator {}
impl method::Add for PyIterator {}
impl method::BitwiseAnd for PyIterator {}
impl method::DivMod for PyIterator {}
impl method::FloorDivision for PyIterator {}
impl method::LeftShift for PyIterator {}
impl method::Modulus for PyIterator {}
impl method::Multiply for PyIterator {}
impl method::MatrixMultiply for PyIterator {}
impl method::BitwiseOr for PyIterator {}
impl method::Pow for PyIterator {}
impl method::RightShift for PyIterator {}
impl method::Subtract for PyIterator {}
impl method::TrueDivision for PyIterator {}
impl method::XOr for PyIterator {}
impl method::ReflectedAdd for PyIterator {}
impl method::ReflectedBitwiseAnd for PyIterator {}
impl method::ReflectedDivMod for PyIterator {}
impl method::ReflectedFloorDivision for PyIterator {}
impl method::ReflectedLeftShift for PyIterator {}
impl method::ReflectedModulus for PyIterator {}
impl method::ReflectedMultiply for PyIterator {}
impl method::ReflectedMatrixMultiply for PyIterator {}
impl method::ReflectedBitwiseOr for PyIterator {}
impl method::ReflectedPow for PyIterator {}
impl method::ReflectedRightShift for PyIterator {}
impl method::ReflectedSubtract for PyIterator {}
impl method::ReflectedTrueDivision for PyIterator {}
impl method::ReflectedXOr for PyIterator {}
impl method::InPlaceAdd for PyIterator {}
impl method::InPlaceBitwiseAnd for PyIterator {}
impl method::InPlaceDivMod for PyIterator {}
impl method::InPlaceFloorDivision for PyIterator {}
impl method::InPlaceLeftShift for PyIterator {}
impl method::InPlaceModulus for PyIterator {}
impl method::InPlaceMultiply for PyIterator {}
impl method::InPlaceMatrixMultiply for PyIterator {}
impl method::InPlaceBitwiseOr for PyIterator {}
impl method::InPlacePow for PyIterator {}
impl method::InPlaceRightShift for PyIterator {}
impl method::InPlaceSubtract for PyIterator {}
impl method::InPlaceTrueDivision for PyIterator {}
impl method::InPlaceXOr for PyIterator {}
impl method::Contains for PyIterator {}
impl method::Iter for PyIterator {}
impl method::Call for PyIterator {}
impl method::Length for PyIterator {}
impl method::LengthHint for PyIterator {}
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

impl method::Reversed for PyIterator {}
impl method::GetItem for PyIterator {}
impl method::SetItem for PyIterator {}
impl method::DeleteItem for PyIterator {}
impl method::Count for PyIterator {}
impl method::Append for PyIterator {}
impl method::Extend for PyIterator {}
impl method::Pop for PyIterator {}
impl method::Remove for PyIterator {}
impl method::IsDisjoint for PyIterator {}
impl method::AddItem for PyIterator {}
impl method::Discard for PyIterator {}
impl method::Clear for PyIterator {}
impl method::Get for PyIterator {}
impl method::Keys for PyIterator {}
impl method::Values for PyIterator {}
impl method::Items for PyIterator {}
impl method::PopItem for PyIterator {}
impl method::Update for PyIterator {}
impl method::SetDefault for PyIterator {}
impl method::Await for PyIterator {}
impl method::Send for PyIterator {}
impl method::Throw for PyIterator {}
impl method::Close for PyIterator {}
impl method::Exit for PyIterator {}
impl method::Enter for PyIterator {}
impl method::DescriptorGet for PyIterator {}
impl method::DescriptorSet for PyIterator {}
impl method::DescriptorSetName for PyIterator {}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//      stdlib traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
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


#[cfg(test)]
mod _api_method {
    #[allow(unused_imports)]
    use runtime::{IteratorProvider, BooleanProvider, IntegerProvider, NoneProvider, TupleProvider};
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

