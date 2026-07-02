package market.femi.api

import kotlin.io.encoding.Base64
import kotlin.io.encoding.ExperimentalEncodingApi

@OptIn(ExperimentalEncodingApi::class)
actual suspend fun qwen3AsrFlash(user: String, pass: String, audio: ByteArray): String =
    runCancelable { addr -> FemiApiJvm.rustFfiQwen3AsrFlash(user, pass, Base64.encode(audio), addr) }.decodeToString()
