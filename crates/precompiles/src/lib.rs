pub mod ecadd;
pub mod keccak;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Error {
    InvalidInput,
    ShortInput,
}

pub trait Precompile {
    fn analyze<I: AsRef<[u8]>>(input: I) -> Result<(i64, usize), Error>;
    fn execute<I: AsRef<[u8]>, O: AsMut<[u8]>>(input: I, output: O) -> Result<(), Error>;
}
