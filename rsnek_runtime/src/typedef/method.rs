use std::fmt;
use std::ops::Deref;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use error::Error;
use result::{RuntimeResult, NativeResult};
use runtime::Runtime;
use traits::{StringProvider, NoneProvider, IntegerProvider, FunctionProvider};
use builtin::precondition::check_fnargs_rt;
use object::{self, RtValue, typing};
use object::method::{self, Id, Hashed};
use object::selfref::{self, SelfRef};
use object::typing::BuiltinType;

use ::resource::strings;
use typedef::dictionary::PyDictType;
use typedef::tuple::PyTupleType;
use typedef::builtin::Builtin;
use typedef::native::{self, WrapperFn, Signature, FuncType, SignatureBuilder};
use typedef::object::PyObjectType;
use ::object::RtObject as ObjectRef;


pub struct PyFunctionType {
    pub function_type: ObjectRef,
}

impl PyFunctionType {
    pub fn init_type(typeref: &ObjectRef, object: &ObjectRef) -> Self {

        let method = PyObjectType::inject_selfref(PyObjectType::alloc(
            native::Object {
                class: typeref.clone(),
                dict: PyDictType::inject_selfref(PyDictType::alloc(native::Dict::new())),
                bases: PyTupleType::inject_selfref(PyTupleType::alloc(vec![object.clone()])),
            }));

        PyFunctionType { function_type: method }
    }
}

impl typing::BuiltinType for PyFunctionType {
    type T = PyFunction;
    type V = native::Func;

    #[inline(always)]
    #[allow(unused_variables)]
    fn new(&self, rt: &Runtime, value: Self::V) -> ObjectRef {
        PyFunctionType::inject_selfref(PyFunctionType::alloc(value))
    }

    fn init_type() -> Self {
        unimplemented!()
    }

    fn inject_selfref(value: Self::T) -> ObjectRef {
        let objref = ObjectRef::new(Builtin::Function(value));
        let new = objref.clone();

        let boxed: &Box<Builtin> = objref.0.borrow();
        match boxed.deref() {
            &Builtin::Function(ref object) => {
                object.rc.set(&objref.clone());
            }
            _ => unreachable!(),
        }
        new
    }

    fn alloc(object: Self::V) -> Self::T {
        PyFunction {
            value: FuncValue(object),
            rc: selfref::RefCount::default(),
        }
    }
}

pub struct FuncValue(pub native::Func);
pub type PyFunction = RtValue<FuncValue>;


impl PyFunction {
    pub fn name(&self) -> &str {
        &self.value.0.name
    }

    pub fn module(&self) -> &str {
        &self.value.0.module
    }

    #[allow(unused_variables)]
    fn call_wrapper(&self,
                    rt: &Runtime,
                    callable: &Box<WrapperFn>,
                    signature: &Signature,
                    pos_args: &ObjectRef,
                    star_args: &ObjectRef,
                    kwargs: &ObjectRef)
                    -> RuntimeResult {

        let boxed: &Box<Builtin> = pos_args.0.borrow();
        match boxed.deref() {
            &Builtin::Tuple(_) => {}
            _ => return Err(Error::typerr("Expected type tuple for pos_args")),
        };

        let boxed: &Box<Builtin> = star_args.0.borrow();
        match boxed.deref() {
            &Builtin::Tuple(_) => {}
            _ => return Err(Error::typerr("Expected type tuple for *args")),
        };

        let boxed: &Box<Builtin> = kwargs.0.borrow();
        match boxed.deref() {
            &Builtin::Dict(_) => {}
            _ => return Err(Error::typerr("Expected type tuple for **args")),
        };

        callable(&rt, &pos_args, &star_args, &kwargs)
    }
}


impl fmt::Display for PyFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Method")
    }
}

impl fmt::Debug for PyFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Method")
    }
}


impl object::PyAPI for PyFunction {}


impl method::GetAttr for PyFunction {
    fn op_getattr(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
        let boxed: &Box<Builtin> = name.0.borrow();
        match boxed.deref() {
            &Builtin::Str(ref pystring) => {
                let selfref = self.rc.upgrade()?;
                #[allow(unused_variables)]
                let callable: Box<native::WrapperFn> = Box::new(move |rt, pos_args, starargs, kwargs| {
                    let b: &Box<Builtin> = selfref.0.borrow();
                    Ok(rt.int(b.native_hash()?))
                });

                match pystring.value.0.as_str() {
                    "__hash__" => {
                        Ok(rt.function(native::Func {
                            name: "method __hash__".to_string(),
                            signature: [].as_args(),
                            module: strings::BUILTINS_MODULE.to_string(),
                            callable: native::FuncType::Wrapper(callable)
                        }))
                    }
                    other => Err(Error::name(other))
                }

            }
            other => Err(Error::typerr(&format!(
                "getattr <int>' requires string for attribute names, not {}",
                other.debug_name())))
        }
    }
}


impl method::Id for PyFunction {
    // TODO: {T104} why do we have to go back through the builtin? Is there a good reason to
    //  special case this at the builtin.rs layer?
    fn native_id(&self) -> native::ObjectId {
        match self.rc.upgrade() {
            Ok(objref) => {
                let boxed: &Box<Builtin> = objref.0.borrow();
                boxed.native_id()
            }
            Err(_) => 0,
        }
    }
}

