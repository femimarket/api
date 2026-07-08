package market.femi.api

import kotlin.io.encoding.Base64
import kotlin.io.encoding.ExperimentalEncodingApi

@OptIn(ExperimentalEncodingApi::class)
actual suspend fun qwen3AsrFlash(audio: ByteArray): String =
    runCancelable { addr -> FemiApiJvm.rustFfiQwen3AsrFlash(Base64.encode(audio), addr) }.decodeToString()
