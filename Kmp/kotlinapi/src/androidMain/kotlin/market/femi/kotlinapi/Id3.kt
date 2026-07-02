package market.femi.kotlinapi

actual suspend fun extractSylt(bytes: ByteArray): String = Id3Jvm.extractSylt(bytes)
