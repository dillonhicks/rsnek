use object::method;

pub trait Coroutine: method::Await + method::Send + method::Throw + method::Close {}
