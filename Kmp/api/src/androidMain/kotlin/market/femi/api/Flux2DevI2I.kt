package market.femi.api

actual suspend fun flux2DevI2I(imageB64: String, prompt: String): ByteArray =
    runCancelable { addr -> FemiApiJvm.rustFfiFlux2DevI2i(imageB64, prompt, addr) }
