package market.femi.kotlinapi

/// Local file storage boundary, mirroring `ProjectService.swift`. Every save and
/// read goes through here; nothing here touches the network. XMP metadata is
/// written/read through the Rust `psxmp_*` FFI (Adobe XMP Toolkit).
expect object ProjectService {
    /// Absolute path to the app's Documents directory. Consumers may set this
    /// once at app start (e.g. `Context.filesDir.path` on Android). Web keeps
    /// it as an empty string — OPFS is a per-origin root.
    var documents: String

    /// Embed XMP metadata into `data` (any format the toolkit recognizes) and
    /// write to `documents/<file>`. When all three of `prompt`/`model`/`subject`
    /// are null the input bytes are written through unchanged.
    suspend fun saveFile(data: ByteArray, named: String, prompt: String? = null, model: String? = null, subject: List<String>? = null)

    suspend fun like(file: String, liked: Boolean)
    suspend fun getAllGenerations(): List<String>
    suspend fun saveAudio(data: ByteArray, named: String)
    suspend fun getAudio(): String?
    suspend fun getPrompt(file: String): String?
    suspend fun getModel(file: String): String?
    suspend fun getSubject(file: String): List<String>?
    suspend fun getLike(file: String): Boolean
    fun getUrl(file: String): String

    fun setCharacterCast(a: String, b: String)
    fun getCharacterCast(): Pair<String, String>?
    fun clearCharacterCast()

    fun setImageEdit(file: String)
    fun getImageEdit(): String?
    fun clearImageEdit()
}
