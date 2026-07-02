@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.api

import kotlinx.coroutines.await
import kotlin.js.Promise

private val AUDIO_EXTS = setOf("mp3", "m4a", "wav", "aac", "caf", "aiff", "aif", "flac", "ogg", "opus")

actual object ProjectService {
    /// OPFS is a per-origin flat namespace — treat all files as living directly
    /// under the origin root.
    actual var documents: String = ""

    actual suspend fun saveFile(data: ByteArray, named: String, prompt: String?, model: String?, subject: List<String>?) {
        opfsWrite(named, data)
        if (prompt != null || model != null || subject != null) {
            rustFfi().psxmp_embed(named, prompt, model, (subject ?: emptyList()).toJsArray()).await<JsNumber>()
        }
    }

    actual suspend fun like(file: String, liked: Boolean) {
        rustFfi().psxmp_set_rating(file, if (liked) 5 else 0).await<JsNumber>()
    }

    actual suspend fun getAllGenerations(): List<String> = opfsListNames()

    actual suspend fun saveAudio(data: ByteArray, named: String) {
        getAllGenerations().filter(::isAudio).forEach { opfsDelete(it) }
        opfsWrite(named, data)
    }

    actual suspend fun getAudio(): String? = getAllGenerations().firstOrNull(::isAudio)

    actual suspend fun getPrompt(file: String): String? =
        rustFfi().psxmp_read_prompt(file).await<JsString?>()?.toString()?.takeIf { it.isNotEmpty() }

    actual suspend fun getModel(file: String): String? =
        rustFfi().psxmp_read_model(file).await<JsString?>()?.toString()?.takeIf { it.isNotEmpty() }

    actual suspend fun getSubject(file: String): List<String>? {
        val count = rustFfi().psxmp_read_subject_count(file).await<JsNumber>().toInt()
        if (count <= 0) return null
        val list = mutableListOf<String>()
        for (i in 0 until count) {
            rustFfi().psxmp_read_subject_at(file, i).await<JsString?>()?.toString()?.let { list.add(it) }
        }
        return list.ifEmpty { null }
    }

    actual suspend fun getLike(file: String): Boolean =
        rustFfi().psxmp_read_rating(file).await<JsNumber>().toInt() in 1..5

    actual fun getUrl(file: String): String = file

    // In-memory operation state (process lifetime).

    private var characterCast: Pair<String, String>? = null
    actual fun setCharacterCast(a: String, b: String) { characterCast = a to b }
    actual fun getCharacterCast(): Pair<String, String>? = characterCast
    actual fun clearCharacterCast() { characterCast = null }

    private var imageEdit: String? = null
    actual fun setImageEdit(file: String) { imageEdit = file }
    actual fun getImageEdit(): String? = imageEdit
    actual fun clearImageEdit() { imageEdit = null }
}

// ---------- OPFS interop (Kotlin/wasm-js) ----------

private fun opfsRootPromise(): Promise<JsAny> = js("navigator.storage.getDirectory()")
private fun opfsWriteJs(root: JsAny, name: String, bytes: JsAny): Promise<JsAny?> = js(
    "root.getFileHandle(name, { create: true }).then(h => h.createWritable()).then(w => w.write(bytes).then(() => w.close()))"
)
private fun opfsDeleteJs(root: JsAny, name: String): Promise<JsAny?> = js("root.removeEntry(name).catch(() => null)")
private fun opfsListJs(root: JsAny): Promise<JsAny> = js(
    "(function(){var out=[];var it=root.keys();function step(){return it.next().then(function(r){if(r.done)return out;out.push(r.value);return step();});}return step();})()"
)
private fun jsArrayLength(arr: JsAny): Int = js("arr.length")
private fun jsArrayGetString(arr: JsAny, index: Int): String = js("arr[index]")

private suspend fun opfsRoot(): JsAny = opfsRootPromise().await<JsAny>()

private suspend fun opfsWrite(name: String, bytes: ByteArray) {
    opfsWriteJs(opfsRoot(), name, bytes.toUint8Array()).await<JsAny?>()
}

private suspend fun opfsDelete(name: String) {
    opfsDeleteJs(opfsRoot(), name).await<JsAny?>()
}

private suspend fun opfsListNames(): List<String> {
    val arr = opfsListJs(opfsRoot()).await<JsAny>()
    val n = jsArrayLength(arr)
    return List(n) { jsArrayGetString(arr, it) }
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

private fun pathExtension(path: String): String {
    val dot = path.lastIndexOf('.')
    return if (dot < 0 || dot == path.length - 1) "" else path.substring(dot + 1)
}

private fun isAudio(path: String): Boolean =
    AUDIO_EXTS.contains(pathExtension(path).lowercase())
