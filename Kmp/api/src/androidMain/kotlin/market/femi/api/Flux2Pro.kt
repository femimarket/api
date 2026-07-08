package market.femi.api

actual suspend fun flux2Pro(prompt: String): ByteArray =
    runCancelable { addr -> FemiApiJvm.rustFfiFlux2Pro(prompt, addr) }
