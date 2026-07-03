@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.api

import kotlinx.coroutines.await
import kotlinx.serialization.json.Json
import kotlin.js.JsAny
import kotlin.js.JsBoolean
import kotlin.js.JsString
import kotlin.js.js
import kotlin.js.toBoolean
import kotlin.js.toJsString

/// Thin passthrough to the Rust wasm-bindgen exports. Rust owns OPFS I/O
/// and all metadata handling.
actual object ProjectService {
    actual suspend fun saveFile(data: ByteArray, named: String, prompt: String?, model: String?, subject: List<String>?) {
        rustFfi().psxmp_save_file(named, data.toUint8Array(), prompt, model, (subject ?: emptyList()).toJsArray()).await<JsAny?>()
    }

    actual suspend fun saveAudio(data: ByteArray, named: String) {
        rustFfi().psxmp_save_audio(named, data.toUint8Array()).await<JsAny?>()
    }

    actual suspend fun like(file: String, liked: Boolean) {
        rustFfi().psxmp_like(file, liked).await<JsAny?>()
    }

    actual suspend fun getAllGenerations(): List<String> {
        val json = rustFfi().psxmp_get_all_generations().await<JsString>().toString()
        return if (json.isEmpty()) emptyList() else Json.decodeFromString<List<String>>(json)
    }

    actual suspend fun getAudio(): String? =
        rustFfi().psxmp_get_audio().await<JsString?>()?.toString()

    actual suspend fun getPrompt(file: String): String? =
        rustFfi().psxmp_get_prompt(file).await<JsString?>()?.toString()

    actual suspend fun getModel(file: String): String? =
        rustFfi().psxmp_get_model(file).await<JsString?>()?.toString()

    actual suspend fun getSubject(file: String): List<String>? {
        val json = rustFfi().psxmp_get_subject(file).await<JsString?>()?.toString() ?: return null
        return if (json.isEmpty()) null else Json.decodeFromString<List<String>>(json)
    }

    actual suspend fun getLike(file: String): Boolean =
        rustFfi().psxmp_get_like(file).await<JsBoolean>().toBoolean()

    actual suspend fun getUrl(file: String): String = file

    actual suspend fun setCharacterCast(a: String, b: String) {
        rustFfi().psxmp_set_character_cast(a, b)
    }

    actual suspend fun getCharacterCast(): Pair<String, String>? {
        val json = rustFfi().psxmp_get_character_cast()?.toString() ?: return null
        val parts = Json.decodeFromString<List<String>>(json)
        return if (parts.size == 2) parts[0] to parts[1] else null
    }

    actual suspend fun clearCharacterCast() {
        rustFfi().psxmp_clear_character_cast()
    }

    actual suspend fun setImageEdit(file: String) {
        rustFfi().psxmp_set_image_edit(file)
    }

    actual suspend fun getImageEdit(): String? =
        rustFfi().psxmp_get_image_edit()?.toString()

    actual suspend fun clearImageEdit() {
        rustFfi().psxmp_clear_image_edit()
    }
}

// ---------- JS interop helpers ----------

private fun makeUint8Array(size: Int): JsAny = js("new Uint8Array(size)")
private fun uint8Set(arr: JsAny, index: Int, value: Byte) { js("arr[index] = value") }
private fun makeArray(): JsAny = js("[]")
private fun arrayPush(arr: JsAny, value: JsAny?) { js("arr.push(value)") }

private fun ByteArray.toUint8Array(): JsAny {
    val a = makeUint8Array(size)
    for (i in indices) uint8Set(a, i, this[i])
    return a
}

private fun List<String>.toJsArray(): JsAny {
    val a = makeArray()
    for (s in this) arrayPush(a, s.toJsString())
    return a
}
