extern crate evmc_precompiles;

use evmc_precompiles::ecadd::ECAdd;
use evmc_precompiles::keccak::Keccak256;
use evmc_precompiles::{AnalysisResult, Error, Precompile};

mod sys;

use sys::*;

impl From<Error> for evmc_execution_status {
    fn from(input: Error) -> Self {
        match input {
            Error::InvalidInput => evmc_execution_status {
                code: evmc_status_code::INVALID_INPUT,
                output_length: 0,
            },
            Error::ShortInput => evmc_execution_status {
                code: evmc_status_code::SHORT_INPUT,
                output_length: 0,
            },
            _ => panic!(),
        }
    }
}

impl evmc_execution_status {
    fn success(len: usize) -> Self {
        Self {
            code: evmc_status_code::SUCCESS,
            output_length: len,
        }
    }

    fn failure() -> Self {
        Self {
            code: evmc_status_code::INVALID_INPUT,
            output_length: 0,
        }
    }
}

impl evmc_analysis_result {
    fn failure() -> Self {
        Self {
            gas_used: -1,
            output_length: 0,
        }
    }
}

impl From<AnalysisResult> for evmc_analysis_result {
    fn from(input: AnalysisResult) -> Self {
        Self {
            gas_used: input.gas_used,
            output_length: input.output_length,
        }
    }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn keccak256_analyze(
    input_ptr: *const u8,
    input_size: usize,
) -> evmc_analysis_result {
    let result = ::std::panic::catch_unwind(|| {
        let input = unsafe { std::slice::from_raw_parts(input_ptr, input_size) };
        if let Ok(result) = Keccak256::analyze(&input) {
            result.into()
        } else {
            evmc_analysis_result::failure()
        }
    });

    if let Ok(result) = result {
        result
    } else {
        evmc_analysis_result::failure()
    }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn keccak256_execute(
    input_ptr: *const u8,
    input_size: usize,
    output_ptr: *mut u8,
    output_size: usize,
) -> evmc_execution_status {
    let result = ::std::panic::catch_unwind(|| {
        let input = unsafe { std::slice::from_raw_parts(input_ptr, input_size) };
        let mut output = unsafe { std::slice::from_raw_parts_mut(output_ptr, output_size) };
        let result = Keccak256::execute(&input, &mut output);
        if result.is_err() {
            result.err().unwrap().into()
        } else {
            evmc_execution_status::success(result.unwrap())
        }
    });

    if let Ok(result) = result {
        result
    } else {
        evmc_execution_status::failure()
    }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn ecadd_analyze(input_ptr: *const u8, input_size: usize) -> evmc_analysis_result {
    let result = ::std::panic::catch_unwind(|| {
        let input = unsafe { std::slice::from_raw_parts(input_ptr, input_size) };
        if let Ok(result) = ECAdd::analyze(&input) {
            result.into()
        } else {
            evmc_analysis_result::failure()
        }
    });

    if let Ok(result) = result {
        result
    } else {
        evmc_analysis_result::failure()
    }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn ecadd_execute(
    input_ptr: *const u8,
    input_size: usize,
    output_ptr: *mut u8,
    output_size: usize,
) -> evmc_execution_status {
    let result = ::std::panic::catch_unwind(|| {
        let input = unsafe { std::slice::from_raw_parts(input_ptr, input_size) };
        let mut output = unsafe { std::slice::from_raw_parts_mut(output_ptr, output_size) };
        let result = ECAdd::execute(&input, &mut output);
        if result.is_err() {
            result.err().unwrap().into()
        } else {
            evmc_execution_status::success(result.unwrap())
        }
    });

    if let Ok(result) = result {
        result
    } else {
        evmc_execution_status::failure()
    }
}
