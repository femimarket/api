package market.femi.api

/// Local file storage boundary. Rust owns the sandboxed `Documents/` root
/// (via `dirs::document_dir` on Apple, `context.filesDir` on Android via
/// androidx.startup, OPFS on wasm). Every call takes a filename.
expect object ProjectService {
    suspend fun saveFile(data: ByteArray, named: String, prompt: String? = null, model: String? = null, subject: List<String>? = null, projectName: String? = null, lyrics: String? = null, shotNumber: String? = null)
    suspend fun saveAudio(data: ByteArray, named: String)
    suspend fun like(file: String, liked: Boolean)
    suspend fun getAllGenerations(): List<String>
    suspend fun getAudio(): String?
    suspend fun getProjectName(file: String): String?
    suspend fun getLyrics(file: String): String?
    suspend fun getShotNumber(file: String): String?
    suspend fun getPrompt(file: String): String?
    suspend fun getModel(file: String): String?
    suspend fun getSubject(file: String): List<String>?
    suspend fun getLike(file: String): Boolean
    suspend fun getUrl(file: String): String

    suspend fun setCharacterCast(a: String, b: String)
    suspend fun getCharacterCast(): Pair<String, String>?
    suspend fun clearCharacterCast()

    suspend fun setImageEdit(file: String)
    suspend fun getImageEdit(): String?
    suspend fun clearImageEdit()
}
