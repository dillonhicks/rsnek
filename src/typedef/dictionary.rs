use std::fmt;
use std::ops::Deref;
use std::borrow::Borrow;
use std::cell::RefCell;
use result::{NativeResult, RuntimeResult};

use runtime::{Runtime, IntegerProvider, NoneProvider, BooleanProvider};
use error::Error;
use typedef::objectref::ObjectRef;
use typedef::native::{self, DictKey};
use typedef::builtin::Builtin;

use object::{self, RtValue, typing};
use object::method::{self, Hashed};
use object::selfref::{self, SelfRef};


#[derive(Clone)]
pub struct PyDictType;


impl typing::BuiltinType for PyDictType {
    type T = PyDict;
    type V = native::Dict;

    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        // TODO: Add check for static range
        PyDictType::inject_selfref(PyDictType::alloc(value))
    }

    fn init_type() -> Self {
        PyDictType{}
    }

    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::Dict(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Dict(ref dict) => {
                dict.rc.set(&objref.clone());
            },
            _ => unreachable!()
        }
        new
    }

    fn alloc(value: Self::V) -> Self::T {
        PyDict {
            value: DictValue(RefCell::new(value)),
            rc: selfref::RefCount::default(),
        }
    }

}


pub struct DictValue(pub RefCell<native::Dict>);
pub type PyDict = RtValue<DictValue>;


impl fmt::Debug for PyDict {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value.0.borrow())
    }
}



// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//       New Style Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+



impl object::PyAPI for PyDict {}
impl method::New for PyDict {}
impl method::Init for PyDict {}
impl method::Delete for PyDict {}
impl method::GetAttr for PyDict {}
impl method::GetAttribute for PyDict {}
impl method::SetAttr for PyDict {}
impl method::DelAttr for PyDict {}
impl method::Id for PyDict {}
impl method::Is for PyDict {}
impl method::IsNot for PyDict {}
impl method::Hashed for PyDict {
    #[allow(unused_variables)]
    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        Err(Error::typerr("Unhashable type dict"))
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
        Err(Error::typerr("Unhashable type dict"))
    }
}
impl method::StringCast for PyDict {}
impl method::BytesCast for PyDict {}
impl method::StringFormat for PyDict {}
impl method::StringRepresentation for PyDict {}
impl method::Equal for PyDict {}
impl method::NotEqual for PyDict {}
impl method::LessThan for PyDict {}
impl method::LessOrEqual for PyDict {}
impl method::GreaterOrEqual for PyDict {}
impl method::GreaterThan for PyDict {}
impl method::BooleanCast for PyDict {
    fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
        Ok(if self.native_bool().unwrap() {
               rt.bool(true)
           } else {
               rt.bool(false)
           })
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
        Ok(!self.value.0.borrow().is_empty())
    }
}

