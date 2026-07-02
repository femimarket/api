package market.femi.kotlinapi

actual suspend fun flux2KleinI2I(user: String, pass: String, imageB64: String, image2B64: String, prompt: String): ByteArray =
    runCancelable { addr -> FemiApiJvm.rustFfiFlux2KleinI2i(user, pass, imageB64, image2B64, prompt, addr) }
