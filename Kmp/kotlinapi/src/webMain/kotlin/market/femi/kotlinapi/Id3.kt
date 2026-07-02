@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.kotlinapi

/// Reads the first SYLT (synchronised lyrics) frame from MP3 `bytes` and returns
/// the timed lines as a JSON array string ("[]" when there are none).
actual suspend fun extractSylt(bytes: ByteArray): String {
    val arr = bytes.toUint8Array()
    return RustFfi.extract_sylt(arr).toString()
}

private fun makeUint8Array(size: Int): JsAny = js("new Uint8Array(size)")
private fun uint8Set(arr: JsAny, index: Int, value: Byte) { js("arr[index] = value") }

private fun ByteArray.toUint8Array(): JsAny {
    val a = makeUint8Array(size)
    for (i in indices) uint8Set(a, i, this[i])
    return a
}
