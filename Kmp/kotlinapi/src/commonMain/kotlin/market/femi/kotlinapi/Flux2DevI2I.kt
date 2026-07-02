package market.femi.kotlinapi

expect suspend fun flux2DevI2I(user: String, pass: String, imageB64: String, prompt: String): ByteArray
