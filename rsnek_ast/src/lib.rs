#![feature(const_fn)]
#![feature(associated_consts)]

extern crate num;
#[macro_use] extern crate itertools;
#[macro_use] extern crate nom;
#[macro_use] extern crate rsnek_proc_macros;

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
