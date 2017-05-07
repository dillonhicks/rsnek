use std;
use std::borrow::Borrow;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::VecDeque;

use num::{Num, Zero};

use ::builtin;
use ::error::{Error, ErrorType};
use ::api::RtObject;
use ::api::typing::BuiltinType;
use ::api::method::{GetItem, SetAttr, GetAttr};
use ::resource::strings;
use ::result::{ObjectResult};
use ::system::{StrongRc, WeakRc};
use ::traits::{
    BooleanProvider,
    BytesProvider,
    CodeProvider,
    DictProvider,
    FloatProvider,
    FrameProvider,
    FunctionProvider,
    IntegerProvider,
    IteratorProvider,
    ListProvider,
    ModuleFinder,
    ModuleImporter,
    ModuleProvider,
    NoneProvider,
    ObjectProvider,
    PyTypeProvider,
    StringProvider,
    TupleProvider,

    DefaultDictProvider,
    DefaultFrameProvider,
    DefaultListProvider,
    DefaultStringProvider,
    DefaultTupleProvider,
};
use ::objects::boolean::PyBooleanType;
use ::objects::builtin::Builtin;
use ::objects::bytes::PyBytesType;
use ::objects::code::PyCodeType;
use ::objects::dictionary::PyDictType;
use ::objects::float::PyFloatType;
use ::objects::frame::PyFrameType;
use ::objects::integer::PyIntegerType;
use ::objects::iterator::{PyIteratorType, IteratorValue};
use ::objects::list::PyListType;
use ::objects::method::PyFunctionType;
use ::objects::module::PyModuleType;
use ::objects::native::{self, SignatureBuilder};
use ::objects::none::{PyNoneType, NONE};
use ::objects::object::PyObjectType;
use ::objects::pytype::PyMeta;
use ::objects::string::PyStringType;
use ::objects::tuple::PyTupleType;

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
    modules: RefCell<RtObject>, // should be a dict
    mod_builtins: RefCell<RtObject>,
}


/// Type that is the Reference Counted wrapper around the actual runtime
///
/// Patterns about References Taken from:
///  https://ricardomartins.cc/2016/06/08/interior-mutability
type RuntimeRef = StrongRc<RuntimeInternal>;
type RuntimeWeakRef = WeakRc<RuntimeInternal>;


/// Cloning a runtime just increases the strong reference count and gives
/// back another RC'd RuntimeInternal wrapper `Runtime`.
impl Clone for Runtime {
    fn clone(&self) -> Self {
        Runtime((self.0).clone())
    }
}


impl Default for WeakRuntime {
    fn default() -> Self {
        WeakRuntime(WeakRc::new())
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

        let rt = Runtime(StrongRc::new(internal));
        {
            let mut _mod: RefMut<RtObject> = rt.0.mod_builtins.borrow_mut();
            *_mod = rt.module(native::None());
        }
        {
            let mut _mod: RefMut<RtObject> = rt.0.modules.borrow_mut();
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
        WeakRuntime(StrongRc::downgrade(&self.0.clone()))
    }

    pub fn register_builtin(&self, func: native::Func) {
        let module: Ref<RtObject> = self.0.mod_builtins.borrow();
        let key = self.str(func.name.as_ref());
        module.op_setattr(&self, &key, &self.function(func)).unwrap();
    }

    pub fn get_builtin(&self, name: &'static str) -> RtObject {
        let module: Ref<RtObject> = self.0.mod_builtins.borrow();
        let key = self.str(name);
        module.op_getattr(&self, &key).unwrap()
    }

}


impl<'a> ModuleImporter<&'a str> for Runtime {
    fn import_module(&self, name: &'a str) -> ObjectResult {
        match name {
            strings::BUILTINS_MODULE => {
                let ref_: Ref<RtObject> = self.0.mod_builtins.borrow();
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
    fn none(&self) -> RtObject {
        self.0
            .types
            .none
            .new(&self, NONE)
    }
}

//
// Boolean
//
#[deprecated]
impl BooleanProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn bool(&self, value: native::None) -> RtObject {
        self.0
            .types
            .bool
            .new(&self, false)
    }
}

impl BooleanProvider<native::Boolean> for Runtime {
    fn bool(&self, value: native::Boolean) -> RtObject {
        self.0
            .types
            .bool
            .new(&self, value)
    }
}

//
// Integer
//
impl<T: Num> IntegerProvider<T> for Runtime
        where native::Integer: std::convert::From<T> {

    fn int(&self, value: T) -> RtObject {
        self.0
            .types
            .int
            .new(&self, native::Integer::from(value))
    }
}


#[deprecated]
impl IntegerProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn int(&self, value: native::None) -> RtObject {
        self.0
            .types
            .int
            .new(&self, native::Integer::zero())
    }
}


//
// Float
//
#[deprecated]
impl FloatProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn float(&self, value: native::None) -> RtObject {
        self.0.types.float.new(&self, 0.0)
    }
}


impl FloatProvider<native::Float> for Runtime {
    fn float(&self, value: native::Float) -> RtObject {
        self.0.types.float.new(&self, value)
    }
}


//
// Iterators
//
#[deprecated]
impl IteratorProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn iter(&self, value: native::None) -> RtObject {
        self.0.types.iterator.empty(&self)
    }
}


impl IteratorProvider<native::Iterator> for Runtime {
    #[allow(unused_variables)]
    fn iter(&self, value: native::Iterator) -> RtObject {
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
#[deprecated]
impl StringProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn str(&self, value: native::None) -> RtObject {
        self.0
            .types
            .string
            .empty
            .clone()
    }
}

#[deprecated]
impl BytesProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn bytes(&self, value: native::None) -> RtObject {
        self.0
            .types
            .bytes
            .empty
            .clone()
    }
}


