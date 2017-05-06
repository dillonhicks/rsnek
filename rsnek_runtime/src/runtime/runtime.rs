use std;
use std::borrow::Borrow;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};
use std::collections::VecDeque;

use num::Zero;

use object::typing::BuiltinType;
use object::method::{GetItem, SetAttr, GetAttr};

use traits::{
    ModuleImporter,
    ModuleProvider,
    ModuleFinder,
    BooleanProvider,
    IntegerProvider,
    FloatProvider,
    IteratorProvider,
    StringProvider,
    BytesProvider,
    NoneProvider,
    ObjectProvider,
    DictProvider,
    TupleProvider,
    ListProvider,
    FunctionProvider,
    PyTypeProvider,
    CodeProvider,
    FrameProvider,

    DefaultDictProvider,
    DefaultFrameProvider,
    DefaultStringProvider,
    DefaultTupleProvider,
    DefaultListProvider,
};

use result::{RuntimeResult};
use error::{Error, ErrorType};
use builtin;

use typedef::native::{self, SignatureBuilder};
use typedef::builtin::Builtin;
use ::object::RtObject as ObjectRef;
use typedef::none::{PyNoneType, NONE};
use typedef::boolean::PyBooleanType;
use typedef::integer::PyIntegerType;
use typedef::float::PyFloatType;
use typedef::iterator::{PyIteratorType, IteratorValue};
use typedef::string::PyStringType;
use typedef::bytes::PyBytesType;
use typedef::dictionary::PyDictType;
use typedef::object::PyObjectType;
use typedef::tuple::PyTupleType;
use typedef::list::PyListType;
use typedef::pytype::PyMeta;
use typedef::method::PyFunctionType;
use typedef::module::PyModuleType;
use typedef::code::PyCodeType;
use typedef::frame::PyFrameType;

use ::resource::strings;

/// Holder struct around the Reference Counted RuntimeInternal that
/// is passable and consumable in the interpreter code.
///
pub struct Runtime(RuntimeRef);
pub struct WeakRuntime(RuntimeWeakRef);


/// Well Known builtin types
pub struct BuiltinTypes {
    none: PyNoneType,
    bool: PyBooleanType,
    int: PyIntegerType,
    float: PyFloatType,
    iterator: PyIteratorType,
    dict: PyDictType,
    string: PyStringType,
    bytes: PyBytesType,
    tuple: PyTupleType,
    list: PyListType,
    function: PyFunctionType,
    object: PyObjectType,
    module: PyModuleType,
    code: PyCodeType,
    frame: PyFrameType,
    meta: PyMeta,
}

/// Concrete struct that holds the current runtime state, heap, etc.
// TODO: {T99} add ability to intern objects
struct RuntimeInternal {
    types: BuiltinTypes,
    modules: RefCell<ObjectRef>, // should be a dict
    mod_builtins: RefCell<ObjectRef>,
}


/// Type that is the Reference Counted wrapper around the actual runtime
///
/// Patterns about References Taken from:
///  https://ricardomartins.cc/2016/06/08/interior-mutability
type RuntimeRef = Rc<Box<RuntimeInternal>>;
type RuntimeWeakRef = Weak<Box<RuntimeInternal>>;


/// Cloning a runtime just increases the strong reference count and gives
/// back another RC'd RuntimeInternal wrapper `Runtime`.
impl Clone for Runtime {
    fn clone(&self) -> Self {
        Runtime((self.0).clone())
    }
}


impl Default for WeakRuntime {
    fn default() -> Self {
        WeakRuntime(Weak::new())
    }
}


impl Runtime {
    pub fn new() -> Runtime {

        let meta = PyMeta::init_type();
        let object = PyObjectType::init_type(&meta.pytype);
        let module = PyModuleType::init_type(&meta.pytype);

        let builtins = BuiltinTypes {
            none: PyNoneType::init_type(),
            bool: PyBooleanType::init_type(),
            int: PyIntegerType::init_type(),
            float: PyFloatType::init_type(),
            iterator: PyIteratorType::init_type(),
            dict: PyDictType::init_type(),
            string: PyStringType::init_type(),
            bytes: PyBytesType::init_type(),
            tuple: PyTupleType::init_type(),
            list: PyListType::init_type(),
            function: PyFunctionType::init_type(&object.pytype, &object.object),
            object: object,
            module: module,
            code: PyCodeType::init_type(),
            frame: PyFrameType::init_type(),
            meta: meta,
        };

        let placeholder = builtins.meta.pytype.clone();

        let internal = RuntimeInternal {
            types: builtins,
            modules: RefCell::new(placeholder.clone()),
            mod_builtins: RefCell::new(placeholder.clone()),
        };

        let rt = Runtime(Rc::new(Box::new(internal)));
        {
            let mut _mod: RefMut<ObjectRef> = rt.0.mod_builtins.borrow_mut();
            *_mod = rt.module(native::None());
        }
        {
            let mut _mod: RefMut<ObjectRef> = rt.0.modules.borrow_mut();
            *_mod = rt.dict(native::None());
        }


        rt.register_builtin(builtin::LenFn::create());
        rt.register_builtin(builtin::PrintFn::create());
        rt.register_builtin(builtin::TypeFn::create());
        rt.register_builtin(builtin::StrFn::create());
        rt.register_builtin(builtin::IntFn::create());
        rt.register_builtin(builtin::AllFn::create());
        rt.register_builtin(builtin::AnyFn::create());
        rt.register_builtin(builtin::ListFn::create());
        rt.register_builtin(builtin::GlobalsFn::create());
        rt.register_builtin(builtin::TupleFn::create());
        rt
    }

