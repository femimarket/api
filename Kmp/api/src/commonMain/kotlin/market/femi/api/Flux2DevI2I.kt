package market.femi.api

expect suspend fun flux2DevI2I(user: String, pass: String, imageB64: String, prompt: String): ByteArray
