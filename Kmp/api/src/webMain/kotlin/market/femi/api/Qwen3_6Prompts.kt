@file:OptIn(kotlin.js.ExperimentalWasmJsInterop::class)

package market.femi.api

import kotlinx.coroutines.await
import kotlin.js.JsString

actual suspend fun qwen3_6_35b_a3b_0GenMusicVideoPrompt(): String {
    return rustFfi().wasm_qwen3_6_35b_a3b_0gen_music_video_prompt().await<JsString>().toString()
}

actual suspend fun qwen3_6_35b_a3b_1GenNewAngleFromXmpImagePrompt(xmpPrompt: String): String {
    return rustFfi().wasm_qwen3_6_35b_a3b_1gen_new_angle_from_xmp_image_prompt(xmpPrompt).await<JsString>().toString()
}

actual suspend fun qwen3_6_35b_a3b_1GenRandomIdeaFromXmpPrompt(xmpPrompt: String): String {
    return rustFfi().wasm_qwen3_6_35b_a3b_1gen_random_idea_from_xmp_prompt(xmpPrompt).await<JsString>().toString()
}

actual suspend fun qwen3_6_35b_a3b_1Gen3VariantsFromXmpPromptAsJsonArray(xmpPrompt: String): String {
    return rustFfi().wasm_qwen3_6_35b_a3b_1gen_3_variants_from_xmp_prompt_as_jsonarray(xmpPrompt).await<JsString>().toString()
}

actual suspend fun qwen3_6_35b_a3b_2GenAugmentIdeaFromXmpPromptAndText(xmpPrompt: String, text: String): String {
    return rustFfi().wasm_qwen3_6_35b_a3b_2gen_augment_idea_from_xmp_prompt_and_text(xmpPrompt, text).await<JsString>().toString()
}

actual suspend fun qwen3_6_35b_a3b_2Gen50_100WordMultishotTimestampedPromptFrom2XmpImagePrompts(xmpPrompt: String, xmpPrompt2: String): String {
    return rustFfi().wasm_qwen3_6_35b_a3b_2gen_50_100_word_multishot_timestamped_prompt_from_2_xmp_image_prompts(xmpPrompt, xmpPrompt2).await<JsString>().toString()
}

actual suspend fun qwen3_6_35b_a3b_3Gen50_100WordMultishotTimestampedPromptFrom3XmpImagePrompts(xmpPrompt: String, xmpPrompt2: String, xmpPrompt3: String): String {
    return rustFfi().wasm_qwen3_6_35b_a3b_3gen_50_100_word_multishot_timestamped_prompt_from_3_xmp_image_prompts(xmpPrompt, xmpPrompt2, xmpPrompt3).await<JsString>().toString()
}

actual suspend fun qwen3_6_35b_a3b_4GenAugmentIdeaFrom3XmpPromptsAndText(xmpPrompt: String, xmpPrompt2: String, xmpPrompt3: String, text: String): String {
    return rustFfi().wasm_qwen3_6_35b_a3b_4gen_augment_idea_from_3_xmp_prompts_and_text(xmpPrompt, xmpPrompt2, xmpPrompt3, text).await<JsString>().toString()
}
