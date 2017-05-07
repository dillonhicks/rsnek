use std::fmt;
use std::ops::Deref;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use ::api::result::Error;
use ::api::result::{ObjectResult, RtResult};
use runtime::Runtime;
use traits::{StringProvider, NoneProvider, IntegerProvider, FunctionProvider};
use ::modules::precondition::{check_kwargs, check_args};
use ::api::method::*;
use api::{self, RtValue, typing};
use api::method::{self, Id, Hashed};
use api::selfref::{self, SelfRef};
use api::typing::BuiltinType;

use ::resources::strings;
use objects::dictionary::PyDictType;
use objects::tuple::PyTupleType;
use objects::builtin::Builtin;
use objects::native::{self, WrapperFn, Signature, FuncType, SignatureBuilder};
use objects::object::PyObjectType;
use ::api::RtObject;


const TYPE_NAME: &'static str = "builtin_function_or_method";


pub struct PyFunctionType {
    pub function_type: RtObject,
}

impl PyFunctionType {
    pub fn init_type(typeref: &RtObject, object: &RtObject) -> Self {

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
    fn new(&self, rt: &Runtime, value: Self::V) -> RtObject {
        PyFunctionType::inject_selfref(PyFunctionType::alloc(value))
    }

    fn init_type() -> Self {
        unimplemented!()
    }

    fn inject_selfref(value: Self::T) -> RtObject {
        let object = RtObject::new(Builtin::Function(value));
        let new = object.clone();

        match object.as_ref() {
            &Builtin::Function(ref func) => {
                func.rc.set(&object.clone());
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

    pub fn type_name(&self) -> &str {
        match self.value.0.callable {
            FuncType::MethodWrapper(_, _) => "method-wrapper",
            FuncType::Wrapper(_) => TYPE_NAME,
            FuncType::Code(_) => "function"
        }
    }

    /* Missing
        [('__ceil__', <function int.__ceil__>),
        ('__class__', int),
        ('__dir__', <function int.__dir__>),
        ('__floor__', <function int.__floor__>),
        ('__format__', <function int.__format__>),
        ('__getnewargs__', <function int.__getnewargs__>),
        ('__init_subclass__', <function int.__init_subclass__>),
        ('__new__', <function int.__new__>),
        ('__reduce__', <function int.__reduce__>),
        ('__reduce_ex__', <function int.__reduce_ex__>),
        ('__round__', <function int.__round__>),
        ('__sizeof__', <function int.__sizeof__>),
        ('__subclasshook__', <function int.__subclasshook__>),
        ('__trunc__', <function int.__trunc__>),
        ('bit_length', <function int.bit_length>),
        ('conjugate', <function int.conjugate>),
        ('denominator', 1),
        ('from_bytes', <function int.from_bytes>),
        ('imag', 0),
        ('numerator', 1),
        ('real', 1),
        ('to_bytes', <function int.to_bytes>)]
    */
    pub fn get_attribute(&self, rt: &Runtime, name: &str) -> ObjectResult {
        match name {
            "__doc__"           => self.try_get_name(rt, name),
            "__abs__"           |
            "__bool__"          |
            "__float__"         |
            "__hash__"          |
            "__index__"         |
            "__int__"           |
            "__invert__"        |
            "__neg__"           |
            "__pos__"           |
            "__repr__"          |
            "__str__"           => self.try_get_unary_method(rt, name),
            "__add__"           |
            "__and__"           |
            "__delattr__"       |
            "__divmod__"        |
            "__eq__"            |
            "__floordiv__"      |
            "__ge__"            |
            "__getattribute__"  |
            "__gt__"            |
            "__le__"            |
            "__lshift__"        |
            "__lt__"            |
            "__mod__"           |
            "__mul__"           |
            "__ne__"            |
            "__or__"            |
            "__radd__"          |
            "__rand__"          |
            "__rdivmod__"       |
            "__rfloordiv__"     |
            "__rlshift__"       |
            "__rmod__"          |
            "__rmul__"          |
            "__ror__"           |
            "__rrshift__"       |
            "__rshift__"        |
            "__rsub__"          |
            "__rtruediv__"      |
            "__rxor__"          |
            "__sub__"           |
            "__truediv__"       |
            "__xor__"           => self.try_get_binary_method(rt, name),
            "__pow__"           |
            "__rpow__"          |
            "__setattr__"       => self.try_get_ternary_method(rt, name),
            missing => return Err(Error::attribute(
                &strings_error_no_attribute!(self.type_name(), missing)))
        }
    }

    fn try_get_name(&self, rt: &Runtime, name: &str) -> ObjectResult {
        match name {
            "__doc__" => Ok(rt.str(strings::INT_DOC_STRING)),
            missing => Err(Error::attribute(
                &strings_error_no_attribute!(self.type_name(), missing)))
        }
    }

    fn try_get_unary_method(&self, rt: &Runtime, name: &str) -> ObjectResult {
        let func = match name {
            "__abs__"       => {PyFunction::op_abs},
            "__bool__"      => {PyFunction::op_bool},
            "__float__"     => {PyFunction::op_float},
            "__hash__"      => {PyFunction::op_hash},
            "__index__"     => {PyFunction::op_index},
            "__int__"       => {PyFunction::op_int},
            "__invert__"    => {PyFunction::op_invert},
            "__neg__"       => {PyFunction::op_neg},
            "__pos__"       => {PyFunction::op_pos},
            "__repr__"      => {PyFunction::op_repr},
            "__str__"       => {PyFunction::op_str},
            missing => return Err(Error::attribute(
                &strings_error_no_attribute!(self.type_name(), missing)))
        };

        unary_method_wrapper!(self, self.type_name(), name, rt, Builtin::Function, func)
    }

    fn try_get_binary_method(&self, rt: &Runtime, name: &str) -> ObjectResult {
        let func = match name {
            "__add__"          => {PyFunction::op_add},
            "__and__"          => {PyFunction::op_and},
            "__delattr__"      => {PyFunction::op_delattr},
            "__divmod__"       => {PyFunction::op_divmod},
            "__eq__"           => {PyFunction::op_eq},
            "__floordiv__"     => {PyFunction::op_floordiv},
            "__ge__"           => {PyFunction::op_ge},
            "__getattribute__" => {PyFunction::op_getattribute},
            "__gt__"           => {PyFunction::op_gt},
            "__le__"           => {PyFunction::op_le},
            "__lshift__"       => {PyFunction::op_lshift},
            "__lt__"           => {PyFunction::op_lt},
            "__mod__"          => {PyFunction::op_mod},
            "__mul__"          => {PyFunction::op_mul},
            "__ne__"           => {PyFunction::op_ne},
            "__or__"           => {PyFunction::op_or},
            "__radd__"         => {PyFunction::op_radd},
            "__rand__"         => {PyFunction::op_rand},
            "__rdivmod__"      => {PyFunction::op_rdivmod},
            "__rfloordiv__"    => {PyFunction::op_rfloordiv},
            "__rlshift__"      => {PyFunction::op_rlshift},
            "__rmod__"         => {PyFunction::op_rmod},
            "__rmul__"         => {PyFunction::op_rmul},
            "__ror__"          => {PyFunction::op_ror},
            "__rrshift__"      => {PyFunction::op_rrshift},
            "__rshift__"       => {PyFunction::op_rshift},
            "__rsub__"         => {PyFunction::op_rsub},
            "__rtruediv__"     => {PyFunction::op_rtruediv},
            "__rxor__"         => {PyFunction::op_rxor},
            "__sub__"          => {PyFunction::op_sub},
            "__truediv__"      => {PyFunction::op_truediv},
            "__xor__"          => {PyFunction::op_xor},
            missing => return Err(Error::attribute(
                &strings_error_no_attribute!(self.type_name(), missing)))
        };

        binary_method_wrapper!(self, self.type_name(), name, rt, Builtin::Function, func)
    }

    fn try_get_ternary_method(&self, rt: &Runtime, name: &str) -> ObjectResult {
        let func = match name {
            "__pow__"          => {PyFunction::op_pow},
            //"__rpow__"         => {PyFunction::op_rpow},
            "__setattr__"      => {PyFunction::op_setattr},
            missing => return Err(Error::attribute(
                &strings_error_no_attribute!(self.type_name(), missing)))
        };

        ternary_method_wrapper!(self, self.type_name(), name, rt, Builtin::Function, func)
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


impl api::PyAPI for PyFunction {}


/// `self.rhs`
impl method::GetAttr for PyFunction {
    fn op_getattr(&self, rt: &Runtime, name: &RtObject) -> ObjectResult {

        match name.as_ref() {
            &Builtin::Str(ref pystring) => {
                let string = pystring.value.0.clone();
                self.get_attribute(&rt, &string)
            }
            other => Err(Error::typerr(&format!(
                "getattr <{}>' requires string for attribute names, not {}",
                TYPE_NAME,
                other.debug_name())))
        }
    }
}


impl method::Id for PyFunction {
    // TODO: {T104} why do we have to go back through the builtin? Is there a good reason to
    //  special case this at the builtin.rs layer?
    fn native_id(&self) -> native::ObjectId {
        match self.rc.upgrade() {
            Ok(this_object) => {
                this_object.native_id()
            }
            Err(_) => 0,
        }
    }
}

impl method::Hashed for PyFunction {
    fn op_hash(&self, rt: &Runtime) -> ObjectResult {
        let value = self.native_hash()?;
        Ok(rt.int(value))
    }

    fn native_hash(&self) -> RtResult<native::HashId> {
        let mut s = DefaultHasher::new();
        self.native_id().hash(&mut s);
        Ok(s.finish())
    }
}

impl method::StringCast for PyFunction {
    fn op_str(&self, rt: &Runtime) -> ObjectResult {
        let value = self.native_str()?;
        Ok(rt.str(value))
    }

    fn native_str(&self) -> RtResult<native::String> {
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
    fn native_eq(&self, other: &Builtin) -> RtResult<native::Boolean> {
        Ok(self.native_id() == other.native_id())
    }
}

impl method::Call for PyFunction {
    fn op_call(&self, rt: &Runtime, pos_args: &RtObject, star_args: &RtObject, kwargs: &RtObject) -> ObjectResult {
        match self.value.0.callable {
            FuncType::MethodWrapper(_, ref func) => func(&rt, &pos_args, &star_args, &kwargs),
            FuncType::Wrapper(ref func) => func(&rt, &pos_args, &star_args, &kwargs),
            FuncType::Code(_) => {
                Err(Error::typerr("'code' object is not callable"))
            }
        }
    }

    #[allow(unused_variables)]
    fn native_call(&self, named_args: &Builtin, args: &Builtin, kwargs: &Builtin) -> RtResult<Builtin> {
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
    use api::method::*;
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
