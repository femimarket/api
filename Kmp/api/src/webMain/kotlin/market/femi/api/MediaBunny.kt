@file:OptIn(ExperimentalWasmJsInterop::class)

package market.femi.api

// mediabunny binding (WebCodecs demux → encode → mux). Kotlin/Wasm can't build JS
// option-object literals or call `new` over a class reference directly, so — per
// the studiofemimarket pattern — the module's classes are bound as JsAny and
// instantiated / configured through small @JsFun glue. Only what clipAudio needs
// is bound. clipAudio is the web equivalent of the Swift Make-Video audio trim.

import kotlinx.coroutines.await
import org.khronos.webgl.ArrayBuffer
import org.khronos.webgl.Uint8Array
import org.khronos.webgl.get
import org.khronos.webgl.set
import kotlin.js.ExperimentalWasmJsInterop
import kotlin.js.JsAny
import kotlin.js.JsModule
import kotlin.js.Promise
import kotlin.js.unsafeCast

@JsModule("mediabunny")
external object MediaBunny {
    val Input: JsAny
    val BufferSource: JsAny
    val Output: JsAny
    val WavOutputFormat: JsAny
    val BufferTarget: JsAny
    val Conversion: JsAny
    val ALL_FORMATS: JsAny
}

// `new clazz(...)` — no direct `new` over a JsAny class ref in Kotlin/Wasm.
@JsFun("(clazz) => new clazz()")
external fun newInstance0(clazz: JsAny): JsAny

@JsFun("(clazz, arg) => new clazz(arg)")
external fun newInstance1(clazz: JsAny, arg: JsAny): JsAny

// option-object literals
@JsFun("(formats, source) => ({ formats, source })")
external fun inputOptions(formats: JsAny, source: JsAny): JsAny

@JsFun("(format, target) => ({ format, target })")
external fun outputOptions(format: JsAny, target: JsAny): JsAny

@JsFun("(input, output, start, end) => ({ input, output, trim: { start, end } })")
external fun conversionOptions(input: JsAny, output: JsAny, start: Double, end: Double): JsAny

// static + instance calls
@JsFun("(Conversion, options) => Conversion.init(options)")
external fun conversionInit(Conversion: JsAny, options: JsAny): Promise<JsAny?>

@JsFun("(conversion) => conversion.execute()")
external fun conversionExecute(conversion: JsAny): Promise<JsAny?>

@JsFun("(target) => target.buffer")
external fun bufferTargetBuffer(target: JsAny): JsAny?

/** Clip [audio] to the window `[startMs, startMs + durationMs)` and return the
 *  segment as WAV bytes — the web equivalent of the Swift AVAssetExportSession
 *  trim. mediabunny handles demux/encode/mux; trim bounds are in seconds. */
suspend fun clipAudio(audio: ByteArray, startMs: Int, durationMs: Int): ByteArray {
    val source = newInstance1(MediaBunny.BufferSource, audio.bytesToU8())
    val input = newInstance1(MediaBunny.Input, inputOptions(MediaBunny.ALL_FORMATS, source))
    val format = newInstance0(MediaBunny.WavOutputFormat)
    val target = newInstance0(MediaBunny.BufferTarget)
    val output = newInstance1(MediaBunny.Output, outputOptions(format, target))
    val conversion = conversionInit(
        MediaBunny.Conversion,
        conversionOptions(input, output, startMs / 1000.0, (startMs + durationMs) / 1000.0),
    ).await<JsAny?>()!!
    conversionExecute(conversion).await<JsAny?>()
    val buffer = bufferTargetBuffer(target) ?: return ByteArray(0)
    return abToBytes(buffer.unsafeCast<ArrayBuffer>())
}

private fun ByteArray.bytesToU8(): Uint8Array {
    val out = Uint8Array(size)
    for (i in indices) out[i] = this[i]
    return out
}

private fun abToBytes(buffer: ArrayBuffer): ByteArray {
    val view = Uint8Array(buffer)
    return ByteArray(view.length) { view[it] }
}
