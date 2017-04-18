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

pub trait ModuleProvider<T> {
    fn module(&self, value: T) -> ObjectRef;
}

pub trait ModuleFinder<T> {
    fn get_module(&self, value: T) -> RuntimeResult;
}

pub trait ModuleImporter<T> {
    fn import_module(&self, value: T) -> RuntimeResult;
}
