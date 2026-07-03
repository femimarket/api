@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.api

import kotlinx.coroutines.await
import kotlin.js.JsAny

actual suspend fun zImageTurbo(user: String, pass: String, prompt: String): ByteArray {
    return rustFfi().wasm_z_image_turbo(user, pass, prompt).await<JsAny?>()!!.uint8ToByteArray()
}
