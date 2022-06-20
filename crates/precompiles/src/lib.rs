use std::error::Error;

pub mod ecadd;
pub mod keccak;

pub trait Precompile {
    fn analyze<I: AsRef<[u8]>>(input: I) -> Result<(i64, usize), ()>;
    fn execute<I: AsRef<[u8]>, O: AsMut<[u8]>>(input: I, output: O) -> Result<(), ()>;
}
