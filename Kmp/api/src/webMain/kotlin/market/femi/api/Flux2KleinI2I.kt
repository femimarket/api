@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.api

import kotlinx.coroutines.await
import kotlin.js.JsAny

actual suspend fun flux2KleinI2I(imageB64: String, image2B64: String, prompt: String): ByteArray {
    return rustFfi().wasm_flux2_klein_i2i(imageB64, image2B64, prompt).await<JsAny?>()!!.uint8ToByteArray()
}
