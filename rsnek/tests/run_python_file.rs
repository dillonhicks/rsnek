extern crate rsnek_runtime;

use rsnek_runtime::resource::strings;
use std::env;
use std::process::Command;


/// Sanity check to make sure the executable sort of works
#[test]
fn run_file() {

    let s: u64= env::vars_os().map(|v| {println!("{:?}", v); 1}).sum();
    let path = format!("../target/debug/{}", strings::PROGRAM);

    let result = Command::new(&path)
        .arg("tests/test.py")
        .status();

    assert!(result.is_ok());
    result.unwrap();
}