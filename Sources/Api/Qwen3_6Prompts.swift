import Foundation
import RustFFI

// The qwen3_6_35b_a3b prompt-generation family. All authless; each returns a
// single generated string (or "Could not respond" on any failure). They share
// the same cancel-flag + detached-task plumbing, so `runStringFFI` holds it once
// and each endpoint just supplies the actual FFI call.
extension Api {
    private static func runStringFFI(
        _ call: @escaping @Sendable (_ flag: UnsafePointer<UInt8>?, _ len: inout Int) -> UnsafeMutablePointer<UInt8>?
    ) async -> String {
        let flag = UnsafeMutablePointer<UInt8>.allocate(capacity: 1)
        flag.initialize(to: 0)
        defer { flag.deinitialize(count: 1); flag.deallocate() }
        let flagAddr = UInt(bitPattern: flag)
        return await withTaskCancellationHandler {
            await Task.detached(priority: .userInitiated) {
                let p = UnsafePointer<UInt8>(bitPattern: flagAddr)
                var len = 0
                guard let ptr = call(p, &len) else { return "" }
                let body = Data(bytesNoCopy: ptr, count: len, deallocator: .free)
                return String(data: body, encoding: .utf8) ?? ""
            }.value
        } onCancel: {
            UnsafeMutablePointer<UInt8>(bitPattern: flagAddr)?.pointee = 1
        }
    }

    public static func qwen3_6_35b_a3b_0GenMusicVideoPrompt() async -> String {
        await runStringFFI { flag, len in
            rust_ffi_qwen3_6_35b_a3b_0gen_music_video_prompt(flag, &len)
        }
    }

    public static func qwen3_6_35b_a3b_1GenNewAngleFromXmpImagePrompt(
        xmpPrompt: String
    ) async -> String {
        await runStringFFI { flag, len in
            xmpPrompt.withCString { x in
                rust_ffi_qwen3_6_35b_a3b_1gen_new_angle_from_xmp_image_prompt(x, flag, &len)
            }
        }
    }

    public static func qwen3_6_35b_a3b_1GenRandomIdeaFromXmpPrompt(
        xmpPrompt: String
    ) async -> String {
        await runStringFFI { flag, len in
            xmpPrompt.withCString { x in
                rust_ffi_qwen3_6_35b_a3b_1gen_random_idea_from_xmp_prompt(x, flag, &len)
            }
        }
    }

    public static func qwen3_6_35b_a3b_1Gen3VariantsFromXmpPromptAsJsonArray(
        xmpPrompt: String
    ) async -> String {
        await runStringFFI { flag, len in
            xmpPrompt.withCString { x in
                rust_ffi_qwen3_6_35b_a3b_1gen_3_variants_from_xmp_prompt_as_jsonarray(x, flag, &len)
            }
        }
    }

    public static func qwen3_6_35b_a3b_2GenAugmentIdeaFromXmpPromptAndText(
        xmpPrompt: String,
        text: String
    ) async -> String {
        await runStringFFI { flag, len in
            xmpPrompt.withCString { x in
                text.withCString { t in
                    rust_ffi_qwen3_6_35b_a3b_2gen_augment_idea_from_xmp_prompt_and_text(x, t, flag, &len)
                }
            }
        }
    }

    public static func qwen3_6_35b_a3b_2Gen50_100WordMultishotTimestampedPromptFrom2XmpImagePrompts(
        xmpPrompt: String,
        xmpPrompt2: String
    ) async -> String {
        await runStringFFI { flag, len in
            xmpPrompt.withCString { x in
                xmpPrompt2.withCString { x2 in
                    rust_ffi_qwen3_6_35b_a3b_2gen_50_100_word_multishot_timestamped_prompt_from_2_xmp_image_prompts(x, x2, flag, &len)
                }
            }
        }
    }

    public static func qwen3_6_35b_a3b_3Gen50_100WordMultishotTimestampedPromptFrom3XmpImagePrompts(
        xmpPrompt: String,
        xmpPrompt2: String,
        xmpPrompt3: String
    ) async -> String {
        await runStringFFI { flag, len in
            xmpPrompt.withCString { x in
                xmpPrompt2.withCString { x2 in
                    xmpPrompt3.withCString { x3 in
                        rust_ffi_qwen3_6_35b_a3b_3gen_50_100_word_multishot_timestamped_prompt_from_3_xmp_image_prompts(x, x2, x3, flag, &len)
                    }
                }
            }
        }
    }

    public static func qwen3_6_35b_a3b_4GenAugmentIdeaFrom3XmpPromptsAndText(
        xmpPrompt: String,
        xmpPrompt2: String,
        xmpPrompt3: String,
        text: String
    ) async -> String {
        await runStringFFI { flag, len in
            xmpPrompt.withCString { x in
                xmpPrompt2.withCString { x2 in
                    xmpPrompt3.withCString { x3 in
                        text.withCString { t in
                            rust_ffi_qwen3_6_35b_a3b_4gen_augment_idea_from_3_xmp_prompts_and_text(x, x2, x3, t, flag, &len)
                        }
                    }
                }
            }
        }
    }
}
