package market.femi.api

expect suspend fun qwen3AsrFlash(user: String, pass: String, audio: ByteArray): String
