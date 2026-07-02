#ifndef RUST_FFI_H
#define RUST_FFI_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// ---------- API endpoints (fallback resolved inside Rust) ----------

uint8_t *rust_ffi_z_image_turbo(const char *user,
                                const char *password,
                                const char *prompt,
                                const uint8_t *cancel_flag,
                                size_t *out_len);

uint8_t *rust_ffi_qwen3_asr_flash(const char *user,
                                  const char *password,
                                  const char *audio_b64,
                                  const uint8_t *cancel_flag,
                                  size_t *out_len);

uint8_t *rust_ffi_qwen3_6_35b_a3b(const char *user,
                                  const char *password,
                                  const char *messages_json,
                                  const uint8_t *cancel_flag,
                                  size_t *out_len);

uint8_t *rust_ffi_nano_banana2(const char *user,
                               const char *password,
                               const char *prompt,
                               const uint8_t *cancel_flag,
                               size_t *out_len);

uint8_t *rust_ffi_flux2_pro(const char *user,
                            const char *password,
                            const char *prompt,
                            const uint8_t *cancel_flag,
                            size_t *out_len);

uint8_t *rust_ffi_flux2_dev_i2i(const char *user,
                                const char *password,
                                const char *image_b64,
                                const char *prompt,
                                const uint8_t *cancel_flag,
                                size_t *out_len);

uint8_t *rust_ffi_flux2_klein_i2i(const char *user,
                                  const char *password,
                                  const char *image_b64,
                                  const char *image2_b64,
                                  const char *prompt,
                                  const uint8_t *cancel_flag,
                                  size_t *out_len);

uint8_t *rust_ffi_ltx2_3a2v(const char *user,
                            const char *password,
                            const char *image_b64,
                            const char *audio_b64,
                            const char *prompt,
                            const uint8_t *cancel_flag,
                            size_t *out_len);

// ---------- ProjectService XMP FFI ----------

int psxmp_embed(const char *path,
                const char *prompt,
                const char *model,
                const char *const *subject,
                int subject_count);

int psxmp_read_prompt(const char *path, char *buf, int buf_len);
int psxmp_read_model(const char *path, char *buf, int buf_len);
int psxmp_read_subject_count(const char *path);
int psxmp_read_subject_at(const char *path, int index, char *buf, int buf_len);
int psxmp_set_rating(const char *path, int rating);
int psxmp_read_rating(const char *path);
int psxmp_read_property(const char *path,
                        const char *namespace_uri,
                        const char *property_name,
                        char *buf,
                        int buf_len);

// ---------- ID3 SYLT FFI ----------

uint8_t *id3_ffi_extract_sylt(const uint8_t *bytes,
                              size_t bytes_len,
                              size_t *out_len);

#ifdef __cplusplus
}
#endif

#endif