impl StringProvider<native::String> for Runtime {
    #[allow(unused_variables)]
    fn str(&self, value: native::String) -> RtObject {
        self.0
            .types
            .string
            .new(&self, value)
    }
}

impl<'a>  StringProvider<&'a str> for Runtime {
    #[allow(unused_variables)]
    fn str(&self, value: &'a str) -> RtObject {
        self.0
            .types
            .string
            .new(&self, String::from(value))
    }
}

impl DefaultStringProvider for Runtime {
    fn default_str(&self) -> RtObject {
        self.0.types.string.empty.clone()
    }
}

//
// Dict
//
impl DictProvider<native::Dict> for Runtime {
    fn dict(&self, value: native::Dict) -> RtObject {
        self.0.types.dict.new(&self, value)
    }
}

#[deprecated]
impl DictProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn dict(&self, value: native::None) -> RtObject {
        self.default_dict()
    }
}


impl DefaultDictProvider for Runtime {
    fn default_dict(&self) -> RtObject {
        self.0.types.dict.new(&self, native::Dict::new())
    }
}

//
// Tuple
//
#[deprecated]
impl TupleProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn tuple(&self, value: native::None) -> RtObject {
        self.default_tuple()
    }
}


impl TupleProvider<native::Tuple> for Runtime {
    fn tuple(&self, value: native::Tuple) -> RtObject {
        self.0
            .types
            .tuple
            .new(&self, value)
    }
}

impl DefaultTupleProvider for Runtime {
    fn default_tuple(&self) -> RtObject {
        self.0.types.tuple.empty.clone()
    }
}

//
// List
//
#[deprecated]
impl ListProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn list(&self, value: native::None) -> RtObject {
        self.default_list()
    }
}

impl ListProvider<native::List> for Runtime {
    fn list(&self, value: native::List) -> RtObject {
        self.0
            .types
            .list
            .new(&self, value)
    }
}


impl DefaultListProvider for Runtime {
    fn default_list(&self) -> RtObject {
        self.0.types.list.empty.clone()
    }
}

//
// Object
//
#[deprecated]
impl ObjectProvider<native::None> for Runtime {
    #[allow(unused_variables)]
    fn object(&self, value: native::None) -> RtObject {
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
    fn object(&self, value: native::Object) -> RtObject {
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
    fn pytype(&self, value: native::None) -> RtObject {
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
    /// Create a function object from the native::Function and return its `RtObject`
    fn function(&self, value: native::Func) -> RtObject {
        self.0
            .types
            .function
            .new(&self, value)
    }
}

#[deprecated]
impl FunctionProvider<native::None> for Runtime {
    /// Create a function object that returns Ok(None)
    #[allow(unused_variables)]
    fn function(&self, value: native::None) -> RtObject {
        self.function(self.none())
    }
}

impl FunctionProvider<RtObject> for Runtime {
    /// Create a function object that returns Ok(value)
    #[allow(unused_variables)]
    fn function(&self, value: RtObject) -> RtObject {
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
    fn code(&self, value: native::Code) -> RtObject {
        self.0.types.code.new(&self, value)
    }
}

//
// Frames
//
impl FrameProvider<native::Frame> for Runtime {
    fn frame(&self, value: native::Frame) -> RtObject {
        self.0.types.frame.new(&self, value)
    }
}


impl DefaultFrameProvider for Runtime {
    #[allow(unused_variables)]
    fn default_frame(&self) -> RtObject {
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
    fn module(&self, value: native::None) -> RtObject {
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
    fn get_module(&self, name: &'static str) -> ObjectResult {
        let modules: Ref<RtObject> = self.0.modules.borrow();

        match modules.op_getitem(&self, &self.str(name)) {
            Ok(objref) => Ok(objref),
            Err(Error(ErrorType::Key, _)) => Err(Error::module_not_found(name)),
            Err(err) => Err(err)
        }
    }
}

impl<'a> ModuleImporter<(&'static str, &'a RtObject)> for Runtime {

    #[allow(unused_variables)]
    fn import_module(&self, args: (&'static str, &RtObject)) -> ObjectResult {
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
mod tests {

    use super::*;
    use api::method::Call;
    use test::Bencher;

    fn setup_test() -> (Runtime) {
        Runtime::new()
    }

    macro_rules! len_bench (
        ($name:ident, $N:expr) => (
            #[bench]
            fn $name(b: &mut Bencher) {
                let rt = setup_test();
                let tuple = rt.tuple((0..$N).map(|_| rt.none()).collect::<Vec<_>>());

                let args = rt.tuple(vec![tuple.clone()]);
                let starargs = rt.tuple(vec![]);
                let kwargs = rt.dict(native::Dict::new());

                let len = rt.get_builtin("len");

                b.iter(|| { len.op_call(&rt, &args, &starargs, &kwargs).unwrap(); });
            }
        );
    );

    len_bench!(builtin_len_tuple_elems_0,      0);
    len_bench!(builtin_len_tuple_elems_1,      1);
    len_bench!(builtin_len_tuple_elems_4,      4);
    len_bench!(builtin_len_tuple_elems_16,     16);
    len_bench!(builtin_len_tuple_elems_64,     64);
    len_bench!(builtin_len_tuple_elems_256,    256);
    len_bench!(builtin_len_tuple_elems_1024,   1024);
    len_bench!(builtin_len_tuple_elems_4096,   4095);
    len_bench!(builtin_len_tuple_elems_16384,  16384);


    #[bench]
    fn int_getattr_hash_method_wrapper(b: &mut Bencher) {
        let rt = setup_test();
        let one = rt.int(1);
        let name = rt.str("__hash__");

        b.iter(|| { one.op_getattr(&rt, &name).unwrap() });
    }
}
