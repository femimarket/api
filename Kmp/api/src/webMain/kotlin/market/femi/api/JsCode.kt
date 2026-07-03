@file:OptIn(ExperimentalWasmJsInterop::class)

package market.femi.api

import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.toComposeImageBitmap
import kotlinx.coroutines.await
import org.jetbrains.skia.Image as SkiaImage
import kotlinx.coroutines.suspendCancellableCoroutine
import org.khronos.webgl.ArrayBuffer
import org.khronos.webgl.DataView
import org.khronos.webgl.Int8Array
import org.khronos.webgl.get
import org.khronos.webgl.set
import org.w3c.dom.MessagePort
import org.w3c.dom.Worker
import kotlin.collections.toByteArray
import kotlin.coroutines.resume
import kotlin.coroutines.resumeWithException
import kotlin.js.ExperimentalWasmJsInterop
import kotlin.js.JsAny
import kotlin.js.JsArray
import kotlin.js.JsName
import kotlin.js.JsString
import kotlin.js.Promise
import kotlin.js.definedExternally
import kotlin.js.get
import kotlin.js.js
import kotlin.js.length
import kotlin.js.toJsString
import kotlin.js.unsafeCast

external val window: Window

external interface Window : JsAny {
    val navigator: Navigator
    val location: WindowLocation
    fun showOpenFilePicker(options: JsAny? = definedExternally): Promise<JsArray<FileSystemFileHandle>>
    fun showDirectoryPicker(options: JsAny? = definedExternally): Promise<FileSystemDirectoryHandle>
}

external interface WindowLocation : JsAny {
    var href: String
}

@JsFun("function(modeStr) { return { mode: modeStr }; }")
external fun getDirectoryPickerOptions(modeStr: String): JsAny

suspend fun Window.pickDirectory(mode: String = "readwrite"): FileSystemDirectoryHandle {
    val options = getDirectoryPickerOptions(mode)
    return this.showDirectoryPicker(options).await()
}

external interface Navigator : JsAny {
    val storage: StorageManager
    val hardwareConcurrency: Int?
}

external interface StorageManager : JsAny {
    fun getDirectory(): Promise<FileSystemDirectoryHandle>
    fun estimate(): Promise<StorageEstimate>
}

external interface StorageEstimate : JsAny {
    val usage: Double?
    val quota: Double?
}

external interface FileSystemHandle : JsAny {
    val kind: JsString
    val name: JsString
    fun queryPermission(options: JsAny? = definedExternally): Promise<JsString> // 👉 ADD THIS
}

@JsFun("function(id, handle) { return { id: id, handle: handle }; }")
external fun createHandleRecord(id: String, handle: FileSystemDirectoryHandle): JsAny

@JsFun("function(record) { return record ? record.handle : null; }")
external fun extractHandle(record: JsAny?): FileSystemDirectoryHandle?




external interface FileSystemFileHandle : FileSystemHandle {
    fun getFile(): Promise<File>
    fun createWritable(options: JsAny? = definedExternally): Promise<FileSystemWritableFileStream>
}

external interface FileSystemDirectoryHandle : FileSystemHandle {
    fun getFileHandle(
        name: String,
        options: JsAny? = definedExternally
    ): Promise<FileSystemFileHandle>

    fun getDirectoryHandle(
        name: String,
        options: JsAny? = definedExternally
    ): Promise<FileSystemDirectoryHandle>

    fun removeEntry(name: String): Promise<JsAny?>
    fun values(): AsyncIterator
}


external interface FileSystemWritableFileStream : JsAny {
    fun write(data: JsString): Promise<JsAny?>
    fun write(data: JsAny): Promise<JsAny?>
    fun close(): Promise<JsAny?>
}

external interface JsIteratorResult : JsAny {
    val done: Boolean
    val value: FileSystemHandle?
}

external interface AsyncIterator : JsAny {
    fun next(): Promise<JsIteratorResult>
}

@JsName("URL")
external object WebURL : JsAny {
    fun createObjectURL(blob: File): JsString
    fun revokeObjectURL(url: JsString)
}

external class TextDecoderStream : JsAny

external interface ReadableStream : JsAny {
    fun pipeThrough(transform: JsAny): ReadableStream
    fun getReader(): ReadableStreamDefaultReader
}

external interface ReadableStreamDefaultReader : JsAny {
    fun read(): Promise<ReadableStreamReadResult>
    fun cancel(): Promise<JsAny?>
}

external interface ReadableStreamReadResult : JsAny {
    val done: Boolean
    val value: JsString? // TextDecoderStream guarantees this is a String, not raw bytes
}

external interface File : JsAny {
    val name: JsString
    val size: Double
    val lastModified: Double // millis since epoch — the web's file creation date
    fun text(): Promise<JsString>
    fun arrayBuffer(): Promise<ArrayBuffer>
    fun stream(): ReadableStream // <--- This fixes the Unresolved Reference
}

external interface FileSystemWritableOptions : JsAny {
    var keepExistingData: Boolean
}

