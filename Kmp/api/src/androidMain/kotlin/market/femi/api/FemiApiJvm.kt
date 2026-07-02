package market.femi.api

internal object FemiApiJvm {
    init { System.loadLibrary("rust_ffi") }
    @JvmStatic external fun rustFfiZImageTurbo(user: String, pass: String, prompt: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiNanoBanana2(user: String, pass: String, prompt: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiFlux2Pro(user: String, pass: String, prompt: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiFlux2DevI2i(user: String, pass: String, img: String, prompt: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiFlux2KleinI2i(user: String, pass: String, img1: String, img2: String, prompt: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiLtx23a2v(user: String, pass: String, img: String, aud: String, prompt: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiQwen3AsrFlash(user: String, pass: String, aud: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiQwen3635bA3b(user: String, pass: String, json: String, cancelFlag: Long): ByteArray
}
