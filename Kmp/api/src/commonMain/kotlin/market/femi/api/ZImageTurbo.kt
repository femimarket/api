package market.femi.api

expect suspend fun zImageTurbo(user: String, pass: String, prompt: String): ByteArray