external interface WriteParams : JsAny {
    var type: String
    var position: Double
    var data: JsString
}

// @JsFun tells the Kotlin/Wasm compiler to generate the JS object factory automatically
@JsFun("function(keep) { return { keepExistingData: keep }; }")
external fun createWritableOptions(keep: Boolean): FileSystemWritableOptions

@JsFun("function(pos, dataStr) { return { type: 'write', position: pos, data: dataStr }; }")
external fun createWriteCommand(pos: Double, dataStr: JsString): WriteParams


// Inline JS helpers for options objects
fun getFileOptions(create: Boolean): JsAny = js("({ create: create })")
fun getPickerOptions(multiple: Boolean): JsAny = js("({ multiple: multiple })")

// --- Core OPFS Implementation ---

suspend fun getOpfs(): FileSystemDirectoryHandle {
//    val v = requireNotNull(dir)
//    return v
    return window.navigator.storage.getDirectory().await()
}

suspend fun getFileHandle(filename: String): FileSystemFileHandle {
    val root = getOpfs()
    return root.getFileHandle(filename, getFileOptions(true)).await()
}

suspend fun getFileObject(handle: FileSystemFileHandle): File {
    return handle.getFile().await()
}

suspend fun readFileContent(handle: FileSystemFileHandle): String {
    val file = getFileObject(handle)
    return file.text().await().toString()
}

suspend fun readFileBytes(filename: String): ArrayBuffer {
    val handle = getFileHandle(filename)
    val file = getFileObject(handle)
    return file.arrayBuffer().await()
}

fun ArrayBuffer.toByteArray(): ByteArray {
    val int8Array = Int8Array(this)
    return List(int8Array.length) { i ->
        int8Array[i]
    }.toByteArray()
}

/// Last-modified time (millis since epoch) of an OPFS file — the web's stand-in
/// for a file creation date, since ProjectService.getUrl is just the name and
/// exposes no timestamps. Used to order/batch generated runs on reload.
suspend fun readFileMtime(filename: String): Double = runCatching {
    getFileObject(getFileHandle(filename)).lastModified
}.getOrDefault(0.0)

/// Decode encoded image bytes (PNG/JPEG/WebP/…) into a Compose ImageBitmap via
/// Skia — the web equivalent of Swift's `UIImage(data:)`. Null when the bytes
/// aren't a decodable image.
fun decodeImageBitmap(bytes: ByteArray): ImageBitmap? = runCatching {
    SkiaImage.makeFromEncoded(bytes).toComposeImageBitmap()
}.getOrNull()

suspend fun writeFileContent(handle: FileSystemFileHandle, data: String) {
    val writable = handle.createWritable().await()
    writable.write(data.toJsString()).await()
    writable.close().await()
}

suspend fun readFileBlobUrl(handle: FileSystemFileHandle): String {
    val file = getFileObject(handle)
    return WebURL.createObjectURL(file).toString()
}

suspend fun deleteFile(filename: String) {
    val root = getOpfs()
    root.removeEntry(filename).await()
}

suspend fun readFile(filename: String): String {
    val handle = getFileHandle(filename)
    return readFileContent(handle)
}

suspend fun readBlob(filename: String): String {
    val handle = getFileHandle(filename)
    return readFileBlobUrl(handle)
}

/// Release an object URL minted by [readBlob] / [readFileBlobUrl] so its backing
/// blob can be freed. Call from an onDispose when a preview closes.
fun revokeBlobUrl(url: String) = WebURL.revokeObjectURL(url.toJsString())

suspend fun writeFile(filename: String, content: String) {
    val handle = getFileHandle(filename)
    writeFileContent(handle, content)
}

suspend fun readDirFiles(dirName: String = ""): List<String> {
    val root = getOpfs()

    val targetDir = if (dirName.isEmpty()) {
        root
    } else {
        root.getDirectoryHandle(dirName).await()
    }

    val files = mutableListOf<String>()
    val iterator = targetDir.values()

    while (true) {
        val nextResult = iterator.next().await()

        if (nextResult.done) {
            break
        }

        // Safely cast the raw JsAny? into the specific FileSystemHandle
        val handle = nextResult.value?.unsafeCast<FileSystemHandle>()

        if (handle != null && handle.kind.toString() == "file") {
            files.add(handle.name.toString())
        }
    }

    return files
}

// --- File Picking & Importing ---

suspend fun pickFile(): List<String> {
    val handles = window.showOpenFilePicker(getPickerOptions(multiple = false)).await()

    if (handles.length == 0) return emptyList()

    val savedFiles = mutableListOf<String>()

    for (i in 0 until handles.length) {
        val srcHandle = handles[i] ?: continue
        val blob = srcHandle.getFile().await()
        val filename = blob.name.toString()

        // Copy the chosen file to OPFS
        val destHandle = getFileHandle(filename)
        val writable = destHandle.createWritable().await()
        writable.write(blob)
            .await() // File system writable stream directly accepts the File/Blob JS object
        writable.close().await()

        savedFiles.add(filename)
    }

    return savedFiles
}

