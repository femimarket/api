package market.femi.api

import kotlinx.serialization.decodeFromString
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

actual suspend fun qwen3_6_35b_a3b(messages: List<ChatMessage>): List<ChatMessage> {
    val data = runCancelable { addr -> FemiApiJvm.rustFfiQwen3635bA3b(Json.encodeToString(messages), addr) }
    return try { Json.decodeFromString<List<ChatMessage>>(data.decodeToString()) }
    catch (e: Exception) { messages + ChatMessage(Role.Assistant, "Could not respond") }
}
