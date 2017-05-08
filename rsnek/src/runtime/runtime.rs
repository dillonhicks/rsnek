//! Interface to
use std;
use std::ops::BitAnd;
use std::borrow::Borrow;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::VecDeque;

use num::{Num, Zero};

use ::modules;
use ::api::result::{Error, ErrorType};
use ::api::RtObject;
use ::api::typing::BuiltinType;
use ::api::method::{GetItem, SetAttr, GetAttr};
use ::resources::strings;
use ::api::result::{ObjectResult};
use ::system::{StrongRc, WeakRc};
use ::runtime::traits::{
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
use ::modules::builtins::Type;
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
use ::system::primitives::{SignatureBuilder};
use ::system::primitives as rs;
use ::objects::none::{PyNoneType, NONE};
use ::objects::object::PyObjectType;
use ::objects::pytype::PyMeta;
use ::objects::set::PySetType;
use ::objects::frozenset::PyFrozenSetType;
use ::objects::string::PyStringType;
use ::objects::tuple::PyTupleType;
use ::objects::complex::PyComplexType;

/// Holder struct around the Reference Counted RuntimeInternal that
/// is passable and consumable in the interpreter code.
pub struct Runtime(RuntimeRef);

/// Same as `Runtime` but holds a weak reference instead of a strong reference.
pub struct WeakRuntime(RuntimeWeakRef);

/// Well Known builtin types that will be initialized by `Runtime` and provided
/// via the type specific provider methods.
struct BuiltinTypes {
    bool: PyBooleanType,
    bytes: PyBytesType,
    code: PyCodeType,
    dict: PyDictType,
    float: PyFloatType,
    frame: PyFrameType,
    frozenset: PyFrozenSetType,
    function: PyFunctionType,
    int: PyIntegerType,
    iterator: PyIteratorType,
    list: PyListType,
    meta: PyMeta,
    module: PyModuleType,
    none: PyNoneType,
    object: PyObjectType,
    set: PySetType,
    string: PyStringType,
    tuple: PyTupleType,
}

// TODO: {T99} add ability to intern objects
/// Concrete struct that holds the current runtime state, heap, etc.
struct RuntimeInternal {
    types: BuiltinTypes,
    modules: RefCell<RtObject>, // should be a dict
    mod_builtins: RefCell<RtObject>,
}


/// Type that is the Reference Counted wrapper around the actual runtime
type RuntimeRef = StrongRc<RuntimeInternal>;
type RuntimeWeakRef = WeakRc<RuntimeInternal>;


impl Clone for Runtime {
    /// Cloning a runtime just increases the strong reference count and gives
    /// back another RC'd RuntimeInternal wrapper `Runtime`.
    fn clone(&self) -> Self {
        Runtime((self.0).clone())
    }
}

impl Default for WeakRuntime {
    /// Creates an empty weakref.
    fn default() -> Self {
        WeakRuntime(WeakRc::new())
    }
}


impl Runtime {
    /// Create a new instance of runtime which will initialize the runtime
    /// available to types implementing the API. Most Api methods take a
    /// `&Runtime` as their first argument after `&self`. The `Interpreter`
    /// is responsible for passing the reference to the instance of the
    /// `Runtime` with which it was instantiated.
    ///
    ///  - All builtin types
    ///  - All builtin functions
    ///  - Create a builtin module
    pub fn new() -> Runtime {

        let meta = PyMeta::init_type();
        let object = PyObjectType::init_type(&meta.pytype);
        let module = PyModuleType::init_type(&meta.pytype);

        let builtins = BuiltinTypes {
            bool: PyBooleanType::init_type(),
            bytes: PyBytesType::init_type(),
            code: PyCodeType::init_type(),
            dict: PyDictType::init_type(),
            float: PyFloatType::init_type(),
            frame: PyFrameType::init_type(),
            frozenset: PyFrozenSetType::init_type(),
            function: PyFunctionType::init_type(&object.pytype, &object.object),
            int: PyIntegerType::init_type(),
            iterator: PyIteratorType::init_type(),
            list: PyListType::init_type(),
            meta: meta,
            module: module,
            none: PyNoneType::init_type(),
            object: object,
            set: PySetType::init_type(),
            string: PyStringType::init_type(),
            tuple: PyTupleType::init_type(),
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
            *_mod = rt.module(rs::None());
        }
        {
            let mut _mod: RefMut<RtObject> = rt.0.modules.borrow_mut();
            *_mod = rt.dict(rs::None());
        }


        rt.register_builtin(modules::builtins::LenFn::create());
        rt.register_builtin(modules::builtins::PrintFn::create());
        rt.register_builtin(modules::builtins::TypeFn::create());
        rt.register_builtin(modules::builtins::StrFn::create());
        rt.register_builtin(modules::builtins::IntFn::create());
        rt.register_builtin(modules::builtins::AllFn::create());
        rt.register_builtin(modules::builtins::AnyFn::create());
        rt.register_builtin(modules::builtins::ListFn::create());
        rt.register_builtin(modules::builtins::GlobalsFn::create());
        rt.register_builtin(modules::builtins::TupleFn::create());
        rt
    }

    /// Temporary method to put a function into the builtin module
    /// until the full module system is complete.
    pub fn register_builtin(&self, func: rs::Func) {
        let module: Ref<RtObject> = self.0.mod_builtins.borrow();
        let key = self.str(func.name.as_ref());
        module.op_setattr(&self, &key, &self.function(func)).unwrap();
    }

    /// Temporary solution to get a builtin function by name until
    /// the module and namespace system is complete.
    pub fn get_builtin(&self, name: &'static str) -> RtObject {
        let module: Ref<RtObject> = self.0.mod_builtins.borrow();
        let key = self.str(name);
        module.op_getattr(&self, &key).unwrap()
    }

}


impl<'a> ModuleImporter<&'a str> for Runtime {
    /// Import a module by path. Currently this will only allow imports of
    /// the builtin module with the name defined by `strings::BUILTINS_MODULE`
    /// but will be expanded to filesystem search in a later version.
    fn import_module(&self, path: &'a str) -> ObjectResult {
        match path {
            strings::BUILTINS_MODULE => {
                let ref_: Ref<RtObject> = self.0.mod_builtins.borrow();
                Ok(ref_.clone())
            },
            _ => Err(Error::module_not_found(path))
        }
    }
}


//
// None
//
impl NoneProvider for Runtime {
    #[inline]
    fn none(&self) -> RtObject {
        self.0.types.none.new(&self, NONE)
    }
}


//
// Boolean
impl BooleanProvider<rs::Boolean> for Runtime {
    fn bool(&self, value: rs::Boolean) -> RtObject {
        self.0.types.bool.new(&self, value)
    }
}


//
// Integer
//
impl<T: Num> IntegerProvider<T> for Runtime
        where rs::Integer: std::convert::From<T> {

    /// Create an `RtObject` from any integer type for which `rs::Integer`
    /// defines a `From` implementation, which is all of the rust native types.
    fn int(&self, value: T) -> RtObject {
        self.0.types.int.new(&self, rs::Integer::from(value))
    }
}


//
// Float
//
impl<T: Num> FloatProvider<T> for Runtime
    where rs::Float: std::convert::From<T> {

    /// Create an `RtObject` from any integer type for which `rs::Integer`
    /// defines a `From` implementation. All of the the rust native number
    /// types should be covered.
    fn float(&self, value: T) -> RtObject {
        self.0.types.float.new(&self, rs::Float::from(value))
    }
}


//
// Iterators
//
#[deprecated]
impl IteratorProvider<rs::None> for Runtime {
    #[allow(unused_variables)]
    fn iter(&self, value: rs::None) -> RtObject {
        self.0.types.iterator.empty(&self)
    }
}


impl IteratorProvider<rs::Iterator> for Runtime {
    #[allow(unused_variables)]
    fn iter(&self, value: rs::Iterator) -> RtObject {
        let wrapped = IteratorValue(value, self.clone());
        self.0.types.iterator.new(&self, wrapped)
    }
}


//
// String
//
#[deprecated]
impl StringProvider<rs::None> for Runtime {
    #[allow(unused_variables)]
    fn str(&self, value: rs::None) -> RtObject {
        self.0.types.string.empty.clone()
    }
}


impl StringProvider<rs::String> for Runtime {
    #[allow(unused_variables)]
    fn str(&self, value: rs::String) -> RtObject {
        self.0
            .types
            .string
            .new(&self, value)
    }
}

impl<'a> StringProvider<&'a str> for Runtime {
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
impl DictProvider<rs::Dict> for Runtime {
    fn dict(&self, value: rs::Dict) -> RtObject {
        self.0.types.dict.new(&self, value)
    }
}

#[deprecated]
impl DictProvider<rs::None> for Runtime {
    #[allow(unused_variables)]
    fn dict(&self, value: rs::None) -> RtObject {
        self.default_dict()
    }
}


impl DefaultDictProvider for Runtime {
    fn default_dict(&self) -> RtObject {
        self.0.types.dict.new(&self, rs::Dict::new())
    }
}

//
// Tuple
//
#[deprecated]
impl TupleProvider<rs::None> for Runtime {
    #[allow(unused_variables)]
    fn tuple(&self, value: rs::None) -> RtObject {
        self.default_tuple()
    }
}


impl TupleProvider<rs::Tuple> for Runtime {
    fn tuple(&self, value: rs::Tuple) -> RtObject {
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
impl ListProvider<rs::None> for Runtime {
    #[allow(unused_variables)]
    fn list(&self, value: rs::None) -> RtObject {
        self.default_list()
    }
}

impl ListProvider<rs::List> for Runtime {
    fn list(&self, value: rs::List) -> RtObject {
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
impl ObjectProvider<rs::None> for Runtime {
    #[allow(unused_variables)]
    fn object(&self, value: rs::None) -> RtObject {
        self.0
            .types
            .object
            .new(&self,
                 rs::Object {
                     class: self.0
                         .types
                         .object
                         .object
                         .clone(),
                     dict: self.dict(rs::None()),
                     bases: self.dict(rs::None()),
                 })
    }
}


impl ObjectProvider<rs::Object> for Runtime {
    #[allow(unused_variables)]
    fn object(&self, value: rs::Object) -> RtObject {
        self.0
            .types
            .object
            .new(&self, value)
    }
}

//
// type
//
impl PyTypeProvider<rs::None> for Runtime {
    #[allow(unused_variables)]
    fn pytype(&self, value: rs::None) -> RtObject {
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
impl FunctionProvider<rs::Func> for Runtime {
    /// Create a function object from the rs::Function and return its `RtObject`
    fn function(&self, value: rs::Func) -> RtObject {
        self.0
            .types
            .function
            .new(&self, value)
    }
}

#[deprecated]
impl FunctionProvider<rs::None> for Runtime {
    /// Create a function object that returns Ok(None)
    #[allow(unused_variables)]
    fn function(&self, value: rs::None) -> RtObject {
        self.function(self.none())
    }
}

impl FunctionProvider<RtObject> for Runtime {
    /// Create a function object that returns Ok(value)
    #[allow(unused_variables)]
    fn function(&self, value: RtObject) -> RtObject {
        let callable: Box<rs::WrapperFn> = Box::new(move |rt, pos_args, starargs, kwargs| Ok(value.clone()));
        self.function(rs::Func {
            name: String::from("returns_const"),
            module: String::from("runtime_provider"),
            signature: [].as_args(),
            callable: rs::FuncType::Wrapper(callable)
        })
    }
}


//
// Code
//
impl CodeProvider<rs::Code> for Runtime {
    fn code(&self, value: rs::Code) -> RtObject {
        self.0.types.code.new(&self, value)
    }
}

//
// Frames
//
impl FrameProvider<rs::Frame> for Runtime {
    fn frame(&self, value: rs::Frame) -> RtObject {
        self.0.types.frame.new(&self, value)
    }
}


impl DefaultFrameProvider for Runtime {
    #[allow(unused_variables)]
    fn default_frame(&self) -> RtObject {
        self.0.types.frame.new(&self, rs::Frame {
            f_lasti: rs::Integer::zero(),
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
impl ModuleProvider<rs::None> for Runtime {
    #[allow(unused_variables)]
    fn module(&self, value: rs::None) -> RtObject {
        self.0
            .types
            .module
            .new(&self,
                 rs::Object {
                     class: self.0
                         .types
                         .object
                         .object
                         .clone(),
                     dict: self.dict(rs::None()),
                     bases: self.dict(rs::None()),
                 })
    }
}


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
                let kwargs = rt.dict(rs::Dict::new());

                let len = rt.get_builtin("len");

                b.iter(|| { len.op_call(&rt, &args, &starargs, &kwargs).unwrap(); });

                let count = len.op_call(&rt, &args, &starargs, &kwargs).unwrap();
                assert_eq!(count, rt.int($N));
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
