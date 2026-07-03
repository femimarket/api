@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.api

import kotlinx.coroutines.await
import kotlin.js.JsAny

actual suspend fun flux2DevI2I(user: String, pass: String, imageB64: String, prompt: String): ByteArray {
    return rustFfi().wasm_flux2_dev_i2i(user, pass, imageB64, prompt).await<JsAny?>()!!.uint8ToByteArray()
}
