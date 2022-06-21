#include <stdint.h>
#include <stddef.h>

enum evmc_status_code {
    SUCCESS = 0,
    INVALID_INPUT = -1,
    SHORT_INPUT = -2,
};

typedef ptrdiff_t evmc_execution_status;

struct evmc_analysis_result {
    int64_t gas_used;
    size_t output_length;
};

struct evmc_analysis_result sample_analyze(const uint8_t* input_ptr, size_t input_size);
evmc_execution_status sample_execute(const uint8_t* input_ptr, size_t input_size, uint8_t* output_ptr, size_t output_size);
