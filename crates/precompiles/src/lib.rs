use std::error::Error;

pub mod keccak;

pub trait Precompile {
    fn execute<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, ()>;
}
