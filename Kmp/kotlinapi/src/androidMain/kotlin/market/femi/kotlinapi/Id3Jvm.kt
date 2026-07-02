package market.femi.kotlinapi

internal object Id3Jvm {
    init { System.loadLibrary("rust_ffi") }
    @JvmStatic external fun extractSylt(bytes: ByteArray): String
}
