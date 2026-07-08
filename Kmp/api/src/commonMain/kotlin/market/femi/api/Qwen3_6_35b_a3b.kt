package market.femi.api

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/** Role of a chat turn. Serialized value matches the wire format ("User" / "Assistant"). */
@Serializable
enum class Role {
    @SerialName("User") User,
    @SerialName("Assistant") Assistant,
}

@Serializable
data class ChatMessage(val role: Role, val content: String)

expect suspend fun qwen3_6_35b_a3b(messages: List<ChatMessage>): List<ChatMessage>
