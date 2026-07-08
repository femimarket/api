#ifndef RUST_FFI_H
#define RUST_FFI_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// ---------- API endpoints (fallback resolved inside Rust) ----------

uint8_t *rust_ffi_z_image_turbo(const char *prompt,
                                const uint8_t *cancel_flag,
                                size_t *out_len);

uint8_t *rust_ffi_qwen3_asr_flash(const char *audio_b64,
                                  const uint8_t *cancel_flag,
                                  size_t *out_len);

uint8_t *rust_ffi_qwen3_6_35b_a3b(const char *messages_json,
                                  const uint8_t *cancel_flag,
                                  size_t *out_len);

uint8_t *rust_ffi_nano_banana2(const char *prompt,
                               const uint8_t *cancel_flag,
                               size_t *out_len);

uint8_t *rust_ffi_flux2_pro(const char *prompt,
                            const uint8_t *cancel_flag,
                            size_t *out_len);

uint8_t *rust_ffi_flux2_dev_i2i(const char *image_b64,
                                const char *prompt,
                                const uint8_t *cancel_flag,
                                size_t *out_len);

uint8_t *rust_ffi_flux2_klein_i2i(const char *image_b64,
                                  const char *image2_b64,
                                  const char *prompt,
                                  const uint8_t *cancel_flag,
                                  size_t *out_len);

uint8_t *rust_ffi_ltx2_3a2v(const char *image_b64,
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

// ---------- qwen3_6_35b_a3b prompt-gen family (authless; return result string bytes) ----------

uint8_t *rust_ffi_qwen3_6_35b_a3b_0gen_music_video_prompt(const uint8_t *cancel_flag,
                                                          size_t *out_len);

uint8_t *rust_ffi_qwen3_6_35b_a3b_1gen_new_angle_from_xmp_image_prompt(const char *xmp_prompt,
                                                                       const uint8_t *cancel_flag,
                                                                       size_t *out_len);

uint8_t *rust_ffi_qwen3_6_35b_a3b_1gen_random_idea_from_xmp_prompt(const char *xmp_prompt,
                                                                   const uint8_t *cancel_flag,
                                                                   size_t *out_len);

uint8_t *rust_ffi_qwen3_6_35b_a3b_1gen_3_variants_from_xmp_prompt_as_jsonarray(const char *xmp_prompt,
                                                                               const uint8_t *cancel_flag,
                                                                               size_t *out_len);

uint8_t *rust_ffi_qwen3_6_35b_a3b_2gen_augment_idea_from_xmp_prompt_and_text(const char *xmp_prompt,
                                                                             const char *text,
                                                                             const uint8_t *cancel_flag,
                                                                             size_t *out_len);

uint8_t *rust_ffi_qwen3_6_35b_a3b_2gen_50_100_word_multishot_timestamped_prompt_from_2_xmp_image_prompts(const char *xmp_prompt,
                                                                                                         const char *xmp_prompt2,
                                                                                                         const uint8_t *cancel_flag,
                                                                                                         size_t *out_len);

uint8_t *rust_ffi_qwen3_6_35b_a3b_3gen_50_100_word_multishot_timestamped_prompt_from_3_xmp_image_prompts(const char *xmp_prompt,
                                                                                                         const char *xmp_prompt2,
                                                                                                         const char *xmp_prompt3,
                                                                                                         const uint8_t *cancel_flag,
                                                                                                         size_t *out_len);

uint8_t *rust_ffi_qwen3_6_35b_a3b_4gen_augment_idea_from_3_xmp_prompts_and_text(const char *xmp_prompt,
                                                                                const char *xmp_prompt2,
                                                                                const char *xmp_prompt3,
                                                                                const char *text,
                                                                                const uint8_t *cancel_flag,
                                                                                size_t *out_len);

// ---------- ProjectService FFI (Rust owns Documents/; every call takes a filename) ----------

void    psxmp_save_file(const char *name, const uint8_t *bytes, size_t len, const char *prompt, const char *model, const char *const *subject, int32_t subject_count, const char *project_name, const char *lyrics, const char *shot_number);
void    psxmp_save_audio(const char *name, const uint8_t *bytes, size_t len);
void    psxmp_like(const char *file, int32_t liked);
int32_t psxmp_get_all_generations(char *buf, int32_t buf_len);
int32_t psxmp_get_audio(char *buf, int32_t buf_len);
int32_t psxmp_get_project_name(const char *file, char *buf, int32_t buf_len);
int32_t psxmp_get_lyrics(const char *file, char *buf, int32_t buf_len);
int32_t psxmp_get_shot_number(const char *file, char *buf, int32_t buf_len);
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
