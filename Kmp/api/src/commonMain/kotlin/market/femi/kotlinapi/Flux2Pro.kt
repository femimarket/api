package market.femi.kotlinapi

expect suspend fun flux2Pro(user: String, pass: String, prompt: String): ByteArray
