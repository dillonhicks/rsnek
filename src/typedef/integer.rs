use std;
use std::fmt;
use std::borrow::Borrow;
use std::ops::Deref;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use num::Zero;

use runtime::{Runtime, BooleanProvider, StringProvider, IntegerProvider};
use error::Error;
use result::{NativeResult, RuntimeResult};
use object::{self, RtValue, method, typing};
use object::selfref::{self, SelfRef};

use typedef::native;
use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;


pub const STATIC_INT_IDX_OFFSET: usize = 5;
pub const STATIC_INT_RANGE: std::ops::Range<isize> = (-(STATIC_INT_IDX_OFFSET as isize)..1025);
pub const STATIC_INT_RANGE_MAX: usize = 1025 + STATIC_INT_IDX_OFFSET;


#[inline(always)]
pub fn format_int(int: &native::Integer) -> native::String {
    format!("{}", *int)
}


#[derive(Clone)]
pub struct PyIntegerType {
    pub static_integers: Vec<ObjectRef>
}


impl typing::BuiltinType for PyIntegerType {
    type T = PyInteger;
    type V = native::Integer;

    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        // TODO: Add check for static range
        PyIntegerType::inject_selfref(PyIntegerType::alloc(value))
    }


    fn init_type() -> Self {
        let range: Vec<ObjectRef> =
            STATIC_INT_RANGE
                .map(|int| native::Integer::from(int))
                .map(|value| {
                    let int = PyIntegerType::alloc(value);
                    PyIntegerType::inject_selfref(int)
                })
                .collect();
                //.map(|obj| heap.alloc_static(obj.to()).unwrap()).collect();

        PyIntegerType {
            static_integers: range
        }
    }

    fn inject_selfref(value: PyInteger) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::Int(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Int(ref int) => {
                int.rc.set(&objref.clone());
            },
            _ => unreachable!()
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        PyInteger {
            value: IntValue(value),
            rc: selfref::RefCount::default(),
        }
    }

}


impl method::New for PyIntegerType {}
impl method::Init for PyIntegerType {}
impl method::Delete for PyIntegerType {}


pub struct IntValue(native::Integer);
pub type PyInteger = RtValue<IntValue>;

// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    Python Object Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
impl object::PyAPI for PyInteger {}
impl method::New for PyInteger {}
impl method::Init for PyInteger {}
impl method::Delete for PyInteger {}
impl method::GetAttr for PyInteger {}
impl method::GetAttribute for PyInteger {}
impl method::SetAttr for PyInteger {}
impl method::DelAttr for PyInteger {}
impl method::Id for PyInteger {}
impl method::Is for PyInteger {}
impl method::IsNot for PyInteger {}
impl method::Hashed for PyInteger {
    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_hash() {
            Ok(value) => Ok(rt.int(native::Integer::from(value))),
            Err(err) => Err(err),
        }
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
        let mut s = DefaultHasher::new();
        self.value.0.hash(&mut s);
        Ok(s.finish())
    }

}

impl method::StringCast for PyInteger {
    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_str() {
            Ok(string) => Ok(rt.str(string)),
            Err(_) => unreachable!(),
        }
    }

    fn native_str(&self) -> NativeResult<native::String> {
        Ok(format_int(&self.value.0))
    }
}

impl method::BytesCast for PyInteger {}
impl method::StringFormat for PyInteger {}
impl method::StringRepresentation for PyInteger {
    fn op_repr(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_repr() {
            Ok(string) => Ok(rt.str(string)),
            Err(_) => unreachable!(),
        }
    }

    fn native_repr(&self) -> NativeResult<native::String> {
        Ok(format_int(&self.value.0))
    }
}

