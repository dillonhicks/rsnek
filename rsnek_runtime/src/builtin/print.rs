use std::borrow::Borrow;

use runtime::Runtime;
use traits::{IteratorProvider, NoneProvider};

use result::{RuntimeResult};
use typedef::objectref::ObjectRef;
use typedef::builtin::Builtin;
use object::method::StringCast;
use typedef::native;


pub struct PrintFn;


impl PrintFn {
    pub fn create() -> (&'static str, native::Function) {
        info!("create builtin"; "function" => "print");
        let func: Box<native::WrapperFn> = Box::new(rs_builtin_print);
        ("print", native::Function::Wrapper(func))
    }
}


#[allow(unused_variables)]
fn rs_builtin_print(rt: &Runtime, pos_args: &ObjectRef,
                    starargs: &ObjectRef,
                    kwargs: &ObjectRef) -> RuntimeResult {
    info!("call"; "native_builtin" => "print");

    let mut strings: Vec<String> = Vec::new();
    let tuple_iterator = match native::Iterator::new(pos_args){
        Ok(iterator) => rt.iter(iterator),
        Err(_) => unreachable!(),
    };

    for objref in tuple_iterator {
        let boxed: &Box<Builtin> = objref.0.borrow();
        let s = match boxed.native_str() {
            Ok(string) => string,
            Err(err) => {
                warn!("Error during call"; "native_builtin" => "str");
                format!("{}", objref)
            }
        };

        strings.push(s);
    }

    // TODO: {T71} Wrap this in the "canblock" macro when implemented
    println!("\n{}", strings.join(" "));
    Ok(rt.none())
}
