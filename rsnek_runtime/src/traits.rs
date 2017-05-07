use ::result::ObjectResult;
use ::api::RtObject;


pub trait New<T> {
    fn new(value: T) -> Self;
}

pub trait NoneProvider {
    fn none(&self) -> RtObject;
}

pub trait BooleanProvider<T> {
    fn bool(&self, value: T) -> RtObject;
}

pub trait IntegerProvider<T> {
    fn int(&self, value: T) -> RtObject;
}

pub trait FloatProvider<T> {
    fn float(&self, value: T) -> RtObject;
}

pub trait IteratorProvider<T> {
    fn iter(&self, value: T) -> RtObject;
}

pub trait DictProvider<T> {
    fn dict(&self, value: T) -> RtObject;
}

pub trait StringProvider<T> {
    fn str(&self, value: T) -> RtObject;
}

pub trait BytesProvider<T> {
    fn bytes(&self, value: T) -> RtObject;
}

pub trait TupleProvider<T> {
    fn tuple(&self, value: T) -> RtObject;
}

pub trait ListProvider<T> {
    fn list(&self, value: T) -> RtObject;
}

pub trait PyTypeProvider<T> {
    fn pytype(&self, value: T) -> RtObject;
}

pub trait ObjectProvider<T> {
    fn object(&self, value: T) -> RtObject;
}

pub trait FunctionProvider<T> {
    fn function(&self, value: T) -> RtObject;
}

pub trait CodeProvider<T> {
    fn code(&self, value: T) -> RtObject;
}

pub trait FrameProvider<T> {
    fn frame(&self, value: T) -> RtObject;
}

pub trait ModuleProvider<T> {
    fn module(&self, value: T) -> RtObject;
}


pub trait DefaultBooleanProvider {
    fn default_bool(&self) -> RtObject;
}

pub trait DefaultIntegerProvider {
    fn default_int(&self) -> RtObject;
}

pub trait DefaultFloatProvider {
    fn default_float(&self) -> RtObject;
}

pub trait DefaultIteratorProvider {
    fn default_iter(&self) -> RtObject;
}

pub trait DefaultDictProvider {
    fn default_dict(&self) -> RtObject;
}

pub trait DefaultStringProvider {
    fn default_str(&self) -> RtObject;
}

pub trait DefaultBytesProvider {
    fn default_bytes(&self) -> RtObject;
}

pub trait DefaultTupleProvider {
    fn default_tuple(&self) -> RtObject;
}

pub trait DefaultListProvider {
    fn default_list(&self) -> RtObject;
}

pub trait DefaultPyTypeProvider {
    fn default_pytype(&self) -> RtObject;
}

pub trait DefaultObjectProvider {
    fn default_object(&self) -> RtObject;
}

pub trait DefaultFunctionProvider {
    fn default_function(&self) -> RtObject;
}

pub trait DefaultCodeProvider {
    fn default_code(&self) -> RtObject;
}

pub trait DefaultFrameProvider {
    fn default_frame(&self) -> RtObject;
}

pub trait DefaultModuleProvider {
    fn default_module(&self) -> RtObject;
}


pub trait ModuleFinder<T> {
    fn get_module(&self, value: T) -> ObjectResult;
}

pub trait ModuleImporter<T> {
    fn import_module(&self, value: T) -> ObjectResult;
}

