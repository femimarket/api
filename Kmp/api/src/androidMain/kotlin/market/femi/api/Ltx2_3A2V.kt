package market.femi.api

actual suspend fun ltx2_3a2v(user: String, pass: String, imageB64: String, audioB64: String, prompt: String): ByteArray =
    runCancelable { addr -> FemiApiJvm.rustFfiLtx23a2v(user, pass, imageB64, audioB64, prompt, addr) }
