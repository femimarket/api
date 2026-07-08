package market.femi.api

actual suspend fun qwen3_6_35b_a3b_0GenMusicVideoPrompt(): String =
    runCancelable { addr -> FemiApiJvm.rustFfiQwen3635bA3b0GenMusicVideoPrompt(addr) }.decodeToString()

actual suspend fun qwen3_6_35b_a3b_1GenNewAngleFromXmpImagePrompt(xmpPrompt: String): String =
    runCancelable { addr -> FemiApiJvm.rustFfiQwen3635bA3b1GenNewAngleFromXmpImagePrompt(xmpPrompt, addr) }.decodeToString()

actual suspend fun qwen3_6_35b_a3b_1GenRandomIdeaFromXmpPrompt(xmpPrompt: String): String =
    runCancelable { addr -> FemiApiJvm.rustFfiQwen3635bA3b1GenRandomIdeaFromXmpPrompt(xmpPrompt, addr) }.decodeToString()

actual suspend fun qwen3_6_35b_a3b_1Gen3VariantsFromXmpPromptAsJsonArray(xmpPrompt: String): String =
    runCancelable { addr -> FemiApiJvm.rustFfiQwen3635bA3b1Gen3VariantsFromXmpPromptAsJsonarray(xmpPrompt, addr) }.decodeToString()

actual suspend fun qwen3_6_35b_a3b_2GenAugmentIdeaFromXmpPromptAndText(xmpPrompt: String, text: String): String =
    runCancelable { addr -> FemiApiJvm.rustFfiQwen3635bA3b2GenAugmentIdeaFromXmpPromptAndText(xmpPrompt, text, addr) }.decodeToString()

actual suspend fun qwen3_6_35b_a3b_2Gen50_100WordMultishotTimestampedPromptFrom2XmpImagePrompts(xmpPrompt: String, xmpPrompt2: String): String =
    runCancelable { addr -> FemiApiJvm.rustFfiQwen3635bA3b2Gen50100WordMultishotTimestampedPromptFrom2XmpImagePrompts(xmpPrompt, xmpPrompt2, addr) }.decodeToString()

actual suspend fun qwen3_6_35b_a3b_3Gen50_100WordMultishotTimestampedPromptFrom3XmpImagePrompts(xmpPrompt: String, xmpPrompt2: String, xmpPrompt3: String): String =
    runCancelable { addr -> FemiApiJvm.rustFfiQwen3635bA3b3Gen50100WordMultishotTimestampedPromptFrom3XmpImagePrompts(xmpPrompt, xmpPrompt2, xmpPrompt3, addr) }.decodeToString()

actual suspend fun qwen3_6_35b_a3b_4GenAugmentIdeaFrom3XmpPromptsAndText(xmpPrompt: String, xmpPrompt2: String, xmpPrompt3: String, text: String): String =
    runCancelable { addr -> FemiApiJvm.rustFfiQwen3635bA3b4GenAugmentIdeaFrom3XmpPromptsAndText(xmpPrompt, xmpPrompt2, xmpPrompt3, text, addr) }.decodeToString()
