extern crate rattlesnake;
use rattlesnake::runtime::{Interpreter, ThreadModel};

fn main() {
    Interpreter::start(ThreadModel::Generator);
}