impl method::IntegerCast for PyDict {}
impl method::FloatCast for PyDict {}
impl method::ComplexCast for PyDict {}
impl method::Rounding for PyDict {}
impl method::Index for PyDict {}
impl method::NegateValue for PyDict {}
impl method::AbsValue for PyDict {}
impl method::PositiveValue for PyDict {}
impl method::InvertValue for PyDict {}
impl method::Add for PyDict {}
impl method::BitwiseAnd for PyDict {}
impl method::DivMod for PyDict {}
impl method::FloorDivision for PyDict {}
impl method::LeftShift for PyDict {}
impl method::Modulus for PyDict {}
impl method::Multiply for PyDict {}
impl method::MatrixMultiply for PyDict {}
impl method::BitwiseOr for PyDict {}
impl method::Pow for PyDict {}
impl method::RightShift for PyDict {}
impl method::Subtract for PyDict {}
impl method::TrueDivision for PyDict {}
impl method::XOr for PyDict {}
impl method::ReflectedAdd for PyDict {}
impl method::ReflectedBitwiseAnd for PyDict {}
impl method::ReflectedDivMod for PyDict {}
impl method::ReflectedFloorDivision for PyDict {}
impl method::ReflectedLeftShift for PyDict {}
impl method::ReflectedModulus for PyDict {}
impl method::ReflectedMultiply for PyDict {}
impl method::ReflectedMatrixMultiply for PyDict {}
impl method::ReflectedBitwiseOr for PyDict {}
impl method::ReflectedPow for PyDict {}
impl method::ReflectedRightShift for PyDict {}
impl method::ReflectedSubtract for PyDict {}
impl method::ReflectedTrueDivision for PyDict {}
impl method::ReflectedXOr for PyDict {}
impl method::InPlaceAdd for PyDict {}
impl method::InPlaceBitwiseAnd for PyDict {}
impl method::InPlaceDivMod for PyDict {}
impl method::InPlaceFloorDivision for PyDict {}
impl method::InPlaceLeftShift for PyDict {}
impl method::InPlaceModulus for PyDict {}
impl method::InPlaceMultiply for PyDict {}
impl method::InPlaceMatrixMultiply for PyDict {}
impl method::InPlaceBitwiseOr for PyDict {}
impl method::InPlacePow for PyDict {}
impl method::InPlaceRightShift for PyDict {}
impl method::InPlaceSubtract for PyDict {}
impl method::InPlaceTrueDivision for PyDict {}
impl method::InPlaceXOr for PyDict {}
impl method::Contains for PyDict {}
impl method::Iter for PyDict {}
impl method::Call for PyDict {}
impl method::Length for PyDict {
    fn op_len(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_len() {
            Ok(int) => Ok(rt.int(int)),
            Err(_) => unreachable!(),
        }
    }

    fn native_len(&self) -> NativeResult<native::Integer> {
        Ok(native::Integer::from(self.value.0.borrow().len()))
    }
}
impl method::LengthHint for PyDict {}
impl method::Next for PyDict {}
impl method::Reversed for PyDict {}
impl method::GetItem for PyDict {
    /// native getitem now that we have self refs?
    #[allow(unused_variables)]
    fn op_getitem(&self, rt: &Runtime, keyref: &ObjectRef) -> RuntimeResult {
        let key_box: &Box<Builtin> = keyref.0.borrow();
        match key_box.native_hash() {
            Ok(hash) => {
                let key = DictKey(hash, keyref.clone());
                println!("HASHED: {:?}", key);
                match self.value.0.borrow().get(&key) {
                    Some(objref) => Ok(objref.clone()),
                    None => {
                        // TODO: use repr for this as per cPython default
                        Err(Error::key("KeyError: no such key"))
                    }
                }
            }
            Err(_) => Err(Error::typerr("TypeError: Unhashable key type")),
        }
    }

    fn native_getitem(&self, key: &Builtin) -> RuntimeResult {
        match key {
            &Builtin::DictKey(ref key) => {
                match self.value.0.borrow().get(key) {
                    Some(value) => Ok(value.clone()),
                    None => Err(Error::key("No such key"))
                }
            }
            _ => Err(Error::typerr("key is not a dictkey type"))
        }
    }

}

impl method::SetItem for PyDict {
    fn op_setitem(&self, rt: &Runtime, keyref: &ObjectRef, valueref: &ObjectRef) -> RuntimeResult {
        let boxed_key: &Box<Builtin> = keyref.0.borrow();
        match boxed_key.native_hash() {
            Ok(hash) => {
                let key = DictKey(hash, keyref.clone());
                let boxed_value: &Box<Builtin> = valueref.0.borrow();

                match self.native_setitem(&Builtin::DictKey(key), boxed_value) {
                    Ok(_) => Ok(rt.none()),
                    Err(err) => Err(err)
                }
            }
            Err(_) => Err(Error::typerr("TypeError: Unhashable key type")),
        }
    }

