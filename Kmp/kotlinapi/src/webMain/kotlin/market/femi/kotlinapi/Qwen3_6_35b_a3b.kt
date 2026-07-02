@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.kotlinapi

import kotlinx.coroutines.await
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

actual suspend fun qwen3_6_35b_a3b(user: String, pass: String, messages: List<ChatMessage>): List<ChatMessage> {
    val json = RustFfi.wasm_qwen3_6_35b_a3b(user, pass, Json.encodeToString(messages)).await<JsString>().toString()
    return try { Json.decodeFromString<List<ChatMessage>>(json) }
    catch (e: Exception) { messages + ChatMessage(Role.Assistant, "Could not respond") }
}