impl method::Hashed for PyFunction {
    fn op_hash(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_hash() {
            Ok(hashid) => Ok(rt.int(hashid)),
            Err(err) => Err(err),
        }
    }

    fn native_hash(&self) -> NativeResult<native::HashId> {
        let mut s = DefaultHasher::new();
        self.native_id().hash(&mut s);
        Ok(s.finish())
    }
}

impl method::StringCast for PyFunction {
    fn op_str(&self, rt: &Runtime) -> RuntimeResult {
        match self.native_str() {
            Ok(string) => Ok(rt.str(string)),
            Err(err) => Err(err)
        }
    }

    fn native_str(&self) -> NativeResult<native::String> {
        let name = match self.value.0.callable {
            FuncType::Wrapper(_) => format!("<builtin-function {}>", self.value.0.name),
            FuncType::MethodWrapper(ref objref, _) => {
                format!("<method-wrapper {} at 0x{:x}>", self.value.0.name, self.rc.upgrade()?.id())
            },
            FuncType::Code(_) =>format!("<function {}>", self.value.0.name),
        };

        Ok(name)
    }
}

impl method::Equal for PyFunction {
    fn native_eq(&self, other: &Builtin) -> NativeResult<native::Boolean> {
        Ok(self.native_id() == other.native_id())
    }
}

impl method::Call for PyFunction {
    fn op_call(&self, rt: &Runtime, pos_args: &ObjectRef, star_args: &ObjectRef, kwargs: &ObjectRef) -> RuntimeResult {
        match self.value.0.callable {
            FuncType::MethodWrapper(_, ref func) |
            FuncType::Wrapper(ref func) => self.call_wrapper(
                &rt, func, &self.value.0.signature, &pos_args, &star_args, &kwargs),
            FuncType::Code(_) => {
                Err(Error::typerr("'code' object is not callable"))
            }
        }
    }

    #[allow(unused_variables)]
    fn native_call(&self, named_args: &Builtin, args: &Builtin, kwargs: &Builtin) -> NativeResult<Builtin> {
        Err(Error::system_not_implemented("PyFunction::native_call()",
                                          &format!("file: {}, line: {}", file!(), line!())))
    }
}


method_not_implemented!(PyFunction,
    AbsValue   Add   AddItem   Append
    Await   BitwiseAnd   BitwiseOr   BooleanCast
    BytesCast  Clear   Close   ComplexCast   Contains   Count   DelAttr
    Delete   DeleteItem   DescriptorGet   DescriptorSet
    DescriptorSetName   Discard   DivMod   Enter
    Exit   Extend   FloatCast FloorDivision   Get   GetAttribute
    GetItem   GreaterOrEqual   GreaterThan  InPlaceAdd   InPlaceBitwiseAnd   InPlaceBitwiseOr
    InPlaceDivMod   InPlaceFloorDivision   InPlaceLeftShift   InPlaceMatrixMultiply
    InPlaceModulus   InPlaceMultiply   InPlacePow   InPlaceRightShift
    InPlaceSubtract   InPlaceTrueDivision   InPlaceXOr   Index
    Init   IntegerCast   InvertValue   Is
    IsDisjoint   IsNot   Items   Iter   Keys   LeftShift   Length   LengthHint
    LessOrEqual   LessThan   MatrixMultiply   Modulus
    Multiply   NegateValue   New   Next   NotEqual   Pop   PopItem   PositiveValue
    Pow   ReflectedAdd   ReflectedBitwiseAnd   ReflectedBitwiseOr
    ReflectedDivMod   ReflectedFloorDivision   ReflectedLeftShift   ReflectedMatrixMultiply
    ReflectedModulus   ReflectedMultiply   ReflectedPow   ReflectedRightShift
    ReflectedSubtract   ReflectedTrueDivision   ReflectedXOr   Remove
    Reversed   RightShift   Rounding   Send   SetAttr   SetDefault   SetItem
    StringFormat   StringRepresentation   Subtract   Throw
    TrueDivision   Update   Values   XOr
);


#[cfg(test)]
mod tests {
    use traits::{FunctionProvider, BooleanProvider, NoneProvider, DictProvider, TupleProvider};
    use object::method::*;
    use super::*;

    fn setup_test() -> (Runtime) {
        Runtime::new()
    }

    #[test]
    fn is_() {
        let rt = setup_test();
        let func = rt.function(native::None());
        let func2 = func.clone();
        let func3 = rt.function(native::None());
        
        let result = func.op_is(&rt, &func2).unwrap();
        assert_eq!(result, rt.bool(true));

        let result = func.op_is(&rt, &func3).unwrap();
        assert_eq!(result, rt.bool(false));
    }


    #[test]
    fn is_not() {
        let rt = setup_test();
        let func = rt.function(native::None());
        let func2 = func.clone();
        let func3 = rt.function(native::None());

        let result = func.op_is_not(&rt, &func2).unwrap();
        assert_eq!(result, rt.bool(false));

        let result = func.op_is_not(&rt, &func3).unwrap();
        assert_eq!(result, rt.bool(true));
    }


    #[test]
    fn __call__() {
        let rt = setup_test();
        let func = rt.function(native::None());

        let pos_args = rt.tuple(native::None());
        let starargs = rt.tuple(native::None());
        let kwargs = rt.dict(native::None());

        let result = func.op_call(&rt, &pos_args, &starargs, &kwargs).unwrap();
        assert_eq!(result, rt.none());
    }

}
