package market.femi.api

import kotlinx.serialization.decodeFromString
import kotlinx.serialization.json.Json

/// Thin passthrough to the Rust JNI. Rust owns the Documents root.
/// Consumer app must call `ProjectService.initDocuments(context.filesDir.absolutePath)`
/// once at startup (e.g. via `androidx.startup` or a stub ContentProvider).
actual object ProjectService {
    fun initDocuments(path: String) = ProjectServiceJvm.psxmpInitDocuments(path)

    actual suspend fun saveFile(data: ByteArray, named: String, prompt: String?, model: String?, subject: List<String>?, projectName: String?, lyrics: String?, shotNumber: String?) =
        ProjectServiceJvm.psxmpSaveFile(named, data, prompt, model, subject?.toTypedArray(), projectName, lyrics, shotNumber)

    actual suspend fun saveAudio(data: ByteArray, named: String) =
        ProjectServiceJvm.psxmpSaveAudio(named, data)

    actual suspend fun like(file: String, liked: Boolean) =
        ProjectServiceJvm.psxmpLike(file, if (liked) 1 else 0)

    actual suspend fun getAllGenerations(): List<String> =
        ProjectServiceJvm.psxmpGetAllGenerations()?.let { Json.decodeFromString<List<String>>(it) } ?: emptyList()

    actual suspend fun getAudio(): String? = ProjectServiceJvm.psxmpGetAudio()

    actual suspend fun getProjectName(file: String): String? =
        ProjectServiceJvm.psxmpGetProjectName(file)?.takeIf { it.isNotEmpty() }

    actual suspend fun getLyrics(file: String): String? =
        ProjectServiceJvm.psxmpGetLyrics(file)?.takeIf { it.isNotEmpty() }

    actual suspend fun getShotNumber(file: String): String? =
        ProjectServiceJvm.psxmpGetShotNumber(file)?.takeIf { it.isNotEmpty() }

    actual suspend fun getPrompt(file: String): String? =
        ProjectServiceJvm.psxmpGetPrompt(file)?.takeIf { it.isNotEmpty() }

    actual suspend fun getModel(file: String): String? =
        ProjectServiceJvm.psxmpGetModel(file)?.takeIf { it.isNotEmpty() }

    actual suspend fun getSubject(file: String): List<String>? =
        ProjectServiceJvm.psxmpGetSubject(file)?.let { Json.decodeFromString<List<String>>(it) }?.takeIf { it.isNotEmpty() }

    actual suspend fun getLike(file: String): Boolean =
        ProjectServiceJvm.psxmpGetLike(file) == 1

    actual suspend fun getUrl(file: String): String = ProjectServiceJvm.psxmpGetUrl(file)

    actual suspend fun setCharacterCast(a: String, b: String) =
        ProjectServiceJvm.psxmpSetCharacterCast(a, b)

    actual suspend fun getCharacterCast(): Pair<String, String>? {
        val json = ProjectServiceJvm.psxmpGetCharacterCast() ?: return null
        val parts = Json.decodeFromString<List<String>>(json)
        return if (parts.size == 2) parts[0] to parts[1] else null
    }

    actual suspend fun clearCharacterCast() = ProjectServiceJvm.psxmpClearCharacterCast()

    actual suspend fun setImageEdit(file: String) = ProjectServiceJvm.psxmpSetImageEdit(file)
    actual suspend fun getImageEdit(): String? = ProjectServiceJvm.psxmpGetImageEdit()
    actual suspend fun clearImageEdit() = ProjectServiceJvm.psxmpClearImageEdit()
}
