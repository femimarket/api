@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.kotlinapi

import kotlin.js.Promise

@JsModule("rust_ffi")
external object RustFfi {
    fun wasm_z_image_turbo(user: String, pass: String, prompt: String): Promise<JsAny?>
    fun wasm_nano_banana2(user: String, pass: String, prompt: String): Promise<JsAny?>
    fun wasm_flux2_pro(user: String, pass: String, prompt: String): Promise<JsAny?>
    fun wasm_flux2_dev_i2i(user: String, pass: String, imageB64: String, prompt: String): Promise<JsAny?>
    fun wasm_flux2_klein_i2i(user: String, pass: String, imageB64: String, image2B64: String, prompt: String): Promise<JsAny?>
    fun wasm_ltx2_3a2v(user: String, pass: String, imageB64: String, audioB64: String, prompt: String): Promise<JsAny?>
    fun wasm_qwen3_asr_flash(user: String, pass: String, audioB64: String): Promise<JsString>
    fun wasm_qwen3_6_35b_a3b(user: String, pass: String, messagesJson: String): Promise<JsString>
    fun extract_sylt(bytes: JsAny): JsString

    // ProjectService (OPFS + XMP)
    fun psxmp_embed(path: String, prompt: String?, model: String?, subjects: JsAny): Promise<JsNumber>
    fun psxmp_read_prompt(path: String): Promise<JsString?>
    fun psxmp_read_model(path: String): Promise<JsString?>
    fun psxmp_read_subject_count(path: String): Promise<JsNumber>
    fun psxmp_read_subject_at(path: String, index: Int): Promise<JsString?>
    fun psxmp_set_rating(path: String, rating: Int): Promise<JsNumber>
    fun psxmp_read_rating(path: String): Promise<JsNumber>
}

private fun uint8Length(arr: JsAny): Int = js("arr.length")
private fun uint8Byte(arr: JsAny, index: Int): Byte = js("arr[index]")

internal fun JsAny.uint8ToByteArray(): ByteArray {
    val a = this
    val n = uint8Length(a)
    return ByteArray(n) { uint8Byte(a, it) }
}
