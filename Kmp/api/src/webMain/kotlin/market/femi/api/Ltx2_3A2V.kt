@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.api

import kotlinx.coroutines.await

actual suspend fun ltx2_3a2v(user: String, pass: String, imageB64: String, audioB64: String, prompt: String): ByteArray {
    return rustFfi().wasm_ltx2_3a2v(user, pass, imageB64, audioB64, prompt).await<JsAny?>()!!.uint8ToByteArray()
}
