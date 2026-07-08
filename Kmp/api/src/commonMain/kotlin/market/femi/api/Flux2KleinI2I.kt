package market.femi.api

expect suspend fun flux2KleinI2I(imageB64: String, image2B64: String, prompt: String): ByteArray