    pub fn downgrade(&self) -> WeakRuntime {
        WeakRuntime(Rc::downgrade(&self.0.clone()))
    }

    pub fn register_builtin(&self, func: native::Func) {
        let boxed: Ref<ObjectRef> = self.0.mod_builtins.borrow();
        let boxed: &Box<Builtin> = boxed.0.borrow();
        let key = self.str(func.name.as_str());
        boxed.op_setattr(&self, &key, &self.function(func)).unwrap();
    }

    pub fn get_builtin(&self, name: &'static str) -> ObjectRef {
        let boxed: Ref<ObjectRef> = self.0.mod_builtins.borrow();
        let boxed: &Box<Builtin> = boxed.0.borrow();
        let key = self.str(name);
        boxed.op_getattr(&self, &key).unwrap()
    }

}

impl<'a> ModuleImporter<&'a str> for Runtime {
    fn import_module(&self, name: &'a str) -> RuntimeResult {
        match name {
            strings::BUILTINS_MODULE => {
                let ref_: Ref<ObjectRef> = self.0.mod_builtins.borrow();
                Ok(ref_.clone())
            },
            _ => Err(Error::module_not_found(name))
        }
    }
}

//
// None
//
impl NoneProvider for Runtime {
    #[inline]
    fn none(&self) -> ObjectRef {
        self.0
            .types
            .none
            .new(&self, NONE)
    }
}

//
// Boolean
//
impl BooleanProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn bool(&self, value: native::None) -> ObjectRef {
        self.0
            .types
            .bool
            .new(&self, false)
    }
}

impl BooleanProvider<native::Boolean> for Runtime {
    fn bool(&self, value: native::Boolean) -> ObjectRef {
        self.0
            .types
            .bool
            .new(&self, value)
    }
}

//
// Integer
//

impl IntegerProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn int(&self, value: native::None) -> ObjectRef {
        self.0
            .types
            .int
            .new(&self, native::Integer::zero())
    }
}


impl IntegerProvider<native::Integer> for Runtime {
    fn int(&self, value: native::Integer) -> ObjectRef {
        self.0
            .types
            .int
            .new(&self, value)
    }
}

impl IntegerProvider<native::ObjectId> for Runtime {
    fn int(&self, value: native::ObjectId) -> ObjectRef {
        self.0
            .types
            .int
            .new(&self, native::Integer::from(value))
    }
}

impl IntegerProvider<i32> for Runtime {
    fn int(&self, value: i32) -> ObjectRef {
        self.0
            .types
            .int
            .new(&self, native::Integer::from(value))
    }
}

impl IntegerProvider<i64> for Runtime {
    fn int(&self, value: i64) -> ObjectRef {
        self.0
            .types
            .int
            .new(&self, native::Integer::from(value))
    }
}

//
// Float
//
impl FloatProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn float(&self, value: native::None) -> ObjectRef {
        self.0.types.float.new(&self, 0.0)
    }
}


impl FloatProvider<native::Float> for Runtime {
    fn float(&self, value: native::Float) -> ObjectRef {
        self.0.types.float.new(&self, value)
    }
}


//
// Iterators
//

impl IteratorProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn iter(&self, value: native::None) -> ObjectRef {
        self.0.types.iterator.empty(&self)
    }
}


impl IteratorProvider<native::Iterator> for Runtime {
    #[allow(unused_variables)]
    fn iter(&self, value: native::Iterator) -> ObjectRef {
        let wrapped = IteratorValue(value, self.clone());
        self.0
            .types
            .iterator
            .new(&self, wrapped)
    }
}


//
// String
//

impl StringProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn str(&self, value: native::None) -> ObjectRef {
        self.0
            .types
            .string
            .empty
            .clone()
    }
}


impl BytesProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn bytes(&self, value: native::None) -> ObjectRef {
        self.0
            .types
            .bytes
            .empty
            .clone()
    }
}


impl StringProvider<native::String> for Runtime {
    #[allow(unused_variables)]
    fn str(&self, value: native::String) -> ObjectRef {
        self.0
            .types
            .string
            .new(&self, value)
    }
}

impl<'a>  StringProvider<&'a str> for Runtime {
    #[allow(unused_variables)]
    fn str(&self, value: &'a str) -> ObjectRef {
        self.0
            .types
            .string
            .new(&self, String::from(value))
    }
}

impl DefaultStringProvider for Runtime {
    fn default_str(&self) -> ObjectRef {
        self.0.types.string.empty.clone()
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
        self.default_dict()
    }
}


