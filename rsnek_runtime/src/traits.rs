use result::RuntimeResult;
use typedef::objectref::ObjectRef;


pub trait New<T> {
    fn new(value: T) -> Self;
}

pub trait NoneProvider {
    fn none(&self) -> ObjectRef;
}

pub trait BooleanProvider<T> {
    fn bool(&self, value: T) -> ObjectRef;
}

pub trait IntegerProvider<T> {
    fn int(&self, value: T) -> ObjectRef;
}

pub trait FloatProvider<T> {
    fn float(&self, value: T) -> ObjectRef;
}

pub trait IteratorProvider<T> {
    fn iter(&self, value: T) -> ObjectRef;
}

pub trait DictProvider<T> {
    fn dict(&self, value: T) -> ObjectRef;
}

pub trait StringProvider<T> {
    fn str(&self, value: T) -> ObjectRef;
}

pub trait BytesProvider<T> {
    fn bytes(&self, value: T) -> ObjectRef;
}

pub trait TupleProvider<T> {
    fn tuple(&self, value: T) -> ObjectRef;
}

pub trait ListProvider<T> {
    fn list(&self, value: T) -> ObjectRef;
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

pub trait CodeProvider<T> {
    fn code(&self, value: T) -> ObjectRef;
}

pub trait FrameProvider<T> {
    fn frame(&self, value: T) -> ObjectRef;
}

pub trait ModuleProvider<T> {
    fn module(&self, value: T) -> ObjectRef;
}


pub trait DefaultBooleanProvider {
    fn default_bool(&self) -> ObjectRef;
}

pub trait DefaultIntegerProvider {
    fn default_int(&self) -> ObjectRef;
}

pub trait DefaultFloatProvider {
    fn default_float(&self) -> ObjectRef;
}

pub trait DefaultIteratorProvider {
    fn default_iter(&self) -> ObjectRef;
}

pub trait DefaultDictProvider {
    fn default_dict(&self) -> ObjectRef;
}

pub trait DefaultStringProvider {
    fn default_str(&self) -> ObjectRef;
}

pub trait DefaultBytesProvider {
    fn default_bytes(&self) -> ObjectRef;
}

pub trait DefaultTupleProvider {
    fn default_tuple(&self) -> ObjectRef;
}

pub trait DefaultListProvider {
    fn default_list(&self) -> ObjectRef;
}

pub trait DefaultPyTypeProvider {
    fn default_pytype(&self) -> ObjectRef;
}

pub trait DefaultObjectProvider {
    fn default_object(&self) -> ObjectRef;
}

pub trait DefaultFunctionProvider {
    fn default_function(&self) -> ObjectRef;
}

pub trait DefaultCodeProvider {
    fn default_code(&self) -> ObjectRef;
}

pub trait DefaultFrameProvider {
    fn default_frame(&self) -> ObjectRef;
}

pub trait DefaultModuleProvider {
    fn default_module(&self) -> ObjectRef;
}


pub trait ModuleFinder<T> {
    fn get_module(&self, value: T) -> RuntimeResult;
}

pub trait ModuleImporter<T> {
    fn import_module(&self, value: T) -> RuntimeResult;
}

