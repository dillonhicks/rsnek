use std::borrow::Borrow;

use error::Error;
use result::{NativeResult, RuntimeResult};
use runtime::Runtime;
use object::{self, RtValue};
use object::method;
use object::attribute;

use typedef::dictionary::DictionaryObject;
use typedef::native;
use typedef::builtin::Builtin;
use typedef::objectref::ObjectRef;
use typedef::string::StringObject;



//struct PyNativeFunction {
//
//}
//
//struct PyMethodWrapper {
//
//}
//
//h struct PyFunction {
//    code: (),
//    args: (),
//    locals: (),
//    consts: ()
//}
//
//enum Function {
//    Builtin(PyNativeFunction),
//
//}
//
//struct PyMethod {
//
//}


pub struct ObjectValue {
    dict: ObjectRef,
}

// ref to dictionary
pub type PyObject = RtValue<ObjectValue>;


// +-+-+-+-+-+-+-+-+-+-+-+-+-+
//    Python Object Traits
// +-+-+-+-+-+-+-+-+-+-+-+-+-+
macro_rules! resolve_inner {
    ($out:ident, $objref:expr, $otype:ty, $resolver:ident) => (
        let boxed: &Box<Builtin> = $objref.borrow();
        let $out: $otype = boxed.$resolver().unwrap();
    )
}

impl attribute::HasDict for PyObject {
    fn get_dict(&self) -> &DictionaryObject {
        resolve_inner!(dictobj, self.value.dict, &DictionaryObject, dict);
        dictobj
    }
}


impl object::PyAPI for PyObject {}
impl method::New for PyObject {}
impl method::Init for PyObject {}
impl method::Delete for PyObject {}
impl method::GetAttr for PyObject  {}

    //
//    // TODO: Need to search the base classes dicts as well, maybe need MRO
//    fn op_getattr(&self, rt: &Runtime, name: &ObjectRef) -> RuntimeResult {
//        let boxed: &Box<Builtin> = name.0.borrow();
//        self.native_getattr(&boxed)
//    }
//
//    fn native_getattr(&self, name: &Builtin) -> NativeResult<&ObjectRef> {
//        match name {
//            &Builtin::Str(ref string) => {
//                let stringref = match string.rc.upgrade() {
//                    Ok(objref) => objref,
//                    Err(err) => return Err(err)
//                };
//
//                let boxed: &Box<Builtin> = self.value.0.borrow();
//                let dict: &native::Dictionary = boxed.dict().unwrap();
//                let key = native::Key(string.native_hash().unwrap(), stringref);
//                match dict.get(&key) {
//                    Some(objref) => Ok(objref),
//                    None => Err(Error::attribute())
//                }
//            },
//            _ => Err(Error::typerr("getattr(): attribute name must be string"))
//        }
//    }
//
//}

impl method::GetAttribute for PyObject {}
impl method::SetAttr for PyObject {}
impl method::DelAttr for PyObject {}
impl method::Id for PyObject {}
impl method::Is for PyObject {}
impl method::IsNot for PyObject {}
impl method::Hashed for PyObject {}
impl method::Equal for PyObject {}
impl method::NotEqual for PyObject {}
impl method::LessThan for PyObject {}
impl method::LessOrEqual for PyObject {}
impl method::GreaterOrEqual for PyObject {}
impl method::GreaterThan for PyObject {}
impl method::BooleanCast for PyObject {}
impl method::IntegerCast for PyObject {}
impl method::FloatCast for PyObject {}
impl method::ComplexCast for PyObject {}
impl method::Rounding for PyObject {}
impl method::Index for PyObject {}
impl method::NegateValue for PyObject {}
impl method::AbsValue for PyObject {}
impl method::PositiveValue for PyObject {}
impl method::InvertValue for PyObject {}
impl method::Add for PyObject {}
impl method::BitwiseAnd for PyObject {}
impl method::DivMod for PyObject {}
impl method::FloorDivision for PyObject {}
impl method::LeftShift for PyObject {}
impl method::Modulus for PyObject {}
impl method::Multiply for PyObject {}
impl method::MatrixMultiply for PyObject {}
impl method::BitwiseOr for PyObject {}
impl method::Pow for PyObject {}
impl method::RightShift for PyObject {}
impl method::Subtract for PyObject {}
impl method::TrueDivision for PyObject {}
impl method::XOr for PyObject {}
impl method::ReflectedAdd for PyObject {}
impl method::ReflectedBitwiseAnd for PyObject {}
impl method::ReflectedDivMod for PyObject {}
impl method::ReflectedFloorDivision for PyObject {}
impl method::ReflectedLeftShift for PyObject {}
impl method::ReflectedModulus for PyObject {}
impl method::ReflectedMultiply for PyObject {}
impl method::ReflectedMatrixMultiply for PyObject {}
impl method::ReflectedBitwiseOr for PyObject {}
impl method::ReflectedPow for PyObject {}
impl method::ReflectedRightShift for PyObject {}
impl method::ReflectedSubtract for PyObject {}
impl method::ReflectedTrueDivision for PyObject {}
impl method::ReflectedXOr for PyObject {}
impl method::InPlaceAdd for PyObject {}
impl method::InPlaceBitwiseAnd for PyObject {}
impl method::InPlaceDivMod for PyObject {}
impl method::InPlaceFloorDivision for PyObject {}
impl method::InPlaceLeftShift for PyObject {}
impl method::InPlaceModulus for PyObject {}
impl method::InPlaceMultiply for PyObject {}
impl method::InPlaceMatrixMultiply for PyObject {}
impl method::InPlaceBitwiseOr for PyObject {}
impl method::InPlacePow for PyObject {}
impl method::InPlaceRightShift for PyObject {}
impl method::InPlaceSubtract for PyObject {}
impl method::InPlaceTrueDivision for PyObject {}
impl method::InPlaceXOr for PyObject {}
impl method::Contains for PyObject {}
impl method::Iter for PyObject {}
impl method::Call for PyObject {}
impl method::Length for PyObject {}
impl method::LengthHint for PyObject {}
impl method::Next for PyObject {}
impl method::Reversed for PyObject {}
impl method::GetItem for PyObject {}
impl method::SetItem for PyObject {}
impl method::DeleteItem for PyObject {}
impl method::Count for PyObject {}
impl method::Append for PyObject {}
impl method::Extend for PyObject {}
impl method::Pop for PyObject {}
impl method::Remove for PyObject {}
impl method::IsDisjoint for PyObject {}
impl method::AddItem for PyObject {}
impl method::Discard for PyObject {}
impl method::Clear for PyObject {}
impl method::Get for PyObject {}
impl method::Keys for PyObject {}
impl method::Values for PyObject {}
impl method::Items for PyObject {}
impl method::PopItem for PyObject {}
impl method::Update for PyObject {}
impl method::SetDefault for PyObject {}
impl method::Await for PyObject {}
impl method::Send for PyObject {}
impl method::Throw for PyObject {}
impl method::Close for PyObject {}
impl method::Exit for PyObject {}
impl method::Enter for PyObject {}
impl method::DescriptorGet for PyObject {}
impl method::DescriptorSet for PyObject {}
impl method::DescriptorSetName for PyObject {}
