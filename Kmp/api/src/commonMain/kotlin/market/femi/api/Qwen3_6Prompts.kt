package market.femi.api

// The qwen3_6_35b_a3b prompt-generation family. All authless; each returns a
// single generated string ("Could not respond" on any failure).

expect suspend fun qwen3_6_35b_a3b_0GenMusicVideoPrompt(): String

expect suspend fun qwen3_6_35b_a3b_1GenNewAngleFromXmpImagePrompt(xmpPrompt: String): String

expect suspend fun qwen3_6_35b_a3b_1GenRandomIdeaFromXmpPrompt(xmpPrompt: String): String

expect suspend fun qwen3_6_35b_a3b_1Gen3VariantsFromXmpPromptAsJsonArray(xmpPrompt: String): String

expect suspend fun qwen3_6_35b_a3b_2GenAugmentIdeaFromXmpPromptAndText(xmpPrompt: String, text: String): String

expect suspend fun qwen3_6_35b_a3b_2Gen50_100WordMultishotTimestampedPromptFrom2XmpImagePrompts(xmpPrompt: String, xmpPrompt2: String): String

expect suspend fun qwen3_6_35b_a3b_3Gen50_100WordMultishotTimestampedPromptFrom3XmpImagePrompts(xmpPrompt: String, xmpPrompt2: String, xmpPrompt3: String): String

expect suspend fun qwen3_6_35b_a3b_4GenAugmentIdeaFrom3XmpPromptsAndText(xmpPrompt: String, xmpPrompt2: String, xmpPrompt3: String, text: String): String
