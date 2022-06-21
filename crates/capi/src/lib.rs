extern crate evmc_precompiles;

use evmc_precompiles::ecadd::ECAdd;
use evmc_precompiles::keccak::Keccak256;
use evmc_precompiles::{AnalysisResult, Error, Precompile};

mod sys;

use sys::*;

fn error_to_c(input: Error) -> evmc_execution_status {
    match input {
        Error::InvalidInput => evmc_status_code::INVALID_INPUT as isize, // FIXME
        Error::ShortInput => evmc_status_code::SHORT_INPUT as isize,     // FIXME
        _ => panic!(),
    }
}

fn success_to_c(len: usize) -> evmc_execution_status {
    len as isize // FIXME
}

fn failure_to_c() -> evmc_execution_status {
    evmc_status_code::INVALID_INPUT as isize // FIXME
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
            error_to_c(result.err().unwrap())
        } else {
            success_to_c(output_size)
        }
    });

    if let Ok(result) = result {
        result
    } else {
        failure_to_c()
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
            error_to_c(result.err().unwrap())
        } else {
            success_to_c(output_size)
        }
    });

    if let Ok(result) = result {
        result
    } else {
        failure_to_c()
    }
}
