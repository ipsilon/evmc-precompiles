extern crate evmc_precompiles;

use evmc_precompiles::ecadd::ECAdd;
use evmc_precompiles::keccak::Keccak256;
use evmc_precompiles::Precompile;

/*
fn allocate_output_data(output: Option<&Vec<u8>>) -> (*const u8, usize) {
    if let Some(buf) = output {
        let buf_len = buf.len();

        // Manually allocate heap memory for the new home of the output buffer.
        let memlayout = std::alloc::Layout::from_size_align(buf_len, 1).expect("Bad layout");
        let new_buf = unsafe { std::alloc::alloc(memlayout) };
        unsafe {
            // Copy the data into the allocated buffer.
            std::ptr::copy(buf.as_ptr(), new_buf, buf_len);
        }

        (new_buf as *const u8, buf_len)
    } else {
        (std::ptr::null(), 0)
    }
}

unsafe fn deallocate_output_data(ptr: *const u8, size: usize) {
    if !ptr.is_null() {
        let buf_layout = std::alloc::Layout::from_size_align(size, 1).expect("Bad layout");
        std::alloc::dealloc(ptr as *mut u8, buf_layout);
    }
}



int32_t identity_exec(const uint8_t* input, uint32_t input_size, uint8_t* output,
    [[maybe_unused]] uint32_t output_size) noexcept
*/

#[repr(C)]
pub struct AnalysisResult {
    gas_used: i64,
    output_length: usize,
}

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
            output_length: input.output_length,
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
) -> i32 {
    let result = ::std::panic::catch_unwind(|| {
        let input = unsafe { std::slice::from_raw_parts(input_ptr, input_size) };
        let mut output = unsafe { std::slice::from_raw_parts_mut(output_ptr, output_size) };
        if let Ok(_) = Keccak256::execute(&input, &mut output) {
            0
        } else {
            // Some error code
            -1
        }
    });

    if let Ok(result) = result {
        result
    } else {
        -1 // Some error code
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
) -> i32 {
    let result = ::std::panic::catch_unwind(|| {
        let input = unsafe { std::slice::from_raw_parts(input_ptr, input_size) };
        let mut output = unsafe { std::slice::from_raw_parts_mut(output_ptr, output_size) };
        if let Ok(_) = ECAdd::execute(&input, &mut output) {
            0
        } else {
            // Some error code
            -1
        }
    });

    if let Ok(result) = result {
        result
    } else {
        -1 // Some error code
    }
}
