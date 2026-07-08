@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.api

import kotlinx.coroutines.await
import kotlin.js.JsAny

actual suspend fun zImageTurbo(prompt: String): ByteArray {
    return rustFfi().wasm_z_image_turbo(prompt).await<JsAny?>()!!.uint8ToByteArray()
}
