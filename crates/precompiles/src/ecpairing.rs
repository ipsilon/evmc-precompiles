extern crate ethereum_bn128;

use crate::*;

pub struct ECPairing;

const BASE_FEE: usize = 100000;
const ELEMENT_FEE: usize = 80000;

impl Precompile for ECPairing {
    fn analyze<I: AsRef<[u8]>>(input: I) -> Result<AnalysisResult, Error> {
        let input_len = input.as_ref().len();
        if input_len % 192 != 0 {
            Err(Error::ShortInput)
        } else {
            let gas_used = BASE_FEE + (input_len / 192) * ELEMENT_FEE;
            Ok(AnalysisResult {
                gas_used: gas_used.try_into().unwrap(),
                output_length: 32,
            })
        }
    }

    fn execute<I: AsRef<[u8]>, O: AsMut<[u8]>>(input: I, mut output: O) -> Result<usize, Error> {
        if output.as_mut().len() % 192 != 0 {
            return Err(Error::ShortInput);
        }

        // FIXME: remove the match and use the output slice directly
        let mut tmp = [0u8; 32];
        match ethereum_bn128::bn128_pairing(input.as_ref(), &mut tmp) {
            Ok(_) => {
                output.as_mut().copy_from_slice(&tmp);
                Ok(64)
            }
            Err(_) => Err(Error::InvalidInput),
        }
    }
}
