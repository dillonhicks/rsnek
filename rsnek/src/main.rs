extern crate rsnek_runtime;
use rsnek_runtime::runtime::{Interpreter, ThreadModel};

fn main() {
    Interpreter::start(ThreadModel::Generator);
}
