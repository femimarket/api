@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.api

import kotlinx.coroutines.await
import kotlinx.serialization.json.Json
import kotlin.js.JsString

actual suspend fun qwen3_6_35b_a3b(user: String, pass: String, messages: List<ChatMessage>): List<ChatMessage> {
    val json = rustFfi().wasm_qwen3_6_35b_a3b(user, pass, Json.encodeToString(messages)).await<JsString>().toString()
    return try { Json.decodeFromString<List<ChatMessage>>(json) }
    catch (e: Exception) { messages + ChatMessage(Role.Assistant, "Could not respond") }
}
