package market.femi.api

actual suspend fun flux2DevI2I(user: String, pass: String, imageB64: String, prompt: String): ByteArray =
    runCancelable { addr -> FemiApiJvm.rustFfiFlux2DevI2i(user, pass, imageB64, prompt, addr) }