    #[allow(unused_variables)]
    fn native_setitem(&self, key: &Builtin, value: &Builtin) -> NativeResult<native::None> {

        let objref = match value.upgrade() {
            Ok(objref) => objref,
            Err(err) => return Err(err)
        };

        match key {
            &Builtin::DictKey(ref key) => {
                self.value.0.borrow_mut().insert(key.clone(), objref);
                Ok(native::None())
            }
            _ => Err(Error::typerr("key is not a dictkey type"))
        }
    }
}
impl method::DeleteItem for PyDict {}
impl method::Count for PyDict {}
impl method::Append for PyDict {}
impl method::Extend for PyDict {}
impl method::Pop for PyDict {}
impl method::Remove for PyDict {}
impl method::IsDisjoint for PyDict {}
impl method::AddItem for PyDict {}
impl method::Discard for PyDict {}
impl method::Clear for PyDict {}
impl method::Get for PyDict {}
impl method::Keys for PyDict {}
impl method::Values for PyDict {}
impl method::Items for PyDict {}
impl method::PopItem for PyDict {}
impl method::Update for PyDict {}
impl method::SetDefault for PyDict {}
impl method::Await for PyDict {}
impl method::Send for PyDict {}
impl method::Throw for PyDict {}
impl method::Close for PyDict {}
impl method::Exit for PyDict {}
impl method::Enter for PyDict {}
impl method::DescriptorGet for PyDict {}
impl method::DescriptorSet for PyDict {}
impl method::DescriptorSetName for PyDict {}




// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//          Tests
// +-+-+-+-+-+-+-+-+-+-+-+-+-+

#[cfg(test)]
mod _api_method {
    use runtime::{StringProvider, DictProvider};
    use object::method::*;
    use super::*;

    fn setup_test() -> (Runtime) {
        Runtime::new()
    }

    #[test]
    fn is_() {
        let rt = setup_test();

        let dict = rt.dict(native::None());
        let dict2 = dict.clone();

        let boxed: &Box<Builtin> = dict.0.borrow();
        let result = boxed.op_is(&rt, &dict2).unwrap();
        assert_eq!(result, rt.bool(true), "BooleanObject is(op_is)");

        let dict3 = rt.dict(native::None());
        let result = boxed.op_is(&rt, &dict3).unwrap();
        assert_eq!(result, rt.bool(false));
    }


    #[test]
    fn __bool__() {
        let rt = setup_test();

        let dict = rt.dict(native::None());
        let boxed: &Box<Builtin> = dict.0.borrow();

        let result = boxed.op_bool(&rt).unwrap();
        assert_eq!(result, rt.bool(false));

        let key = rt.str("helloworld");
        let value = rt.int(1234);
        boxed.op_setitem(&rt, &key, &value).unwrap();

        let result = boxed.op_bool(&rt).unwrap();
        assert_eq!(result, rt.bool(true));
    }

    #[test]
    #[should_panic]
    fn __int__() {
        let rt = setup_test();

        let dict = rt.dict(native::None());
        let boxed: &Box<Builtin> = dict.0.borrow();

        boxed.op_int(&rt).unwrap();
    }

    /// Mutable collection types should not be hashable
    #[test]
    #[should_panic]
    fn __hash__() {
        let rt = setup_test();
        let dict = rt.dict(native::None());
        let boxed: &Box<Builtin> = dict.0.borrow();

        boxed.op_hash(&rt).unwrap();
    }


    #[test]
    fn __setitem__() {
        let rt = setup_test();
        let dict = rt.dict(native::None());

        let key= rt.str("hello");
        let value = rt.int(234);

        let boxed: &Box<Builtin> = dict.0.borrow();

        let result = boxed.op_setitem(&rt, &key, &value).unwrap();
        assert_eq!(result, rt.none());

    }

