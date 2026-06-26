#ifndef RUST_FFI_H
#define RUST_FFI_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

uint8_t *rust_ffi_z_image_turbo(const char *token,
                                const char *prompt,
                                const uint8_t *cancel_flag,
                                uint16_t *out_status,
                                size_t *out_len);

#ifdef __cplusplus
}
#endif

#endif
