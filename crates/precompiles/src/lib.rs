pub mod ecadd;
pub mod keccak;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Error {
    InvalidInput,
    ShortInput,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct AnalysisResult {
    pub gas_used: i64,
    pub output_length: usize,
}

pub trait Precompile {
    fn analyze<I: AsRef<[u8]>>(input: I) -> Result<AnalysisResult, Error>;
    /// Returns the consumed output length.
    fn execute<I: AsRef<[u8]>, O: AsMut<[u8]>>(input: I, output: O) -> Result<usize, Error>;
}
