package market.femi.api

actual suspend fun nanoBanana2(prompt: String): ByteArray =
    runCancelable { addr -> FemiApiJvm.rustFfiNanoBanana2(prompt, addr) }