suspend fun pickFiles(): List<String> {
    val handles = window.showOpenFilePicker(getPickerOptions(multiple = true)).await()

    if (handles.length == 0) return emptyList()

    val savedFiles = mutableListOf<String>()

    for (i in 0 until handles.length) {
        val srcHandle = handles[i] ?: continue
        val blob = srcHandle.getFile().await()
        val filename = blob.name.toString()

        // Copy the chosen file to OPFS
        val destHandle = getFileHandle(filename)
        val writable = destHandle.createWritable().await()
        writable.write(blob).await()
        writable.close().await()

        savedFiles.add(filename)
    }

    return savedFiles
}


external class AudioContext : JsAny {
    val currentTime: Double
    val state: String
    val destination: AudioDestinationNode

    fun resume(): Promise<JsAny?>
    fun suspend(): Promise<JsAny?>
    fun close(): Promise<JsAny?>
    fun createBufferSource(): AudioBufferSourceNode
    fun decodeAudioData(audioData: JsAny): Promise<AudioBuffer>
}

external interface AudioNode : JsAny {
    fun connect(destinationNode: AudioNode)
    fun disconnect()
}

external interface AudioDestinationNode : AudioNode

external interface AudioBuffer : JsAny {
    val duration: Double
    val length: Int
    val numberOfChannels: Int
    val sampleRate: Double
}

external interface AudioBufferSourceNode : AudioNode {
    var buffer: AudioBuffer?
    var loop: Boolean
    var loopStart: Double
    var loopEnd: Double
    var playbackRate: AudioParam

    fun start(
        whenTime: Double = definedExternally,
        offset: Double = definedExternally,
        duration: Double = definedExternally
    )
    fun stop(whenTime: Double = definedExternally)
}

external interface AudioParam : JsAny {
    var value: Double
    fun setValueAtTime(value: Double, startTime: Double)
    fun linearRampToValueAtTime(value: Double, endTime: Double)
}

@OptIn(ExperimentalWasmJsInterop::class)
external val navigator: Navigator

class AudioMathState(
    var baseTime: Double = 0.0,
    var startOffset: Double = 0.0,
    var loopStart: Double = 0.0,
    var loopEnd: Double = 0.0,
){
    fun getLoopedTime(trackTime: Double): Double {
        return loopStart + ((trackTime - loopEnd) % (loopEnd - loopStart))
    }
}


@OptIn(ExperimentalWasmJsInterop::class)
external interface OffscreenCanvas : JsAny

@OptIn(ExperimentalWasmJsInterop::class)
@JsFun("(port) => [port]")
external fun singleTransferList(port: MessagePort): JsArray<JsAny>

@OptIn(ExperimentalWasmJsInterop::class)
@JsFun("(canvas, ports) => [canvas, ...ports]")
external fun buildMainTransferList(canvas: OffscreenCanvas, ports: JsArray<MessagePort>): JsArray<JsAny>

@OptIn(ExperimentalWasmJsInterop::class)
@JsFun("() => []")
external fun <T : JsAny> emptyJsArray(): JsArray<T>

@OptIn(ExperimentalWasmJsInterop::class)
@JsFun("(arr, item) => { arr.push(item); }")
external fun <T : JsAny> jsArrayPush(arr: JsArray<T>, item: T)


@OptIn(ExperimentalWasmJsInterop::class)
external class SharedArrayBuffer(byteLength: Int) : JsAny

@JsFun("(buf) => new Uint8Array(buf).fill(32)")
external fun fillSpaces(buf: SharedArrayBuffer)


@JsFun("function() { return Date.now(); }")
external fun jsDateNow(): Double


var dir by mutableStateOf<FileSystemDirectoryHandle?>(null)


@JsFun("(buf, text) => new Uint8Array(buf).set(new TextEncoder().encode(text))")
external fun copyTextToBuffer(buf: SharedArrayBuffer, text: String)

fun SharedArrayBuffer.writeText(text: String) {
    fillSpaces(this)
    require(text.length <= Int8Array(this.unsafeCast<ArrayBuffer>()).length) {
        "text too large for shared buffer (${text.length} chars)"
    }
    copyTextToBuffer(this, text)
}

fun SharedArrayBuffer.writeFloat64(offset: Int, value: Double) {
    DataView(this.unsafeCast<ArrayBuffer>()).setFloat64(offset, value, littleEndian = true)
}

/*
@OptIn(ExperimentalWasmJsInterop::class)
suspend fun <T : JsAny?> Promise<T>.await(): T = suspendCancellableCoroutine { cont ->
    this.then(
        onFulfilled = { result ->
            cont.resume(result)
            null // Return null to satisfy JS Promise signature
        },
        onRejected = { error ->
            // Wrap JS error in a Kotlin exception
            cont.resumeWithException(RuntimeException("Promise rejected: $error"))
            null
        }
    )
}*/
