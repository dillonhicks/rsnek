extern crate librsnek;

use librsnek::resources::strings;
use std::env;
use std::process::Command;


/// Sanity check to make sure the executable sort of works
#[test]
fn run_file() {

    env::vars_os().map(|v| {println!("{:?}", v); 1}).sum::<usize>();
    let path = format!("../target/debug/{}", strings::PROGRAM);

    let result = Command::new(&path)
        .arg("tests/test.py")
        .status();

    assert!(result.is_ok());
    result.unwrap();
}