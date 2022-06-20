extern crate tiny_keccak;

use crate::Precompile;
use std::error::Error;

pub struct Keccak256;

impl Precompile for Keccak256 {
    fn execute<I: AsRef<[u8]>, O: AsMut<[u8]>>(input: I, mut output: O) -> Result<(), ()> {
        tiny_keccak::Keccak::keccak256(input.as_ref(), output.as_mut());
        Ok(())
    }
}
