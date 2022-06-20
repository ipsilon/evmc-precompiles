extern crate tiny_keccak;

use crate::Precompile;
use std::error::Error;

pub struct Keccak256;

impl Precompile for Keccak256 {
    fn execute<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, ()> {
        let mut hash = [0u8; 32];
        //        tiny_keccak::Keccak::keccak256(&input[..], &mut hash);
        tiny_keccak::Keccak::keccak256(input.as_ref(), &mut hash);
        Ok(hash.to_vec())
    }
}
