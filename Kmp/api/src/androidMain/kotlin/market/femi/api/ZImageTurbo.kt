package market.femi.api

actual suspend fun zImageTurbo(prompt: String): ByteArray =
    runCancelable { addr -> FemiApiJvm.rustFfiZImageTurbo(prompt, addr) }
