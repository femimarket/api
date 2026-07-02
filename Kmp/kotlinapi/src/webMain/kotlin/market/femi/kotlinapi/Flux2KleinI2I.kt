@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.kotlinapi

import kotlinx.coroutines.await

actual suspend fun flux2KleinI2I(user: String, pass: String, imageB64: String, image2B64: String, prompt: String): ByteArray {
    return RustFfi.wasm_flux2_klein_i2i(user, pass, imageB64, image2B64, prompt).await<JsAny?>()!!.uint8ToByteArray()
}
