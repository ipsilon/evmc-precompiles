extern crate ethereum_bn128;

use crate::*;

pub struct ECAdd;

const BASE_FEE: i64 = 500;

impl Precompile for ECAdd {
    fn analyze<I: AsRef<[u8]>>(input: I) -> Result<AnalysisResult, Error> {
        if input.as_ref().len() != 64 {
            Err(Error::ShortInput)
        } else {
            Ok(AnalysisResult {
                gas_used: BASE_FEE,
                output_length: 64,
            })
        }
    }

    fn execute<I: AsRef<[u8]>, O: AsMut<[u8]>>(input: I, mut output: O) -> Result<(), Error> {
        // FIXME: remove the match and use the output slice directly
        let mut tmp = [0u8; 64];
        match ethereum_bn128::bn128_add(input.as_ref(), &mut tmp) {
            Ok(_) => {
                output.as_mut().copy_from_slice(&tmp);
                Ok(())
            }
            Err(_) => Err(Error::InvalidInput),
        }
    }
}
