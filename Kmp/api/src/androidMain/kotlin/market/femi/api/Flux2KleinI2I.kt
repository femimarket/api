package market.femi.api

actual suspend fun flux2KleinI2I(imageB64: String, image2B64: String, prompt: String): ByteArray =
    runCancelable { addr -> FemiApiJvm.rustFfiFlux2KleinI2i(imageB64, image2B64, prompt, addr) }
