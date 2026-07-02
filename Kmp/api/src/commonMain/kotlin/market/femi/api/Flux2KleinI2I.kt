package market.femi.api

expect suspend fun flux2KleinI2I(user: String, pass: String, imageB64: String, image2B64: String, prompt: String): ByteArray
