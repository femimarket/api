package market.femi.api

actual suspend fun nanoBanana2(user: String, pass: String, prompt: String): ByteArray =
    runCancelable { addr -> FemiApiJvm.rustFfiNanoBanana2(user, pass, prompt, addr) }
