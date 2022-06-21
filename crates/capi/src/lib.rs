extern crate evmc_precompiles;

use evmc_precompiles::ecadd::ECAdd;
use evmc_precompiles::keccak::Keccak256;
use evmc_precompiles::{Error, Precompile};

mod sys;

use sys::*;

fn error_to_c(input: Error) -> evmc_status_code {
    match input {
        Error::InvalidInput => evmc_status_code::INVALID_INPUT,
        Error::ShortInput => evmc_status_code::SHORT_INPUT,
        _ => panic!(),
    }
}

type AnalysisResult = evmc_analysis_result;

impl AnalysisResult {
    fn failure() -> Self {
        Self {
            gas_used: -1,
            output_length: 0,
        }
    }
}

impl From<evmc_precompiles::AnalysisResult> for AnalysisResult {
    fn from(input: evmc_precompiles::AnalysisResult) -> AnalysisResult {
        AnalysisResult {
            gas_used: input.gas_used,
            output_length: input.output_length as u32,
        }
    }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn keccak256_analyze(input_ptr: *const u8, input_size: usize) -> AnalysisResult {
    let result = ::std::panic::catch_unwind(|| {
        let input = unsafe { std::slice::from_raw_parts(input_ptr, input_size) };
        if let Ok(result) = Keccak256::analyze(&input) {
            result.into()
        } else {
            AnalysisResult::failure()
        }
    });

    if let Ok(result) = result {
        result
    } else {
        AnalysisResult::failure()
    }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn keccak256_execute(
    input_ptr: *const u8,
    input_size: usize,
    output_ptr: *mut u8,
    output_size: usize,
) -> evmc_status_code {
    let result = ::std::panic::catch_unwind(|| {
        let input = unsafe { std::slice::from_raw_parts(input_ptr, input_size) };
        let mut output = unsafe { std::slice::from_raw_parts_mut(output_ptr, output_size) };
        let result = Keccak256::execute(&input, &mut output);
        if result.is_err() {
            error_to_c(result.err().unwrap())
        } else {
            evmc_status_code::SUCCESS
        }
    });

    if let Ok(result) = result {
        result
    } else {
        evmc_status_code::INVALID_INPUT
    }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn ecadd_analyze(input_ptr: *const u8, input_size: usize) -> AnalysisResult {
    let result = ::std::panic::catch_unwind(|| {
        let input = unsafe { std::slice::from_raw_parts(input_ptr, input_size) };
        if let Ok(result) = ECAdd::analyze(&input) {
            result.into()
        } else {
            AnalysisResult::failure()
        }
    });

    if let Ok(result) = result {
        result
    } else {
        AnalysisResult::failure()
    }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn ecadd_execute(
    input_ptr: *const u8,
    input_size: usize,
    output_ptr: *mut u8,
    output_size: usize,
) -> evmc_status_code {
    let result = ::std::panic::catch_unwind(|| {
        let input = unsafe { std::slice::from_raw_parts(input_ptr, input_size) };
        let mut output = unsafe { std::slice::from_raw_parts_mut(output_ptr, output_size) };
        let result = ECAdd::execute(&input, &mut output);
        if result.is_err() {
            error_to_c(result.err().unwrap())
        } else {
            evmc_status_code::SUCCESS
        }
    });

    if let Ok(result) = result {
        result
    } else {
        evmc_status_code::INVALID_INPUT
    }
}
