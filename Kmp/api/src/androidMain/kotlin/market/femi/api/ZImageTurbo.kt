package market.femi.api

actual suspend fun zImageTurbo(user: String, pass: String, prompt: String): ByteArray =
    runCancelable { addr -> FemiApiJvm.rustFfiZImageTurbo(user, pass, prompt, addr) }
