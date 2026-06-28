#ifndef RUST_FFI_H
#define RUST_FFI_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

uint8_t *rust_ffi_z_image_turbo(const char *user,
                                const char *password,
                                const char *prompt,
                                const uint8_t *cancel_flag,
                                uint16_t *out_status,
                                size_t *out_len);

uint8_t *rust_ffi_qwen3_asr_flash(const char *user,
                                  const char *password,
                                  const char *audio_b64,
                                  const uint8_t *cancel_flag,
                                  uint16_t *out_status,
                                  size_t *out_len);

uint8_t *rust_ffi_qwen3_6_35b_a3b(const char *user,
                                  const char *password,
                                  const char *messages_json,
                                  const uint8_t *cancel_flag,
                                  uint16_t *out_status,
                                  size_t *out_len);

uint8_t *rust_ffi_nano_banana2(const char *user,
                               const char *password,
                               const char *prompt,
                               const uint8_t *cancel_flag,
                               uint16_t *out_status,
                               size_t *out_len);

uint8_t *rust_ffi_flux2_pro(const char *user,
                            const char *password,
                            const char *prompt,
                            const uint8_t *cancel_flag,
                            uint16_t *out_status,
                            size_t *out_len);

uint8_t *rust_ffi_flux2_dev_i2i(const char *user,
                                const char *password,
                                const char *image_b64,
                                const char *prompt,
                                const uint8_t *cancel_flag,
                                uint16_t *out_status,
                                size_t *out_len);

uint8_t *rust_ffi_flux2_klein_i2i(const char *user,
                                  const char *password,
                                  const char *image_b64,
                                  const char *image2_b64,
                                  const char *prompt,
                                  const uint8_t *cancel_flag,
                                  uint16_t *out_status,
                                  size_t *out_len);

uint8_t *rust_ffi_ltx2_3a2v(const char *user,
                            const char *password,
                            const char *image_b64,
                            const char *audio_b64,
                            const char *prompt,
                            const uint8_t *cancel_flag,
                            uint16_t *out_status,
                            size_t *out_len);

#ifdef __cplusplus
}
#endif

#endif
