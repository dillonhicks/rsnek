/// runtime.rs - The RSnek Runtime which will eventually be the interpreter
use std;
use std::borrow::Borrow;
use std::ops::Deref;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use num::Zero;

use object::typing::BuiltinType;
use object::method::{Length};

use result::{NativeResult, RuntimeResult};
use error::Error;

use typedef::native;
use typedef::builtin::Builtin;
use typedef::objectref::ObjectRef;
use typedef::none::{PyNoneType, NONE};
use typedef::boolean::PyBooleanType;
use typedef::integer::PyIntegerType;
use typedef::string::PyStringType;
use typedef::dictionary::PyDictType;
use typedef::object::PyObjectType;
use typedef::tuple::PyTupleType;
use typedef::pytype::PyMeta;
use typedef::method::PyFunctionType;


pub trait NoneProvider {
    fn none(&self) -> ObjectRef;
}

pub trait BooleanProvider<T> {
    fn bool(&self, value: T) -> ObjectRef;
}

pub trait IntegerProvider<T> {
    fn int(&self, value: T) -> ObjectRef;
}

pub trait DictProvider<T> {
    fn dict(&self, value: T) -> ObjectRef;
}

pub trait StringProvider<T> {
    fn str(&self, value: T) -> ObjectRef;
}

pub trait TupleProvider<T> {
    fn tuple(&self, value: T) -> ObjectRef;
}

pub trait PyTypeProvider<T> {
    fn pytype(&self, value: T) -> ObjectRef;
}


pub trait ObjectProvider<T> {
    fn object(&self, value: T) -> ObjectRef;
}

pub trait FunctionProvider<T> {
    fn function(&self, value: T) -> ObjectRef;
}

/// Holder struct around the Reference Counted RuntimeInternal that
/// is passable and consumable in the interpreter code.
///
pub struct Runtime(RuntimeRef);


/// Well Known builtin types
pub struct BuiltinTypes {
    none: PyNoneType,
    bool: PyBooleanType,
    int: PyIntegerType,
    dict: PyDictType,
    string: PyStringType,
    tuple: PyTupleType,
    function: PyFunctionType,
    object: PyObjectType,
    meta: PyMeta,
}


/// Concrete struct that holds the current runtime state, heap, etc.
/// TODO: add ability to intern objects?
struct RuntimeInternal {
    types: BuiltinTypes,
    funcs: RefCell<native::KWDict>,
}


/// Type that is the Reference Counted wrapper around the actual runtime
///
/// Patterns about References Taken from:
///  https://ricardomartins.cc/2016/06/08/interior-mutability
type RuntimeRef = Rc<Box<RuntimeInternal>>;


/// Cloning a runtime just increases the strong reference count and gives
/// back another RC'd RuntimeInternal wrapper `Runtime`.
impl Clone for Runtime {
    fn clone(&self) -> Self {
        Runtime((self.0).clone())
    }
}


#[inline]
fn check_args(count: usize, pos_args: &ObjectRef) -> NativeResult<native::None> {
    let boxed: &Box<Builtin> = pos_args.0.borrow();
    match boxed.deref() {
        &Builtin::Tuple(ref tuple) => {
            if tuple.value.0.len() == count {
                Ok(native::None())
            }
            else {
                Err(Error::typerr("Argument mismatch 1"))
            }
        },
        _ => Err(Error::typerr("Expected type tuple for pos_args"))
    }
}

#[inline]
fn check_kwargs(count: usize, kwargs: &ObjectRef) -> NativeResult<native::None> {
    let boxed: &Box<Builtin> = kwargs.0.borrow();
    match boxed.deref() {

        &Builtin::Dict(ref dict) => {
            let borrowed: Ref<native::Dict> = dict.value.0.borrow();

            if borrowed.len() == count {
                Ok(native::None())
            } else {
                Err(Error::typerr("Argument mismatch 2"))
            }
        },
        _ => Err(Error::typerr("Expected type tuple for pos_args"))
    }

}


fn builtin_len() -> native::Function {
    let func: Box<native::WrapperFn> = Box::new(|rt, pos_args, starargs, kwargs| {
        match check_args(1, &pos_args) {
            Err(err) => return Err(err),
            _ => {}
        };

        match check_args(0, &starargs) {
            Err(err) => return Err(err),
            _ => {}
        };

        match check_kwargs(0, &kwargs) {
            Err(err) => return Err(err),
            _ => {}
        };

        let boxed: &Box<Builtin> = pos_args.0.borrow();
        boxed.op_len(&rt)
    });

    native::Function::Wrapper(func)
}



impl Runtime {
    pub fn new() -> Runtime {

        let meta = PyMeta::init_type();
        let object = PyObjectType::init_type(&meta.pytype);

        let builtins = BuiltinTypes {
            none: PyNoneType::init_type(),
            bool: PyBooleanType::init_type(),
            int: PyIntegerType::init_type(),
            dict: PyDictType::init_type(),
            string: PyStringType::init_type(),
            tuple: PyTupleType::init_type(),
            function: PyFunctionType::init_type(&object.pytype, &object.object),
            object: object,
            meta: meta
        };

        let internal = RuntimeInternal {
            types: builtins,
            funcs: RefCell::new(native::KWDict::new())
        };


        let rt = Runtime(Rc::new(Box::new(internal)));
        rt.register_builtin("len", builtin_len());
        rt
    }

