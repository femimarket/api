package market.femi.api

internal object FemiApiJvm {
    init { System.loadLibrary("rust_ffi") }
    @JvmStatic external fun rustFfiZImageTurbo(prompt: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiNanoBanana2(prompt: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiFlux2Pro(prompt: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiFlux2DevI2i(img: String, prompt: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiFlux2KleinI2i(img1: String, img2: String, prompt: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiLtx23a2v(img: String, aud: String, prompt: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiLtx23a2vComfyui(comfyKey: String, img: String, aud: String, prompt: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiQwen3AsrFlash(aud: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiQwen3635bA3b(json: String, cancelFlag: Long): ByteArray

    // qwen3_6_35b_a3b prompt-gen family (authless; return generated string bytes).
    @JvmStatic external fun rustFfiQwen3635bA3b0GenMusicVideoPrompt(cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiQwen3635bA3b1GenNewAngleFromXmpImagePrompt(xmpPrompt: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiQwen3635bA3b1GenRandomIdeaFromXmpPrompt(xmpPrompt: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiQwen3635bA3b1Gen3VariantsFromXmpPromptAsJsonarray(xmpPrompt: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiQwen3635bA3b2GenAugmentIdeaFromXmpPromptAndText(xmpPrompt: String, text: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiQwen3635bA3b2Gen50100WordMultishotTimestampedPromptFrom2XmpImagePrompts(xmpPrompt: String, xmpPrompt2: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiQwen3635bA3b3Gen50100WordMultishotTimestampedPromptFrom3XmpImagePrompts(xmpPrompt: String, xmpPrompt2: String, xmpPrompt3: String, cancelFlag: Long): ByteArray
    @JvmStatic external fun rustFfiQwen3635bA3b4GenAugmentIdeaFrom3XmpPromptsAndText(xmpPrompt: String, xmpPrompt2: String, xmpPrompt3: String, text: String, cancelFlag: Long): ByteArray
}
