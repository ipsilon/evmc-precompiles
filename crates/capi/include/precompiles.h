#include <stdint.h>

enum evmc_status_code {
    SUCCESS = 0,
    INVALID_INPUT = -1,
    SHORT_INPUT = -2,
};

struct evmc_analysis_result {
    int64_t gas_used;
    uint32_t output_length;
};

struct evmc_analysis_result sample_analyze(const uint8_t* input_ptr, uint32_t input_size);
enum evmc_status_code sample_execute(const uint8_t* input_ptr, uint32_t input_size, uint8_t* output_ptr, uint32_t output_size);
