@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.api

import kotlinx.coroutines.await
import kotlin.js.JsAny

actual suspend fun flux2Pro(user: String, pass: String, prompt: String): ByteArray {
    return rustFfi().wasm_flux2_pro(user, pass, prompt).await<JsAny?>()!!.uint8ToByteArray()
}
