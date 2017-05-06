use std::fmt;
use std::ops::Deref;
use std::borrow::Borrow;
use std::cell::RefCell;


use result::{NativeResult, RuntimeResult};
use runtime::Runtime;
use traits::{IntegerProvider, NoneProvider, BooleanProvider, TupleProvider};
use error::Error;
use ::object::RtObject;
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
    fn new(&self, rt: &Runtime, value: Self::V) -> RtObject {
        PyDictType::inject_selfref(PyDictType::alloc(value))
    }

    fn init_type() -> Self {
        PyDictType {}
    }

    fn inject_selfref(value: Self::T) -> RtObject {
        let objref = RtObject::new(Builtin::Dict(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Dict(ref dict) => {
                dict.rc.set(&objref.clone());
            }
            _ => unreachable!(),
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


impl object::PyAPI for PyDict {}


impl method::Hashed for PyDict {
    #[allow(unused_variables)]
    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        Err(Error::typerr("Unhashable type dict"))
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
        Err(Error::typerr("Unhashable type dict"))
    }
}
impl method::StringCast for PyDict {
    fn native_str(&self) -> NativeResult<native::String> {
        let mut strings: Vec<String> = Vec::new();

        for (key, value) in self.value.0.borrow().iter() {

            let keyobj = &key.1;
            let boxed: &Box<Builtin> = keyobj.0.borrow();
            let ks = match boxed.native_str() {
                Ok(s) => s,
                Err(_) => format!("{}", boxed)
            };

            let boxed: &Box<Builtin> = value.0.borrow();
            let vs = match boxed.native_str() {
                Ok(s) => s,
                Err(_) => format!("{}", boxed)
            };

            strings.push([ks, vs].join(": "));
        }

        Ok(format!("{{{}}}", strings.join(", ")))
    }
}


impl method::BooleanCast for PyDict {
    fn op_bool(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_bool() {
            Ok(value) => Ok(rt.bool(value)),
            Err(err) => Err(err)
        }
    }

    fn native_bool(&self) -> NativeResult<native::Boolean> {
        Ok(!self.value
                .0
                .borrow()
                .is_empty())
    }
}


impl method::Length for PyDict {
    fn op_len(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_len() {
            Ok(int) => Ok(rt.int(int)),
            Err(_) => unreachable!(),
        }
    }

    fn native_len(&self) -> NativeResult<native::Integer> {
        Ok(native::Integer::from(self.value
                                     .0
                                     .borrow()
                                     .len()))
    }
}

impl method::GetItem for PyDict {
    /// native getitem now that we have self refs?
    #[allow(unused_variables)]
    fn op_getitem(&self, rt: &Runtime, keyref: &RtObject) -> RuntimeResult {
        let key_box: &Box<Builtin> = keyref.0.borrow();
        match key_box.native_hash() {
            Ok(hash) => {
                let key = DictKey(hash, keyref.clone());
                println!("HASHED: {:?}", key);
                match self.value
                          .0
                          .borrow()
                          .get(&key) {
                    Some(objref) => Ok(objref.clone()),
                    None => {
                        Err(Error::key(&format!("KeyError: {:?}", keyref.to_string())))
                    }
                }
            }
            Err(_) => Err(Error::typerr("TypeError: Unhashable key type")),
        }
    }

    fn native_getitem(&self, key: &Builtin) -> RuntimeResult {
        match key {
            &Builtin::DictKey(ref key) => {
                match self.value
                          .0
                          .borrow()
                          .get(key) {
                    Some(value) => Ok(value.clone()),
                    None => Err(Error::key("No such key")),
                }
            }
            _ => Err(Error::typerr("key is not a dictkey type")),
        }
    }
}

impl method::SetItem for PyDict {
    fn op_setitem(&self, rt: &Runtime, keyref: &RtObject, valueref: &RtObject) -> RuntimeResult {
        let boxed_key: &Box<Builtin> = keyref.0.borrow();
        match boxed_key.native_hash() {
            Ok(hash) => {
                let key = DictKey(hash, keyref.clone());
                let boxed_value: &Box<Builtin> = valueref.0.borrow();

                match self.native_setitem(&Builtin::DictKey(key), boxed_value) {
                    Ok(_) => Ok(rt.none()),
                    Err(err) => Err(err),
                }
            }
            Err(_) => Err(Error::typerr("TypeError: Unhashable key type")),
        }
    }

    #[allow(unused_variables)]
    fn native_setitem(&self, key: &Builtin, value: &Builtin) -> NativeResult<native::None> {

        let objref = match value.upgrade() {
            Ok(objref) => objref,
            Err(err) => return Err(err),
        };

        match key {
            &Builtin::DictKey(ref key) => {
                self.value
                    .0
                    .borrow_mut()
                    .insert(key.clone(), objref);
                Ok(native::None())
            }
            _ => Err(Error::typerr("key is not a dictkey type")),
        }
    }
}

impl method::Keys for PyDict {

    fn meth_keys(&self, rt: &Runtime) -> RuntimeResult {
        let keys = self.native_meth_keys()?;
        Ok(rt.tuple(keys))
    }

    fn native_meth_keys(&self) -> NativeResult<native::Tuple> {
        let keys = self.value.0.borrow().keys()
            .map(|key| key.value())
            .collect::<native::Tuple>();

        Ok(keys)
    }
}


method_not_implemented!(PyDict,
    AbsValue   Add   AddItem   Append   
    Await   BitwiseAnd   BitwiseOr   BytesCast   
    Call   Clear   Close   ComplexCast   
    Contains   Count   DelAttr   Delete   
    DeleteItem   DescriptorGet   DescriptorSet   DescriptorSetName   
    Discard   DivMod   Enter   Equal   
    Exit   Extend   FloatCast   FloorDivision   
    Get   GetAttr   GetAttribute   GreaterOrEqual   
    GreaterThan   Id   Index   Init   
    InPlaceAdd   InPlaceBitwiseAnd   InPlaceBitwiseOr   InPlaceDivMod   
    InPlaceFloorDivision   InPlaceLeftShift   InPlaceMatrixMultiply   InPlaceModulus   
    InPlaceMultiply   InPlacePow   InPlaceRightShift   InPlaceSubtract   
    InPlaceTrueDivision   InPlaceXOr   IntegerCast   InvertValue   
    Is   IsDisjoint   IsNot   Items   
    Iter   LeftShift   LengthHint   LessOrEqual   
    LessThan   MatrixMultiply   Modulus   Multiply   
    NegateValue   New   Next   NotEqual   
    Pop   PopItem   PositiveValue   Pow   
    ReflectedAdd   ReflectedBitwiseAnd   ReflectedBitwiseOr   ReflectedDivMod   
    ReflectedFloorDivision   ReflectedLeftShift   ReflectedMatrixMultiply   ReflectedModulus   
    ReflectedMultiply   ReflectedPow   ReflectedRightShift   ReflectedSubtract   
    ReflectedTrueDivision   ReflectedXOr   Remove   Reversed   
    RightShift   Rounding   Send   SetAttr   
    SetDefault   StringFormat   StringRepresentation   Subtract   
    Throw   TrueDivision   Update   Values   
    XOr
);


#[cfg(test)]
mod tests {
    use traits::{StringProvider, DictProvider};
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

        let result = dict.op_is(&rt, &dict2).unwrap();
        assert_eq!(result, rt.bool(true));

        let dict3 = rt.dict(native::None());
        let result = dict.op_is(&rt, &dict3).unwrap();
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

        let key = rt.str("hello");
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
