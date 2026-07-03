package market.femi.api

internal object ProjectServiceJvm {
    init { System.loadLibrary("rust_ffi") }

    @JvmStatic external fun psxmpInitDocuments(path: String)

    @JvmStatic external fun psxmpSaveFile(name: String, bytes: ByteArray, prompt: String?, model: String?, subject: Array<String>?)
    @JvmStatic external fun psxmpSaveAudio(name: String, bytes: ByteArray)
    @JvmStatic external fun psxmpLike(file: String, liked: Int)

    @JvmStatic external fun psxmpGetAllGenerations(): String?
    @JvmStatic external fun psxmpGetAudio(): String?
    @JvmStatic external fun psxmpGetPrompt(file: String): String?
    @JvmStatic external fun psxmpGetModel(file: String): String?
    @JvmStatic external fun psxmpGetSubject(file: String): String?
    @JvmStatic external fun psxmpGetLike(file: String): Int
    @JvmStatic external fun psxmpGetUrl(file: String): String

    @JvmStatic external fun psxmpSetCharacterCast(a: String, b: String)
    @JvmStatic external fun psxmpGetCharacterCast(): String?
    @JvmStatic external fun psxmpClearCharacterCast()
    @JvmStatic external fun psxmpSetImageEdit(file: String)
    @JvmStatic external fun psxmpGetImageEdit(): String?
    @JvmStatic external fun psxmpClearImageEdit()
}
