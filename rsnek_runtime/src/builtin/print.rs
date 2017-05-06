use std::borrow::Borrow;

use runtime::Runtime;
use traits::{IteratorProvider, NoneProvider};

use result::{RuntimeResult};
use ::object::RtObject as ObjectRef;
use ::resource::strings;
use typedef::builtin::Builtin;
use object::method::StringCast;
use typedef::native::{self, Signature, Func, FuncType};


pub struct PrintFn;


impl PrintFn {
    pub fn create() -> native::Func {
        trace!("create builtin"; "function" => "print");
        let callable: Box<native::WrapperFn> = Box::new(rs_builtin_print);

        Func {
            name: String::from("print"),
            module: String::from(strings::BUILTINS_MODULE),
            callable: FuncType::Wrapper(callable),
            signature: Signature::new(
                &["value"], &[], Some("*objs"), Some("**opts"))
        }
    }
}


#[allow(unused_variables)]
fn rs_builtin_print(rt: &Runtime, pos_args: &ObjectRef,
                    starargs: &ObjectRef,
                    kwargs: &ObjectRef) -> RuntimeResult {
    trace!("call"; "native_builtin" => "print");

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

    let output = strings.iter().fold(String::new(), |acc, s| acc + "\n >>>  " + s);

    // TODO: {T71} Wrap this in the "canblock" macro when implemented
    info!("rs_builtin_print";
        "output" => format!("\n{}\n", output));

    Ok(rt.none())
}