impl method::Equal for PyInteger {
    fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match self.native_eq(builtin.deref()) {
            Ok(value) => Ok(rt.bool(value)),
            Err(err) => Err(err),
        }
    }

    fn native_eq(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        match *other {
            Builtin::Int(ref obj) => Ok(self.value.0 == obj.value.0),
            _ => Ok(false),
        }
    }
}
impl method::NotEqual for PyInteger {}
impl method::LessThan for PyInteger {}
impl method::LessOrEqual for PyInteger {}
impl method::GreaterOrEqual for PyInteger {}
impl method::GreaterThan for PyInteger {}
impl method::BooleanCast for PyInteger {
    fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
        if self.native_bool().unwrap() {
            Ok(rt.bool(true))
        } else {
            Ok(rt.bool(false))
        }
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
        return Ok(!self.value.0.is_zero());
    }
}
impl method::IntegerCast for PyInteger {}
impl method::FloatCast for PyInteger {}
impl method::ComplexCast for PyInteger {}
impl method::Rounding for PyInteger {}
impl method::Index for PyInteger {}
impl method::NegateValue for PyInteger {}
impl method::AbsValue for PyInteger {}
impl method::PositiveValue for PyInteger {}
impl method::InvertValue for PyInteger {}
impl method::Add for PyInteger {
    fn op_add(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
        let builtin: &Box<Builtin> = rhs.0.borrow();

        match builtin.deref() {
            &Builtin::Int(ref obj) => {
                Ok(rt.int(&self.value.0 + &obj.value.0))
            }
            _ => Err(Error::typerr("TypeError cannot add to int")),
        }
    }

}

impl method::BitwiseAnd for PyInteger {}
impl method::DivMod for PyInteger {}
impl method::FloorDivision for PyInteger {}
impl method::LeftShift for PyInteger {}
impl method::Modulus for PyInteger {}
impl method::Multiply for PyInteger {}
impl method::MatrixMultiply for PyInteger {}
impl method::BitwiseOr for PyInteger {}
impl method::Pow for PyInteger {}
impl method::RightShift for PyInteger {}
impl method::Subtract for PyInteger {}
impl method::TrueDivision for PyInteger {}
impl method::XOr for PyInteger {}
impl method::ReflectedAdd for PyInteger {}
impl method::ReflectedBitwiseAnd for PyInteger {}
impl method::ReflectedDivMod for PyInteger {}
impl method::ReflectedFloorDivision for PyInteger {}
impl method::ReflectedLeftShift for PyInteger {}
impl method::ReflectedModulus for PyInteger {}
impl method::ReflectedMultiply for PyInteger {}
impl method::ReflectedMatrixMultiply for PyInteger {}
impl method::ReflectedBitwiseOr for PyInteger {}
impl method::ReflectedPow for PyInteger {}
impl method::ReflectedRightShift for PyInteger {}
impl method::ReflectedSubtract for PyInteger {}
impl method::ReflectedTrueDivision for PyInteger {}
impl method::ReflectedXOr for PyInteger {}
impl method::InPlaceAdd for PyInteger {}
impl method::InPlaceBitwiseAnd for PyInteger {}
impl method::InPlaceDivMod for PyInteger {}
impl method::InPlaceFloorDivision for PyInteger {}
impl method::InPlaceLeftShift for PyInteger {}
impl method::InPlaceModulus for PyInteger {}
impl method::InPlaceMultiply for PyInteger {}
impl method::InPlaceMatrixMultiply for PyInteger {}
impl method::InPlaceBitwiseOr for PyInteger {}
impl method::InPlacePow for PyInteger {}
impl method::InPlaceRightShift for PyInteger {}
impl method::InPlaceSubtract for PyInteger {}
impl method::InPlaceTrueDivision for PyInteger {}
impl method::InPlaceXOr for PyInteger {}
impl method::Contains for PyInteger {}
impl method::Iter for PyInteger {}
impl method::Call for PyInteger {}
impl method::Length for PyInteger {}
impl method::LengthHint for PyInteger {}
impl method::Next for PyInteger {}
impl method::Reversed for PyInteger {}
impl method::GetItem for PyInteger {}
impl method::SetItem for PyInteger {}
impl method::DeleteItem for PyInteger {}
impl method::Count for PyInteger {}
impl method::Append for PyInteger {}
impl method::Extend for PyInteger {}
impl method::Pop for PyInteger {}
impl method::Remove for PyInteger {}
impl method::IsDisjoint for PyInteger {}
impl method::AddItem for PyInteger {}
impl method::Discard for PyInteger {}
impl method::Clear for PyInteger {}
impl method::Get for PyInteger {}
impl method::Keys for PyInteger {}
impl method::Values for PyInteger {}
impl method::Items for PyInteger {}
impl method::PopItem for PyInteger {}
impl method::Update for PyInteger {}
impl method::SetDefault for PyInteger {}
impl method::Await for PyInteger {}
impl method::Send for PyInteger {}
impl method::Throw for PyInteger {}
impl method::Close for PyInteger {}
impl method::Exit for PyInteger {}
impl method::Enter for PyInteger {}
impl method::DescriptorGet for PyInteger {}
impl method::DescriptorSet for PyInteger {}
impl method::DescriptorSetName for PyInteger {}


