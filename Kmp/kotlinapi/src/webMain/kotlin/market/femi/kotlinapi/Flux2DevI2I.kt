@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.kotlinapi

import kotlinx.coroutines.await

actual suspend fun flux2DevI2I(user: String, pass: String, imageB64: String, prompt: String): ByteArray {
    return RustFfi.wasm_flux2_dev_i2i(user, pass, imageB64, prompt).await<JsAny?>()!!.uint8ToByteArray()
}
