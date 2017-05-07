use std::borrow::Borrow;

use itertools::Itertools;

use ::api::method::StringCast;
use ::api::RtObject as ObjectRef;
use ::resources::strings;
use ::api::result::{ObjectResult};
use ::runtime::Runtime;
use ::runtime::traits::{IteratorProvider, NoneProvider};
use ::modules::builtins::Type;
use ::system::primitives::{Signature, Func, FuncType};
use ::system::primitives as rs;


pub struct PrintFn;


impl PrintFn {
    pub fn create() -> rs::Func {
        trace!("create builtin"; "function" => "print");
        let callable: Box<rs::WrapperFn> = Box::new(rs_builtin_print);

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
                    kwargs: &ObjectRef) -> ObjectResult {
    trace!("call"; "native_builtin" => "print");

    let mut strings: Vec<String> = Vec::new();
    let tuple_iterator = match rs::Iterator::new(pos_args){
        Ok(iterator) => rt.iter(iterator),
        Err(_) => unreachable!(),
    };

     let output = &tuple_iterator.map(|object| {
            match object.native_str() {
                 Ok(string) => string,
                Err(err) => {
                    warn!("Error during call"; "native_builtin" => "str");
                    format!("{}", object)
                }
            }
        })
         .map(|s| format!(">>> {}",s ))
         .join("\n");

    // TODO: {T71} Wrap this in the "canblock" macro when implemented
    info!("rs_builtin_print";
        "output" => format!("\n{}\n", output));

    Ok(rt.none())
}
