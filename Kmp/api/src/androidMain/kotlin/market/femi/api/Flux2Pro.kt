package market.femi.api

actual suspend fun flux2Pro(user: String, pass: String, prompt: String): ByteArray =
    runCancelable { addr -> FemiApiJvm.rustFfiFlux2Pro(user, pass, prompt, addr) }