impl DefaultDictProvider for Runtime {
    fn default_dict(&self) -> ObjectRef {
        self.0.types.dict.new(&self, native::Dict::new())
    }
}

//
// Tuple
//
impl TupleProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn tuple(&self, value: native::None) -> ObjectRef {
        self.default_tuple()
    }
}


impl TupleProvider<native::Tuple> for Runtime {
    fn tuple(&self, value: native::Tuple) -> ObjectRef {
        self.0
            .types
            .tuple
            .new(&self, value)
    }
}

impl DefaultTupleProvider for Runtime {
    fn default_tuple(&self) -> ObjectRef {
        self.0.types.tuple.empty.clone()
    }
}

//
// List
//

impl ListProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn list(&self, value: native::None) -> ObjectRef {
        self.default_list()
    }
}

impl ListProvider<native::List> for Runtime {
    fn list(&self, value: native::List) -> ObjectRef {
        self.0
            .types
            .list
            .new(&self, value)
    }
}



impl DefaultListProvider for Runtime {
    fn default_list(&self) -> ObjectRef {
        self.0.types.list.empty.clone()
    }
}

//
// Object
//
impl ObjectProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn object(&self, value: native::None) -> ObjectRef {
        self.0
            .types
            .object
            .new(&self,
                 native::Object {
                     class: self.0
                         .types
                         .object
                         .object
                         .clone(),
                     dict: self.dict(native::None()),
                     bases: self.dict(native::None()),
                 })
    }
}


impl ObjectProvider<native::Object> for Runtime {
    #[allow(unused_variables)]
    fn object(&self, value: native::Object) -> ObjectRef {
        self.0
            .types
            .object
            .new(&self, value)
    }
}

//
// type
//
impl PyTypeProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn pytype(&self, value: native::None) -> ObjectRef {
        self.0
            .types
            .meta
            .pytype
            .clone()
    }
}


//
// Functions and Methods
//
impl FunctionProvider<native::Func> for Runtime {
    /// Create a function object from the native::Function and return its `ObjectRef`
    fn function(&self, value: native::Func) -> ObjectRef {
        self.0
            .types
            .function
            .new(&self, value)
    }
}

impl FunctionProvider<native::None> for Runtime {
    /// Create a function object that returns Ok(None)
    #[allow(unused_variables)]
    fn function(&self, value: native::None) -> ObjectRef {
        self.function(self.none())
    }
}

impl FunctionProvider<ObjectRef> for Runtime {
    /// Create a function object that returns Ok(value)
    #[allow(unused_variables)]
    fn function(&self, value: ObjectRef) -> ObjectRef {
        let callable: Box<native::WrapperFn> = Box::new(move |rt, pos_args, starargs, kwargs| Ok(value.clone()));
        self.function(native::Func {
            name: String::from("returns_const"),
            module: String::from("runtime_provider"),
            signature: [].as_args(),
            callable: native::FuncType::Wrapper(callable)
        })
    }
}


//
// Code
//
impl CodeProvider<native::Code> for Runtime {
    fn code(&self, value: native::Code) -> ObjectRef {
        self.0.types.code.new(&self, value)
    }
}

//
// Frames
//
impl FrameProvider<native::Frame> for Runtime {
    fn frame(&self, value: native::Frame) -> ObjectRef {
        self.0.types.frame.new(&self, value)
    }
}

impl DefaultFrameProvider for Runtime {
    #[allow(unused_variables)]
    fn default_frame(&self) -> ObjectRef {
        self.0.types.frame.new(&self, native::Frame {
            f_lasti: native::Integer::zero(),
            f_builtins: self.default_dict(),
            f_code: self.none(),
            f_back: self.none(),
            blocks: VecDeque::new(),
        })
    }
}

//
// Module
//
impl ModuleProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn module(&self, value: native::None) -> ObjectRef {
        self.0
            .types
            .module
            .new(&self,
                 native::Object {
                     class: self.0
                         .types
                         .object
                         .object
                         .clone(),
                     dict: self.dict(native::None()),
                     bases: self.dict(native::None()),
                 })
    }
}


// Module registry
impl ModuleFinder<&'static str> for Runtime {
    fn get_module(&self, name: &'static str) -> RuntimeResult {
        let boxed: Ref<ObjectRef> = self.0.modules.borrow();
        let boxed: &Box<Builtin> = boxed.0.borrow();

        match boxed.op_getitem(&self, &self.str(name)) {
            Ok(objref) => Ok(objref),
            Err(Error(ErrorType::Key, _)) => Err(Error::module_not_found(name)),
            Err(err) => Err(err)
        }
    }
}

impl<'a> ModuleImporter<(&'static str, &'a ObjectRef)> for Runtime {

    #[allow(unused_variables)]
    fn import_module(&self, args: (&'static str, &ObjectRef)) -> RuntimeResult {
        Err(Error::not_implemented())
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

        b.iter(|| { len.op_call(&rt, &args, &starargs, &kwargs).unwrap(); });

        let result = len.op_call(&rt, &args, &starargs, &kwargs).unwrap();
        assert_eq!(result, rt.int(3));
    }
}
