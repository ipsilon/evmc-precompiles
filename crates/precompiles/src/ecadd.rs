extern crate ethereum_bn128;

use crate::Precompile;
use std::error::Error;

pub struct ECAdd;

const BASE_FEE: i64 = 500;

impl Precompile for ECAdd {
    fn analyze<I: AsRef<[u8]>>(input: I) -> Result<(i64, usize), ()> {
        Ok((BASE_FEE, 64))
    }

    fn execute<I: AsRef<[u8]>, O: AsMut<[u8]>>(input: I, mut output: O) -> Result<(), ()> {
        // FIXME: remove the match and use the output slice directly
        let mut tmp = [0u8; 64];
        match ethereum_bn128::bn128_add(input.as_ref(), &mut tmp) {
            Ok(_) => {
                output.as_mut().copy_from_slice(&tmp);
                Ok(())
            }
            Err(_) => Err(()),
        }
    }
}