// ---------------
//  stdlib traits
// ---------------


impl fmt::Display for PyInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.0)
    }
}

impl fmt::Debug for PyInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value.0)
    }
}

#[cfg(all(feature="old", test))]
mod old {
    // ---
    // OLD
    // ---

    #[derive(Clone, Debug, Hash, Eq, PartialEq)]
    pub struct IntegerObject {
        pub value: native::Integer,
    }

    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///       Struct Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

    impl IntegerObject {
        #[inline]
        pub fn new_i64(value: i64) -> IntegerObject {
            let integer = IntegerObject { value: native::Integer::from(value) };

            return integer;
        }

        #[inline]
        pub fn new_u64(value: u64) -> IntegerObject {
            let integer = IntegerObject { value: native::Integer::from(value) };

            return integer;
        }

        pub fn new_bigint(value: native::Integer) -> IntegerObject {
            let integer = IntegerObject { value: native::Integer::from(value) };

            return integer;
        }
    }


    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    //    Python Object Traits - old
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+

    impl objectref::RtObject for IntegerObject {}

    impl object::model::PyObject for IntegerObject {}

    impl object::model::PyBehavior for IntegerObject {
        fn op_add(&self, runtime: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
            // If this fails the interpreter is fucked anyways because the runtime has been dealloc'd
            let builtin: &Box<Builtin> = rhs.0.borrow();

            match builtin.deref() {
                &Builtin::Integer(ref obj) => {
                    let new_number: ObjectRef = IntegerObject::new_bigint(&self.value + &obj.value).to();
                    runtime.alloc(new_number)
                }
                &Builtin::Float(ref obj) => {
                    let new_number: ObjectRef = FloatObject::add_integer(obj, &self)?.to();
                    runtime.alloc(new_number)
                }
                _ => Err(Error(ErrorType::Type, "TypeError cannot add to int")),
            }
        }

        fn native_hash(&self) -> NativeResult<native::HashId> {
            let mut s = SipHasher::new();
            self.hash(&mut s);
            Ok(s.finish())
        }

        fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
            match self.native_hash() {
                Ok(value) => rt.alloc(ObjectRef::new(Builtin::Integer(IntegerObject::new_u64(value)))),
                Err(err) => Err(err),
            }
        }

        fn op_eq(&self, rt: &Runtime, rhs: &ObjectRef) -> RuntimeResult {
            let builtin: &Box<Builtin> = rhs.0.borrow();

            match self.native_eq(builtin.deref()) {
                Ok(value) => if value { Ok(rt.OldTrue()) } else { Ok(rt.OldFalse()) },
                Err(err) => Err(err),
            }
        }

        fn native_eq(&self, other: &Builtin) -> NativeResult<native::Boolean> {
            match other {
                &Builtin::Integer(ref obj) => Ok(self.value == obj.value),
                _ => Ok(false),
            }
        }

        fn native_bool(&self) -> NativeResult<native::Boolean> {
            return Ok(!self.value.is_zero());
        }

