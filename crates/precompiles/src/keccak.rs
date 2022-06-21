extern crate tiny_keccak;

use crate::*;

pub struct Keccak256;

const BASE_FEE: usize = 60;
const WORD_FEE: usize = 12;

impl Precompile for Keccak256 {
    fn analyze<I: AsRef<[u8]>>(input: I) -> Result<AnalysisResult, Error> {
        let input_len = input.as_ref().len();
        let cost = BASE_FEE + ((input_len + 31) / 32) * WORD_FEE;
        Ok(AnalysisResult {
            gas_used: cost.try_into().unwrap(),
            output_length: input_len,
        }) // FIXME
    }

    fn execute<I: AsRef<[u8]>, O: AsMut<[u8]>>(input: I, mut output: O) -> Result<usize, Error> {
        tiny_keccak::Keccak::keccak256(input.as_ref(), output.as_mut());
        Ok(output.as_mut().len())
    }
}
