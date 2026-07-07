@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.api

import kotlinx.coroutines.await
import kotlin.js.JsAny


actual suspend fun ltx2_3a2v_comfyui(comfyKey: String, imageB64: String, audioB64: String, prompt: String): ByteArray {
    return rustFfi().wasm_ltx2_3a2v_comfyui(comfyKey, imageB64, audioB64, prompt).await<JsAny?>()!!.uint8ToByteArray()
}