    #[test]
    fn __getitem__() {
        let rt = setup_test();
        let dict = rt.dict(native::None());

        let key = rt.str("hello");
        let value = rt.int(234);

        let boxed: &Box<Builtin> = dict.0.borrow();

        let result = boxed.op_setitem(&rt, &key, &value).unwrap();
        assert_eq!(result, rt.none());

        println!("{:?}", dict);
        println!("{:?}", key);
        let result = boxed.op_getitem(&rt, &key).unwrap();
        assert_eq!(result, value);
    }

}


#[cfg(all(feature="old", test))]
mod old {
    #[derive(Clone, Debug)]
    pub struct DictionaryObject {
        value: RefCell<Dict>,
    }

    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    //       Struct Traits
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+

    impl DictionaryObject {
        fn new() -> Self {
            return DictionaryObject { value: RefCell::new(Dict::new()) };
        }
    }

    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    //    Python Object Traits
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+
    impl typedef::objectref::RtObject for DictionaryObject {}

    impl object::model::PyObject for DictionaryObject {}

    impl object::model::PyBehavior for DictionaryObject {
        fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
            Err(Error::typerr("Unhashable type dict"))
        }

        fn native_hash(&self) -> NativeResult<native::HashId> {
            Err(Error::typerr("Unhashable type dict"))
        }

        fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
            Ok(if self.native_bool().unwrap() {
                rt.OldTrue()
            } else {
                rt.OldFalse()
            })
        }

        fn native_bool(&self) -> NativeResult<native::Boolean> {
            Ok(!self.value.borrow().is_empty())
        }

        fn op_len(&self, rt: &Runtime) -> RuntimeResult {
            match self.native_len() {
                Ok(int) => rt.alloc(IntegerObject::new_bigint(int).to()),
                Err(_) => unreachable!(),
            }
        }

        fn native_len(&self) -> NativeResult<native::Integer> {
            match Integer::from_usize(self.value.borrow().len()) {
                Some(int) => Ok(int),
                None => Err(Error::value("ValueError converting native integer")),
            }
        }

        fn op_setitem(&self, rt: &Runtime, keyref: &ObjectRef, valueref: &ObjectRef) -> RuntimeResult {
            let key_value: &Box<Builtin> = keyref.borrow();
            match key_value.native_hash() {
                Ok(hash) => {
                    let key = DictKey(hash, keyref.clone());
                    let result = self.value.borrow_mut().insert(key, valueref.clone());
                    //if result.is_some() {Ok(rt.None())} else {Err(Error::runtime("RuntimeError: Cannot add item to dictionary"))}
                    Ok(rt.None())
                }
                Err(err) => Err(Error::typerr("TypeError: Unhashable key type")),
            }
        }

        fn native_setitem(&self, key: &Builtin, value: &Builtin) -> NativeResult<native::NoneValue> {
            // TODO: enforce all objects containing a weakref to itself so we can support
            // the clone and upgrade here for the native map api. Should check if the strong count
            // exists or otherwise return Err because it would mean the value is unmanaged which
            // leads to memory leaks and such.
            Err(Error::not_implemented())
        }

        /// native getitem now that we have self refs?
        fn op_getitem(&self, rt: &Runtime, keyref: &ObjectRef) -> RuntimeResult {
            let key_box: &Box<Builtin> = keyref.borrow();
            match key_box.native_hash() {
                Ok(hash) => {
                    let key = DictKey(hash, keyref.clone());
                    match self.value.borrow().get(&key) {
                        Some(value) => Ok(value.clone()),
                        None => {
                            // TODO: use repr for this as per cPython default
                            Err(Error::key("KeyError: no such key"))
                        }
                    }
                }
                Err(err) => Err(Error::typerr("TypeError: Unhashable key type")),
            }
        }
    }


    impl typedef::objectref::ToRtWrapperType<Builtin> for DictionaryObject {
        fn to(self) -> Builtin {
            Builtin::Dictionary(self)
        }
    }


    impl typedef::objectref::ToRtWrapperType<ObjectRef> for DictionaryObject {
        fn to(self) -> ObjectRef {
            ObjectRef::new(Builtin::Dictionary(self))
        }
    }


    impl fmt::Display for DictionaryObject {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.value)
        }
    }

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

        use super::*;

        /// Call the identity function to make sure it succeeds
        #[test]
        fn identity() {
            let mut rt = Runtime::new(None);
            let empty_dict: ObjectRef = rt.alloc(DictionaryObject::new().to()).unwrap();

            let dict_ref: &Box<Builtin> = empty_dict.0.borrow();

            let result = dict_ref.identity(&rt).unwrap();
        }

        /// Ensure that the dictionary reference equality succeeds with itself
        #[test]
        fn is_() {
            let mut rt = Runtime::new(None);
            let empty_dict: ObjectRef = rt.alloc(DictionaryObject::new().to()).unwrap();

            let dict_ref: &Box<Builtin> = empty_dict.0.borrow();

            let result = dict_ref.op_is(&rt, &empty_dict).unwrap();
            assert_eq!(result, rt.OldTrue());
        }

        /// Ensure that the dictionary reference equality does not
    /// match against two separarte dictionaries.
        #[test]
        fn is_not() {
            let mut rt = Runtime::new(None);
            let empty_dict: ObjectRef = rt.alloc(DictionaryObject::new().to()).unwrap();
            let another_dict: ObjectRef = rt.alloc(DictionaryObject::new().to()).unwrap();

            let dict_ref: &Box<Builtin> = empty_dict.0.borrow();

            let result = dict_ref.op_is_not(&rt, &another_dict).unwrap();
            assert_eq!(result, rt.OldFalse());
        }

        /// Mutable collection types should not be hashable
        #[test]
        #[should_panic]
        fn __hash__() {
            let mut rt = Runtime::new(None);
            let empty_dict: ObjectRef = rt.alloc(DictionaryObject::new().to()).unwrap();

            let dict_ref: &Box<Builtin> = empty_dict.0.borrow();
            let result = dict_ref.op_hash(&rt).unwrap();
        }


        #[test]
        fn __bool__() {
            let mut rt = Runtime::new(None);
            let zero_int: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
            let empty_dict: ObjectRef = rt.alloc(DictionaryObject::new().to()).unwrap();

            let dict_ref: &Box<Builtin> = empty_dict.0.borrow();

            let result = dict_ref.op_len(&rt).unwrap();
            assert_eq!(result, zero_int);
        }



        api_test_stub!(unary, self, __del__, Delete, op_del, native_del);
        api_test_stub!(unary, self, __repr__, ToStringRepr, op_repr, native_repr);
        api_test_stub!(unary, self, __str__, ToString, op_str, native_str);

        /// Called by `bytes()` to compute a byte-string representation of an object.
    /// This should return a bytes object.
        api_test_stub!(unary, self, __bytes__, ToBytes, op_bytes, native_bytes);
        api_test_stub!(binary, self, __format__, Format, op_format, native_format);


        /// The object comparison functions are useful for all objects,
    /// and are named after the rich comparison operators they support:
        api_test_stub!(binary, self, __lt__, LessThan, op_lt, native_lt);
        api_test_stub!(binary, self, __le__, LessOrEqual, op_le, native_le);
        api_test_stub!(binary, self, __eq__, Equal, op_eq, native_eq, native::Boolean);
        api_test_stub!(binary, self, __ne__, NotEqual, op_ne, native_ne, native::Boolean);
        api_test_stub!(binary, self, __ge__, GreaterOrEqual, op_ge, native_ge);
        api_test_stub!(binary, self, __gt__, GreaterThan, op_gt, native_gt);

        // Called by built-in function hash() and for operations on members of hashed collections including
        // set, frozenset, and dict. __hash__() should return an integer. The only required property is
        // that objects which compare equal have the same hash value; it is advised to mix together
        // the hash values of the components of the object that also play a part in comparison
        // of objects by packing them into a tuple and hashing the tuple. Example:
        // api_test_stub!(unary, self, __hash__, Hashable, op_hash, native_hash, native::HashId);

        // Identity operators
        // api_test_stub!(unary, self, identity, Identity, identity, native_identity, native::Boolean);
        // api_test_stub!(unary, self, __bool__, Truth, op_bool, native_bool, native::Boolean);
        api_test_stub!(unary, self, __not__, Not, op_not, native_not, native::Boolean);
        //api_test_stub!(binary, self, is_, Is, op_is, native_is, native::Boolean);
        //api_test_stub!(binary, self, is_not, IsNot, op_is_not, native_is_not, native::Boolean);

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
        api_test_stub!(binary, self, __add__, Add, op_add, native_add);
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
        api_test_stub!(unary, self, __complex__, ToComplex, op_complex, native_complex);
        api_test_stub!(unary, self, __int__, ToInt, op_int, native_int);
        api_test_stub!(unary, self, __float__, ToFloat, op_float, native_float);
        api_test_stub!(unary, self, __round__, ToRounded, op_round, native_round);
        api_test_stub!(unary, self, __index__, ToIndex, op_index, native_index);



        #[cfg(test)]
        mod impl_mappingpybehavior {
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

            use super::*;


            #[test]
            fn __len__() {
                let mut rt = Runtime::new(None);
                let zero: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
                let empty_dict: ObjectRef = rt.alloc(DictionaryObject::new().to()).unwrap();

                let dict_ref: &Box<Builtin> = empty_dict.0.borrow();

                let result = dict_ref.op_len(&rt).unwrap();
                assert_eq!(result, zero);
            }

            #[test]
            fn __setitem__() {
                let mut rt = Runtime::new(None);
                let key: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
                let value: ObjectRef = rt.alloc(StringObject::from_str("Dictionary Value").to()).unwrap();
                let empty_dict: ObjectRef = rt.alloc(DictionaryObject::new().to()).unwrap();

                let dict_ref: &Box<Builtin> = empty_dict.0.borrow();

                let result = dict_ref.op_setitem(&rt, &key, &value).unwrap();
                assert_eq!(result, rt.None());

                let result = dict_ref.op_len(&rt).unwrap();
                assert_eq!(result, rt.OneOld());
            }

            #[test]
            fn __getitem__() {
                let mut rt = Runtime::new(None);
                let key: ObjectRef = rt.alloc(IntegerObject::new_u64(0).to()).unwrap();
                let value: ObjectRef = rt.alloc(StringObject::from_str("Dictionary Value").to()).unwrap();
                let empty_dict: ObjectRef = rt.alloc(DictionaryObject::new().to()).unwrap();

                let dict_ref: &Box<Builtin> = empty_dict.0.borrow();

                let result = dict_ref.op_setitem(&rt, &key, &value).unwrap();
                assert_eq!(result, rt.None());

                let result = dict_ref.op_getitem(&rt, &key).unwrap();
                assert_eq!(value, result);
            }

            // 3.3.6. Emulating container types
            //api_test_stub!(unary, self, __len__, Length, op_len, native_len);
            api_test_stub!(unary, self, __length_hint__, LengthHint, op_length_hint, native_length_hint);
            //api_test_stub!(binary, self, __getitem__, GetItem, op_getitem, native_getitem);
            //api_test_stub!(ternary, self, __setitem__, SetItem, op_setitem, native_setitem);

            api_test_stub!(binary, self, __missing__, MissingItem, op_missing, native_missing);
            api_test_stub!(binary, self, __delitem__, DeleteItem, op_delitem, native_delitem);
            api_test_stub!(unary, self, __iter__, Iterator, op_iter, native_iter);
            api_test_stub!(unary, self, __reversed__, Reverse, op_reverse, native_reverse);
            api_test_stub!(binary, self, __contains__, Contains, op_contains, native_contains);
        }
    }
}