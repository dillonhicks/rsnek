use bincode;
use serde::Serialize;

/// Take some input that implements `serde::Serialize` and convert it to
/// bincode format.
///
pub fn bincode<'a, T>(input: &'a T) -> Vec<u8> where T: Serialize {
    bincode::serialize(&input, bincode::Infinite).unwrap()
}