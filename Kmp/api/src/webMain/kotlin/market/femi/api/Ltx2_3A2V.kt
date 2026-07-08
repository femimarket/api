@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.api

import kotlinx.coroutines.await
import kotlin.js.JsAny


actual suspend fun ltx2_3a2v(imageB64: String, audioB64: String, prompt: String): ByteArray {
    return rustFfi().wasm_ltx2_3a2v(imageB64, audioB64, prompt).await<JsAny?>()!!.uint8ToByteArray()
}
