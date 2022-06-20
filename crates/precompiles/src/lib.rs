use std::error::Error;

pub mod keccak;

pub trait Precompile {
    fn execute<I: AsRef<[u8]>, O: AsMut<[u8]>>(input: I, output: O) -> Result<(), ()>;
}
