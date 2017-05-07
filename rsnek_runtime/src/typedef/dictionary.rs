use std::fmt;
use std::ops::Deref;
use std::borrow::Borrow;
use std::cell::RefCell;

use ::error::Error;
use ::api::method::{self, Hashed, StringRepresentation};
use ::api::RtObject;
use ::api::selfref::{self, SelfRef};
use ::api::{self, RtValue, typing};
use ::result::{RtResult, ObjectResult};
use ::runtime::Runtime;
use ::traits::{IntegerProvider, NoneProvider, BooleanProvider, TupleProvider};
use ::typedef::builtin::Builtin;
use ::typedef::native::{self, DictKey};


const TYPE_NAME: &'static str = "dict";


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
        let object = RtObject::new(Builtin::Dict(value));
        let new = object.clone();

        match object.as_ref() {
            &Builtin::Dict(ref dict) => {
                dict.rc.set(&object.clone());
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


impl api::PyAPI for PyDict {}


impl method::Hashed for PyDict {
    #[allow(unused_variables)]
    fn op_hash(&self, rt: &Runtime) -> ObjectResult {
        Err(Error::typerr(&format!("Unhashable type {}", TYPE_NAME)))
    }

    fn native_hash(&self) -> RtResult<native::HashId> {
        Err(Error::typerr(&format!("Unhashable type {}", TYPE_NAME)))
    }
}

impl method::StringCast for PyDict {
    fn native_str(&self) -> RtResult<native::String> {
        let mut strings: Vec<String> = Vec::new();

        for (key_wrapper, value) in self.value.0.borrow().iter() {

            let key = &key_wrapper.1;
            let ks = match key.native_repr() {
                Ok(s) => s,
                Err(_) => format!("{:?}", key)
            };

            let vs = match value.native_repr() {
                Ok(s) => s,
                Err(_) => format!("{:?}", value)
            };

            strings.push([ks, vs].join(": "));
        }

        Ok(format!("{{{}}}", strings.join(", ")))
    }
}


impl method::BooleanCast for PyDict {
    fn op_bool(&self, rt: &Runtime) -> ObjectResult {
        match self.native_bool() {
            Ok(value) => Ok(rt.bool(value)),
            Err(err) => Err(err)
        }
    }

    fn native_bool(&self) -> RtResult<native::Boolean> {
        Ok(!self.value.0.borrow().is_empty())
    }
}


impl method::Length for PyDict {
    fn op_len(&self, rt: &Runtime) -> ObjectResult {
        match self.native_len() {
            Ok(int) => Ok(rt.int(int)),
            Err(_) => unreachable!(),
        }
    }

    fn native_len(&self) -> RtResult<native::Integer> {
        Ok(native::Integer::from(self.value.0.borrow().len()))
    }
}

impl method::GetItem for PyDict {
    #[allow(unused_variables)]
    fn op_getitem(&self, rt: &Runtime, key: &RtObject) -> ObjectResult {
        match key.native_hash() {
            Ok(hash) => {
                let key_wrapper = DictKey(hash, key.clone());
                match self.value.0.borrow().get(&key_wrapper) {
                    Some(objref) => Ok(objref.clone()),
                    None => {
                        Err(Error::key(&format!("KeyError: {:?}", key)))
                    }
                }
            }
            Err(_) => Err(Error::typerr("TypeError: Unhashable key type")),
        }
    }

    fn native_getitem(&self, key: &Builtin) -> ObjectResult {
        match key {
            &Builtin::DictKey(ref key) => {
                match self.value.0.borrow().get(key) {
                    Some(value) => Ok(value.clone()),
                    None =>  Err(Error::key(&format!("KeyError: {:?}", key))),
                }
            }
            _ => Err(Error::typerr("key is not a dictkey type")),
        }
    }
}

impl method::SetItem for PyDict {
    fn op_setitem(&self, rt: &Runtime, key: &RtObject, value: &RtObject) -> ObjectResult {
        match key.native_hash() {
            Ok(hash) => {
                let key_wrapper = Builtin::DictKey(DictKey(hash, key.clone()));

                match self.native_setitem(&key_wrapper, value.as_ref()) {
                    Ok(_) => Ok(rt.none()),
                    Err(err) => Err(err),
                }
            }
            Err(_) => Err(Error::typerr("TypeError: Unhashable key type")),
        }
    }

    #[allow(unused_variables)]
    fn native_setitem(&self, key: &Builtin, value: &Builtin) -> RtResult<native::None> {
        match key {
            &Builtin::DictKey(ref key) => {
                self.value.0.borrow_mut().insert(key.clone(), value.upgrade()?);
                Ok(native::None())
            }
            _ => Err(Error::typerr("key is not a dictkey type")),
        }
    }
}

impl method::Keys for PyDict {

    fn meth_keys(&self, rt: &Runtime) -> ObjectResult {
        let keys = self.native_meth_keys()?;
        Ok(rt.tuple(keys))
    }

    fn native_meth_keys(&self) -> RtResult<native::Tuple> {
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
    use api::method::*;
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

        let result = dict.op_bool(&rt).unwrap();
        assert_eq!(result, rt.bool(false));

        let key = rt.str("helloworld");
        let value = rt.int(1234);
        dict.op_setitem(&rt, &key, &value).unwrap();

        let result = dict.op_bool(&rt).unwrap();
        assert_eq!(result, rt.bool(true));
    }

    #[test]
    #[should_panic]
    fn __int__() {
        let rt = setup_test();
        
        let dict = rt.dict(native::None());
        dict.op_int(&rt).unwrap();
    }

    /// Mutable collection types should not be hashable
    #[test]
    #[should_panic]
    fn __hash__() {
        let rt = setup_test();
        let dict = rt.dict(native::None());

        dict.op_hash(&rt).unwrap();
    }


    #[test]
    fn __setitem__() {
        let rt = setup_test();
        let dict = rt.dict(native::None());

        let key = rt.str("hello");
        let value = rt.int(234);

        let result = dict.op_setitem(&rt, &key, &value).unwrap();
        assert_eq!(result, rt.none());

    }

    #[test]
    fn __getitem__() {
        let rt = setup_test();
        let dict = rt.dict(native::None());

        let key = rt.str("hello");
        let value = rt.int(234);


        let result = dict.op_setitem(&rt, &key, &value).unwrap();
        assert_eq!(result, rt.none());

        info!("{:?}", dict);
        info!("{:?}", key);
        let result = dict.op_getitem(&rt, &key).unwrap();
        assert_eq!(result, value);
    }

}