        fn op_int(&self, rt: &Runtime) -> RuntimeResult {
            match self.native_int() {
                Ok(int) => {
                    // TODO: once self refs are are implemented, just
                    // clone the ref and pass it back
                    rt.alloc(IntegerObject::new_bigint(int).to())
                }
                Err(err) => Err(err),
            }
        }

        fn native_int(&self) -> NativeResult<native::Integer> {
            return Ok(self.value.clone());
        }

        fn op_float(&self, rt: &Runtime) -> RuntimeResult {
            match self.native_float() {
                Ok(float) => rt.alloc(FloatObject::new(float).to()),
                Err(err) => Err(err),
            }
        }

        fn native_float(&self) -> NativeResult<native::Float> {
            return Ok(self.value.to_f64().unwrap());
        }

        fn op_complex(&self, rt: &Runtime) -> RuntimeResult {
            match self.native_complex() {
                Ok(value) => rt.alloc(ComplexObject::from_native(value).to()),
                Err(err) => Err(err),
            }
        }

        fn native_complex(&self) -> NativeResult<native::Complex> {
            return Ok(native::Complex::new(self.value.to_f64().unwrap(), 0.));
        }

        fn op_index(&self, rt: &Runtime) -> RuntimeResult {
            self.op_int(&rt)
        }

        fn native_index(&self) -> NativeResult<native::Integer> {
            return self.native_int();
        }

        fn op_repr(&self, rt: &Runtime) -> RuntimeResult {
            match self.native_repr() {
                Ok(string) => Ok(rt.str(string)),
                Err(err) => unreachable!(),
            }
        }

        fn native_repr(&self) -> NativeResult<native::String> {
            Ok(format!("{}", self.value))
        }

        fn op_str(&self, rt: &Runtime) -> RuntimeResult {
            self.op_repr(rt)
        }

        fn native_str(&self) -> NativeResult<native::String> {
            return self.native_repr();
        }
    }


    impl objectref::ToRtWrapperType<Builtin> for IntegerObject {
        #[inline]
        fn to(self) -> Builtin {
            return Builtin::Integer(self);
        }
    }

    impl objectref::ToRtWrapperType<ObjectRef> for IntegerObject {
        #[inline]
        fn to(self) -> ObjectRef {
            ObjectRef::new(self.to())
        }
    }


    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///      stdlib Traits
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+

    impl fmt::Display for IntegerObject {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }


    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+
