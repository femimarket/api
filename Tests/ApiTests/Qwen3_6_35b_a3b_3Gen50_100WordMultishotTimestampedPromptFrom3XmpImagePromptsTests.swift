import Testing
import Foundation
@testable import Api

struct Qwen3_6_35b_a3b_3Gen50_100WordMultishotTimestampedPromptFrom3XmpImagePromptsTests {
    let xmpPrompt = "A lone astronaut on a red desert plateau at golden hour, wide cinematic shot, dust drifting"
    let xmpPrompt2 = "Neon-lit rain-slicked alley in a cyberpunk city, close-up on a figure under an umbrella"
    let xmpPrompt3 = "A wooden sailboat cutting through turquoise water at midday, aerial drone view"

    @Test func returnsPrompt() async throws {
        let result = await Api.qwen3_6_35b_a3b_3Gen50_100WordMultishotTimestampedPromptFrom3XmpImagePrompts(xmpPrompt: xmpPrompt, xmpPrompt2: xmpPrompt2, xmpPrompt3: xmpPrompt3)
        print(result)
        #expect(result != "Could not respond")
        #expect(!result.isEmpty)
    }

    @Test func cancellationReturnsFallback() async throws {
        let task = Task {
            await Api.qwen3_6_35b_a3b_3Gen50_100WordMultishotTimestampedPromptFrom3XmpImagePrompts(xmpPrompt: xmpPrompt, xmpPrompt2: xmpPrompt2, xmpPrompt3: xmpPrompt3)
        }
        try await Task.sleep(nanoseconds: 100_000_000)
        task.cancel()
        let start = Date()
        let result = await task.value
        let elapsed = Date().timeIntervalSince(start)
        #expect(result == "Could not respond")
        #expect(elapsed < 1.0, "cancel should resolve in <1s, took \(elapsed)s")
    }
}
