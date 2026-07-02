package market.femi.api

actual suspend fun extractSylt(bytes: ByteArray): String = Id3Jvm.extractSylt(bytes)
