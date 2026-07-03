@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.api

import kotlin.io.encoding.Base64
import kotlin.io.encoding.ExperimentalEncodingApi
import kotlinx.coroutines.await
import kotlin.js.JsString

@OptIn(ExperimentalEncodingApi::class)
actual suspend fun qwen3AsrFlash(user: String, pass: String, audio: ByteArray): String {
    return rustFfi().wasm_qwen3_asr_flash(user, pass, Base64.encode(audio)).await<JsString>().toString()
}
