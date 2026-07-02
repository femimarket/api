package market.femi.api

import java.io.File

private val AUDIO_EXTS = setOf("mp3", "m4a", "wav", "aac", "caf", "aiff", "aif", "flac", "ogg", "opus")

actual object ProjectService {
    actual var documents: String = System.getProperty("java.io.tmpdir") ?: "/tmp"

    actual suspend fun saveFile(data: ByteArray, named: String, prompt: String?, model: String?, subject: List<String>?) {
        val bytes = if (prompt == null && model == null && subject == null) data
                    else embedXMP(data, named, prompt, model, subject)
        writeBytes(bytes, named)
    }

    actual suspend fun like(file: String, liked: Boolean) {
        val rating = if (liked) 5 else 0
        val result = ProjectServiceJvm.psxmpSetRating(getUrl(file), rating)
        check(result == 0) { "psxmp_set_rating failed for $file with code $result" }
    }

    actual suspend fun getAllGenerations(): List<String> =
        File(documents).listFiles()?.map { it.absolutePath } ?: emptyList()

    actual suspend fun saveAudio(data: ByteArray, named: String) {
        getAllGenerations().filter { isAudio(it) }.forEach { File(it).delete() }
        writeBytes(data, named)
    }

    actual suspend fun getAudio(): String? = getAllGenerations().firstOrNull { isAudio(it) }

    actual suspend fun getPrompt(file: String): String? =
        ProjectServiceJvm.psxmpReadPrompt(getUrl(file))?.takeIf { it.isNotEmpty() }

    actual suspend fun getModel(file: String): String? =
        ProjectServiceJvm.psxmpReadModel(getUrl(file))?.takeIf { it.isNotEmpty() }

    actual suspend fun getSubject(file: String): List<String>? {
        val path = getUrl(file)
        val count = ProjectServiceJvm.psxmpReadSubjectCount(path)
        if (count <= 0) return null
        val list = (0 until count).mapNotNull { ProjectServiceJvm.psxmpReadSubjectAt(path, it) }
        return list.ifEmpty { null }
    }

    actual suspend fun getLike(file: String): Boolean =
        ProjectServiceJvm.psxmpReadRating(getUrl(file)) in 1..5

    actual fun getUrl(file: String): String =
        File(documents, File(file).name).absolutePath

    // MARK: in-memory operation state (process lifetime)

    private var characterCast: Pair<String, String>? = null
    actual fun setCharacterCast(a: String, b: String) { characterCast = a to b }
    actual fun getCharacterCast(): Pair<String, String>? = characterCast
    actual fun clearCharacterCast() { characterCast = null }

    private var imageEdit: String? = null
    actual fun setImageEdit(file: String) { imageEdit = file }
    actual fun getImageEdit(): String? = imageEdit
    actual fun clearImageEdit() { imageEdit = null }

    // MARK: internals

    private fun embedXMP(data: ByteArray, file: String, prompt: String?, model: String?, subject: List<String>?): ByteArray {
        val ext = File(file).extension
        val tmp = File.createTempFile("psvc-", if (ext.isEmpty()) null else ".$ext")
        try {
            tmp.writeBytes(data)
            val result = ProjectServiceJvm.psxmpEmbed(tmp.absolutePath, prompt, model, subject?.toTypedArray())
            check(result == 0) { "psxmp_embed failed for $file with code $result" }
            return tmp.readBytes()
        } finally {
            tmp.delete()
        }
    }

    private fun writeBytes(data: ByteArray, named: String) {
        val dest = File(getUrl(named))
        dest.parentFile?.mkdirs()
        dest.writeBytes(data)
    }

    private fun isAudio(path: String): Boolean =
        AUDIO_EXTS.contains(File(path).extension.lowercase())
}
