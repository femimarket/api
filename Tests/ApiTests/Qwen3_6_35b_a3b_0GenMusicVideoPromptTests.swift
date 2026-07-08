import Testing
import Foundation
@testable import Api

struct Qwen3_6_35b_a3b_0GenMusicVideoPromptTests {
    @Test func returnsPrompt() async throws {
        let result = await Api.qwen3_6_35b_a3b_0GenMusicVideoPrompt()
        print(result)
        #expect(result != "Could not respond")
        #expect(!result.isEmpty)
    }

    @Test func cancellationReturnsFallback() async throws {
        let task = Task {
            await Api.qwen3_6_35b_a3b_0GenMusicVideoPrompt()
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
