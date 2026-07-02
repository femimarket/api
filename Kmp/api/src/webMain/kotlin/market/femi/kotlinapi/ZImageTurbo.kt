@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.kotlinapi

import kotlinx.coroutines.await

actual suspend fun zImageTurbo(user: String, pass: String, prompt: String): ByteArray {
    return RustFfi.wasm_z_image_turbo(user, pass, prompt).await<JsAny?>()!!.uint8ToByteArray()
}
