package market.femi.kotlinapi

internal object ProjectServiceJvm {
    init { System.loadLibrary("rust_ffi") }
    @JvmStatic external fun psxmpEmbed(path: String, prompt: String?, model: String?, subject: Array<String>?): Int
    @JvmStatic external fun psxmpReadPrompt(path: String): String?
    @JvmStatic external fun psxmpReadModel(path: String): String?
    @JvmStatic external fun psxmpReadSubjectCount(path: String): Int
    @JvmStatic external fun psxmpReadSubjectAt(path: String, index: Int): String?
    @JvmStatic external fun psxmpSetRating(path: String, rating: Int): Int
    @JvmStatic external fun psxmpReadRating(path: String): Int
}
