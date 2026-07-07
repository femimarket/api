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

uint8_t *rust_ffi_ltx2_3a2v_comfyui(const char *comfy_key,
                                    const char *image_b64,
                                    const char *audio_b64,
                                    const char *prompt,
                                    const uint8_t *cancel_flag,
                                    size_t *out_len);

// ---------- ProjectService FFI (Rust owns Documents/; every call takes a filename) ----------

void    psxmp_save_file(const char *name, const uint8_t *bytes, size_t len, const char *prompt, const char *model, const char *const *subject, int32_t subject_count);
void    psxmp_save_audio(const char *name, const uint8_t *bytes, size_t len);
void    psxmp_like(const char *file, int32_t liked);
int32_t psxmp_get_all_generations(char *buf, int32_t buf_len);
int32_t psxmp_get_audio(char *buf, int32_t buf_len);
int32_t psxmp_get_prompt(const char *file, char *buf, int32_t buf_len);
int32_t psxmp_get_model(const char *file, char *buf, int32_t buf_len);
int32_t psxmp_get_subject(const char *file, char *buf, int32_t buf_len);
int32_t psxmp_get_like(const char *file);
int32_t psxmp_read_property(const char *file, const char *ns, const char *name, char *buf, int32_t buf_len);
int32_t psxmp_get_url(const char *file, char *buf, int32_t buf_len);
void    psxmp_set_character_cast(const char *a, const char *b);
int32_t psxmp_get_character_cast(char *buf, int32_t buf_len);
void    psxmp_clear_character_cast(void);
void    psxmp_set_image_edit(const char *file);
int32_t psxmp_get_image_edit(char *buf, int32_t buf_len);
void    psxmp_clear_image_edit(void);

// ---------- ID3 SYLT FFI ----------

uint8_t *id3_ffi_extract_sylt(const uint8_t *bytes,
                              size_t bytes_len,
                              size_t *out_len);

#ifdef __cplusplus
}
#endif

#endif