    pub fn register_builtin(&self, name: &'static str, func: native::Function)  {
        let mut funcs = self.0.funcs.borrow_mut();
        let func_obj = self.function(func);
        funcs.insert(name.to_string(), func_obj.clone());
    }

    pub fn get_builtin(&self, name: &'static str) -> ObjectRef {
        let funcs = self.0.funcs.borrow();
        let key = name.to_string();
        match funcs.get(&key) {
            Some(objref) => objref.clone(),
            None => self.none()
        }
    }
}

//
// None
//
impl NoneProvider for Runtime {
    #[inline]
    fn none(&self) -> ObjectRef {
        self.0.types.none.new(&self, NONE)
    }
}

//
// Boolean
//
impl BooleanProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn bool(&self, value: native::None) -> ObjectRef {
        self.0.types.bool.new(&self, false)
    }
}

impl BooleanProvider<native::Boolean> for Runtime {
    fn bool(&self, value: native::Boolean) -> ObjectRef {
        self.0.types.bool.new(&self, value)
    }
}

//
// Integer
//

impl IntegerProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn int(&self, value: native::None) -> ObjectRef {
        self.0.types.int.new(&self, native::Integer::zero())
    }
}


impl IntegerProvider<native::Integer> for Runtime {
    fn int(&self, value: native::Integer) -> ObjectRef {
        self.0.types.int.new(&self, value)
    }
}

impl IntegerProvider<native::ObjectId> for Runtime {
    fn int(&self, value: native::ObjectId) -> ObjectRef {
        self.0.types.int.new(&self, native::Integer::from(value))
    }
}

impl IntegerProvider<i32> for Runtime {
    fn int(&self, value: i32) -> ObjectRef {
        self.0.types.int.new(&self, native::Integer::from(value))
    }
}


//
// String
//

impl StringProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn str(&self, value: native::None) -> ObjectRef {
        self.0.types.string.empty.clone()
    }
}

impl StringProvider<native::String> for Runtime {
    #[allow(unused_variables)]
    fn str(&self, value: native::String) -> ObjectRef {
        self.0.types.string.new(&self, value)
    }
}

impl StringProvider<&'static str> for Runtime {
    #[allow(unused_variables)]
    fn str(&self, value: &'static str) -> ObjectRef {
        self.0.types.string.new(&self, value.to_string())
    }
}

//
// Dict
//
impl DictProvider<native::Dict> for Runtime {
    fn dict(&self, value: native::Dict) -> ObjectRef {
        self.0.types.dict.new(&self, value)
    }
}

impl DictProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn dict(&self, value: native::None) -> ObjectRef {
        self.0.types.dict.new(&self, native::Dict::new())
    }
}

//
// Tuple
//
impl TupleProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn tuple(&self, value: native::None) -> ObjectRef {
        self.0.types.tuple.empty.clone()
    }
}


impl TupleProvider<native::Tuple> for Runtime {
    fn tuple(&self, value: native::Tuple) -> ObjectRef {
        self.0.types.tuple.new(&self, value)
    }
}

//
// Object
//
impl ObjectProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn object(&self, value: native::None) -> ObjectRef {
        self.0.types.object.new(&self, native::Object {
            class: self.0.types.object.object.clone(),
            dict: self.dict(native::None()),
            bases: self.dict(native::None())
        })
    }
}


impl ObjectProvider<native::Object> for Runtime {
    #[allow(unused_variables)]
    fn object(&self, value: native::Object) -> ObjectRef {
        self.0.types.object.new(&self, value)
    }
}

//
// type
//
impl PyTypeProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn pytype(&self, value: native::None) -> ObjectRef {
        self.0.types.meta.pytype.clone()
    }
}


//
// method
//
impl FunctionProvider<native::Function> for Runtime {
    fn function(&self, value: native::Function) -> ObjectRef {
        self.0.types.function.new(&self, value)
    }
}


impl FunctionProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn function(&self, value: native::None) -> ObjectRef {
        self.function(self.none())
    }
}

impl FunctionProvider<ObjectRef> for Runtime {
    #[allow(unused_variables)]
    fn function(&self, value: ObjectRef) -> ObjectRef {
        let func: Box<native::WrapperFn> = Box::new(move |rt, pos_args, starargs, kwargs| Ok(value.clone()));
        self.function(native::Function::Wrapper(func))
    }
}

// stdlib
impl std::fmt::Debug for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Runtime()")
    }
}

#[cfg(test)]
mod _api {

    use super::*;
    use object::method::Call;
    use test::Bencher;

    fn setup_test() -> (Runtime) {
        Runtime::new()
    }

    #[bench]
    fn test_builtin_len(b: &mut Bencher) {
        let rt = setup_test();
        let tuple = rt.tuple(vec![rt.none(), rt.none(), rt.none()]);

        let args = rt.tuple(vec![tuple.clone()]);
        let starargs = rt.tuple(vec![]);
        let kwargs = rt.dict(native::Dict::new());

        let func = rt.get_builtin("len");
        let len: &Box<Builtin> = func.0.borrow();

        b.iter(|| {
            let result = len.op_call(&rt, &args, &starargs, &kwargs).unwrap();
            assert_eq!(result, rt.int(1));
        })

    }
}