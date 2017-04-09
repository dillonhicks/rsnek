#![feature(const_fn)]
#![feature(associated_consts)]

#[macro_use]
extern crate rsnek_proc_macros;
extern crate num;

#[macro_use]
extern crate itertools;

mod ast;
mod macros;
mod token;
mod tokenizer;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
