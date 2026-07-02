@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.api

import kotlinx.coroutines.await

actual suspend fun nanoBanana2(user: String, pass: String, prompt: String): ByteArray {
    return rustFfi().wasm_nano_banana2(user, pass, prompt).await<JsAny?>()!!.uint8ToByteArray()
}