///          Tests
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+
    #[cfg(test)]
    mod impl_pybehavior {
        use std;
        use std::rc::Rc;
        use std::ops::Deref;
        use typedef::objectref::{self, ObjectRef};

        use runtime::{Runtime, DEFAULT_HEAP_CAPACITY};
        use typedef::integer;
        use typedef::builtin::Builtin;
        use super::{Integer, IntegerObject};
        use typedef::float::FloatObject;
        use typedef::string::StringObject;
        use typedef::tuple::TupleObject;
        use typedef::list::ListObject;
        use typedef::objectref::ToRtWrapperType;
        use typedef::complex::ComplexObject;
        use object::model::PyBehavior;

        use num::ToPrimitive;
        use std::cmp::PartialEq;
        use std::borrow::Borrow;


        #[test]
        fn test_integer_alloc() {
            let mut runtime = Runtime::new(None);
            assert_eq!(runtime.heap_size(), 0);

            let one_object = ObjectRef::new(Builtin::Integer(IntegerObject::new_i64(1)));
            let one: ObjectRef = runtime.alloc(one_object.clone()).unwrap();

            let one_clone = one.clone();
            assert_eq!(Rc::strong_count(&one.0), 5);
        }

        /// Create integer object on the stack and try to allocate it
    /// in the runtime.
    ///
        #[test]
        fn test_alloc_integer() {
            let mut runtime = Runtime::new(None);
            assert_eq!(runtime.heap_size(), 0);

            let one_object: ObjectRef = IntegerObject::new_i64(1).to();
            let one: ObjectRef = runtime.alloc(one_object.clone()).unwrap();

            /// A new integer should only alloc one spot on the heap
            assert_eq!(runtime.heap_size(), 1);
            println!("{:?}", runtime);
        }

        /// int+int => int
    /// api::BinaryOperation::op_add
        #[test]
        fn __add__() {
            let mut runtime = Runtime::new(None);
            assert_eq!(runtime.heap_size(), 0);

            let one_object: ObjectRef = IntegerObject::new_i64(1).to();
            let one: ObjectRef = runtime.alloc(one_object.clone()).unwrap();
            assert_eq!(runtime.heap_size(), 1);

            let another_one: ObjectRef = IntegerObject::new_i64(1).to();
            let one2: ObjectRef = runtime.alloc(another_one.clone()).unwrap();
            assert_eq!(runtime.heap_size(), 2);

            let one_ref: &Box<Builtin> = one.0.borrow();
            let two = one_ref.op_add(&mut runtime, &another_one).unwrap();
            assert_eq!(runtime.heap_size(), 3);

            println!("{:?}", runtime);
        }

        /// Just try to init the runtime
        #[test]
        fn __eq__() {
            let mut rt = Runtime::new(None);
            assert_eq!(rt.heap_size(), 0);


            let one_object: ObjectRef = IntegerObject::new_i64(1).to();
            let one: ObjectRef = rt.alloc(one_object.clone()).unwrap();
            assert_eq!(rt.heap_size(), 1);

            let another_one: ObjectRef = IntegerObject::new_i64(1).to();
            let one2: ObjectRef = rt.alloc(another_one.clone()).unwrap();
            assert_eq!(rt.heap_size(), 2);

            println!("{:?}", rt);

            let one_builtin: &Box<Builtin> = one.0.borrow();
            let result = one_builtin.op_eq(&mut rt, &one2).unwrap();

            assert_eq!(result, rt.OldTrue())
        }

        #[test]
        #[allow(non_snake_case)]
        fn __bool__() {
            let mut rt = Runtime::new(None);

            let neg_one: ObjectRef = rt.alloc(IntegerObject::new_i64(-1).to()).unwrap();
            let zero: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
            let pos_one: ObjectRef = rt.alloc(IntegerObject::new_u64(1).to()).unwrap();

            let test_case: &Box<Builtin> = neg_one.0.borrow();
            let result = test_case.op_bool(&rt).unwrap();
            assert_eq!(result, rt.OldTrue());

            let test_case: &Box<Builtin> = zero.0.borrow();
            let result = test_case.op_bool(&rt).unwrap();
            assert_eq!(result, rt.OldFalse());

            let test_case: &Box<Builtin> = pos_one.0.borrow();
            let result = test_case.op_bool(&rt).unwrap();
            assert_eq!(result, rt.OldTrue());
        }

        #[test]
        fn __int__() {
            let mut rt = Runtime::new(None);

            let neg_one: ObjectRef = rt.alloc(IntegerObject::new_i64(-1).to()).unwrap();
            let zero: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
            let pos_one: ObjectRef = rt.alloc(IntegerObject::new_u64(1).to()).unwrap();

            let test_case: &Box<Builtin> = neg_one.0.borrow();
            let result = test_case.op_int(&rt).unwrap();
            assert_eq!(result, neg_one);

            let test_case: &Box<Builtin> = zero.0.borrow();
            let result = test_case.op_int(&rt).unwrap();
            assert_eq!(result, zero);

            let test_case: &Box<Builtin> = pos_one.0.borrow();
            let result = test_case.op_int(&rt).unwrap();
            assert_eq!(result, pos_one);
        }

        #[test]
        fn __complex__() {
            let mut rt = Runtime::new(None);

            let zero_complex = rt.alloc(ComplexObject::from_f64(0.0, 0.0).to()).unwrap();
            let one_complex = rt.alloc(ComplexObject::from_f64(1.0, 0.0).to()).unwrap();

            let zero_int: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
            let one_int: ObjectRef = rt.alloc(IntegerObject::new_u64(1).to()).unwrap();

            let test_case: &Box<Builtin> = zero_int.0.borrow();
            let result = test_case.op_complex(&rt).unwrap();
            assert_eq!(result, zero_complex);

            let test_case: &Box<Builtin> = one_int.0.borrow();
            let result = test_case.op_complex(&rt).unwrap();
            assert_eq!(result, one_complex);
        }

        #[test]
        fn __float__() {
            let mut rt = Runtime::new(None);

            let zero_float = rt.alloc(FloatObject::new(0.0).to()).unwrap();
            let one_float = rt.alloc(FloatObject::new(1.0).to()).unwrap();

            let zero_int: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
            let one_int: ObjectRef = rt.alloc(IntegerObject::new_u64(1).to()).unwrap();

            let test_case: &Box<Builtin> = zero_int.0.borrow();
            let result = test_case.op_complex(&rt).unwrap();
            assert_eq!(result, zero_float);

            let test_case: &Box<Builtin> = one_int.0.borrow();
            let result = test_case.op_complex(&rt).unwrap();
            assert_eq!(result, one_float);
        }

        #[test]
        fn __str__() {
            let mut rt = Runtime::new(None);

            let neg_one_str: ObjectRef = rt.alloc(StringObject::from_str("-1").to()).unwrap();
            let zero_str: ObjectRef = rt.alloc(StringObject::from_str("0").to()).unwrap();
            let pos_one_str: ObjectRef = rt.alloc(StringObject::from_str("1").to()).unwrap();

            let neg_one: ObjectRef = rt.alloc(IntegerObject::new_i64(-1).to()).unwrap();
            let zero: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
            let pos_one: ObjectRef = rt.alloc(IntegerObject::new_u64(1).to()).unwrap();

            let test_case: &Box<Builtin> = neg_one.0.borrow();
            let result = test_case.op_str(&rt).unwrap();
            assert_eq!(result, neg_one_str);

            let test_case: &Box<Builtin> = zero.0.borrow();
            let result = test_case.op_str(&rt).unwrap();
            assert_eq!(result, zero_str);

            let test_case: &Box<Builtin> = pos_one.0.borrow();
            let result = test_case.op_str(&rt).unwrap();
            assert_eq!(result, pos_one_str);
        }


        #[test]
        fn __repr__() {
            let mut rt = Runtime::new(None);

            let neg_one_str: ObjectRef = rt.alloc(StringObject::from_str("-1").to()).unwrap();
            let zero_str: ObjectRef = rt.alloc(StringObject::from_str("0").to()).unwrap();
            let pos_one_str: ObjectRef = rt.alloc(StringObject::from_str("1").to()).unwrap();

            let neg_one: ObjectRef = rt.alloc(IntegerObject::new_i64(-1).to()).unwrap();
            let zero: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
            let pos_one: ObjectRef = rt.alloc(IntegerObject::new_u64(1).to()).unwrap();

            let test_case: &Box<Builtin> = neg_one.0.borrow();
            let result = test_case.op_repr(&rt).unwrap();
            assert_eq!(result, neg_one_str);

            let test_case: &Box<Builtin> = zero.0.borrow();
            let result = test_case.op_repr(&rt).unwrap();
            assert_eq!(result, zero_str);

            let test_case: &Box<Builtin> = pos_one.0.borrow();
            let result = test_case.op_repr(&rt).unwrap();
            assert_eq!(result, pos_one_str);
        }

        api_test_stub!(unary, self, __del__, Delete, op_del, native_del);
        // api_test_stub!(unary, self, __repr__, ToStringRepr, op_repr, native_repr);
        // api_test_stub!(unary, self, __str__, ToString, op_str, native_str);

        /// Called by `bytes()` to compute a byte-string representation of an object.
    /// This should return a bytes object.
        api_test_stub!(unary, self, __bytes__, ToBytes, op_bytes, native_bytes);
        api_test_stub!(binary, self, __format__, Format, op_format, native_format);


        /// The object comparison functions are useful for all objects,
    /// and are named after the rich comparison operators they support:
        api_test_stub!(binary, self, __lt__, LessThan, op_lt, native_lt);
        api_test_stub!(binary, self, __le__, LessOrEqual, op_le, native_le);
        //api_test_stub!(binary, self, __eq__, Equal, op_eq, native_eq, native::Boolean);
        api_test_stub!(binary, self, __ne__, NotEqual, op_ne, native_ne, native::Boolean);
        api_test_stub!(binary, self, __ge__, GreaterOrEqual, op_ge, native_ge);
        api_test_stub!(binary, self, __gt__, GreaterThan, op_gt, native_gt);

        // Called by built-in function hash() and for operations on members of hashed collections including
        // set, frozenset, and dict. __hash__() should return an integer. The only required property is
        // that objects which compare equal have the same hash value; it is advised to mix together
        // the hash values of the components of the object that also play a part in comparison
        // of objects by packing them into a tuple and hashing the tuple. Example:
        api_test_stub!(unary, self, __hash__, Hashable, op_hash, native_hash, native::HashId);

        // Identity operators
        api_test_stub!(unary, self, identity, Identity, identity, native_identity, native::Boolean);
        // api_test_stub!(unary, self, __bool__, Truth, op_bool, native_bool, native::Boolean);
        api_test_stub!(unary, self, __not__, Not, op_not, native_not, native::Boolean);
        api_test_stub!(binary, self, is_, Is, op_is, native_is, native::Boolean);
        api_test_stub!(binary, self, is_not, IsNot, op_is_not, native_is_not, native::Boolean);

        // 3.3.6. Emulating container types
        api_test_stub!(unary, self, __len__, Length, op_len, native_len);
        api_test_stub!(unary, self, __length_hint__, LengthHint, op_length_hint, native_length_hint);
        api_test_stub!(binary, self, __getitem__, GetItem, op_getitem, native_getitem);
        api_test_stub!(binary, self, __missing__, MissingItem, op_missing, native_missing);
        api_test_stub!(ternary, self, __setitem__, SetItem, op_setitem, native_setitem);
        api_test_stub!(binary, self, __delitem__, DeleteItem, op_delitem, native_delitem);
        api_test_stub!(unary, self, __iter__, Iterator, op_iter, native_iter);
        api_test_stub!(unary, self, __reversed__, Reverse, op_reverse, native_reverse);
        api_test_stub!(binary, self, __contains__, Contains, op_contains, native_contains);

        // 3.3.7. Emulating numeric types
        //
        // The following methods can be defined to emulate numeric objects. Methods corresponding to
        // operations that are not supported by the particular kind of number implemented
        // (e.g., bitwise operations for non-integral numbers) should be left undefined.
        // api_test_stub!(binary, self, __add__, Add, op_add, native_add);
        api_test_stub!(binary, self, __and__, And, op_and, native_and);
        api_test_stub!(binary, self, __divmod__, DivMod, op_divmod, native_divmod);
        api_test_stub!(binary, self, __floordiv__, FloorDivision, op_floordiv, native_floordiv);
        api_test_stub!(binary, self, __lshift__, LeftShift, op_lshift, native_lshift);
        api_test_stub!(binary, self, __mod__, Modulus, op_mod, native_mod);
        api_test_stub!(binary, self, __mul__, Multiply, op_mul, native_mul);
        api_test_stub!(binary, self, __matmul__, MatrixMultiply, op_matmul, native_matmul);
        api_test_stub!(binary, self, __or__, Or, op_or, native_or);
        api_test_stub!(ternary, self, __pow__, Pow, op_pow, native_pow);
        api_test_stub!(binary, self, __rshift__, RightShift, op_rshift, native_rshift);
        api_test_stub!(binary, self, __sub__, Subtract, op_sub, native_sub);
        api_test_stub!(binary, self, __truediv__, TrueDivision, op_truediv, native_truediv);
        api_test_stub!(binary, self, __xor__, XOr, op_xor, native_xor);

        // Reflected Operators
        api_test_stub!(binary, self, __radd__, ReflectedAdd, op_radd, native_radd);
        api_test_stub!(binary, self, __rand__, ReflectedAnd, op_rand, native_rand);
        api_test_stub!(binary, self, __rdivmod__, ReflectedDivMod, op_rdivmod, native_rdivmod);
        api_test_stub!(binary, self, __rfloordiv__, ReflectedFloorDivision, op_rfloordiv, native_rfloordiv);
        api_test_stub!(binary, self, __rlshift__, ReflectedLeftShift, op_rlshift, native_rlshift);
        api_test_stub!(binary, self, __rmod__, ReflectedModulus, op_rmod, native_rmod);
        api_test_stub!(binary, self, __rmul__, ReflectedMultiply, op_rmul, native_rmul);
        api_test_stub!(binary, self, __rmatmul__, ReflectedMatrixMultiply, op_rmatmul, native_rmatmul);
        api_test_stub!(binary, self, __ror__, ReflectedOr, op_ror, native_ror);
        api_test_stub!(binary, self, __rpow__, ReflectedPow, op_rpow, native_rpow);
        api_test_stub!(binary, self, __rrshift__, ReflectedRightShift, op_rrshift, native_rrshift);
        api_test_stub!(binary, self, __rsub__, ReflectedSubtract, op_rsub, native_rsub);
        api_test_stub!(binary, self, __rtruediv__, ReflectedTrueDivision, op_rtruediv, native_rtruediv);
        api_test_stub!(binary, self, __rxor__, ReflectedXOr, op_rxor, native_rxor);

        // In place operators
        api_test_stub!(binary, self, __iadd__, InPlaceAdd, op_iadd, native_iadd);
        api_test_stub!(binary, self, __iand__, InPlaceAnd, op_iand, native_iand);
        api_test_stub!(binary, self, __idivmod__, InPlaceDivMod, op_idivmod, native_idivmod);
        api_test_stub!(binary, self, __ifloordiv__, InPlaceFloorDivision, op_ifloordiv, native_ifloordiv);
        api_test_stub!(binary, self, __ilshift__, InPlaceLeftShift, op_ilshift, native_ilshift);
        api_test_stub!(binary, self, __imod__, InPlaceModulus, op_imod, native_imod);
        api_test_stub!(binary, self, __imul__, InPlaceMultiply, op_imul, native_imul);
        api_test_stub!(binary, self, __imatmul__, InPlaceMatrixMultiply, op_imatmul, native_imatmul);
        api_test_stub!(binary, self, __ior__, InPlaceOr, op_ior, native_ior);
        api_test_stub!(ternary, self, __ipow__, InPlacePow, op_ipow, native_ipow);
        api_test_stub!(binary, self, __irshift__, InPlaceRightShift, op_irshift, native_irshift);
        api_test_stub!(binary, self, __isub__, InPlaceSubtract, op_isub, native_isub);
        api_test_stub!(binary, self, __itruediv__, InPlaceTrueDivision, op_itruediv, native_itruediv);
        api_test_stub!(binary, self, __ixor__, InPlaceXOr, op_ixor, native_ixor);

        // Standard unary operators
        api_test_stub!(unary, self, __neg__, Negate, op_neg, native_neg);
        api_test_stub!(unary, self, __abs__, Abs, op_abs, native_abs);
        api_test_stub!(unary, self, __pos__, Positive, op_pos, native_pos);
        api_test_stub!(unary, self, __invert__, Invert, op_invert, native_invert);

        // Standard numeric conversions
        //    api_test_stub!(unary, self, __complex__, ToComplex, op_complex, native_complex);
        //    api_test_stub!(unary, self, __int__, ToInt, op_int, native_int);
        //    api_test_stub!(unary, self, __float__, ToFloat, op_float, native_float);
        //    api_test_stub!(unary, self, __round__, ToRounded, op_round, native_round);
        //    api_test_stub!(unary, self, __index__, ToIndex, op_index, native_index);
    }
}
