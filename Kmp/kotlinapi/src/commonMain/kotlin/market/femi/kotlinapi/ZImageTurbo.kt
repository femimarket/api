package market.femi.kotlinapi

expect suspend fun zImageTurbo(user: String, pass: String, prompt: String): ByteArray
