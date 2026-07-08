import Testing
import Foundation
@testable import Api

struct Qwen3AsrFlashTests {
    @Test func returnsLyrics() async throws {
        let url = Bundle.module.url(forResource: "test_audio", withExtension: "mp3")!
        let audio = try Data(contentsOf: url)
        let lyrics = await Api.qwen3AsrFlash(audio: audio)
        #expect(lyrics != "Could not process lyrics")
        #expect(lyrics != "Top up to transcribe lyrics")
        #expect(!lyrics.isEmpty)
    }

    @Test func cancellationReturnsFallback() async throws {
        let url = Bundle.module.url(forResource: "test_audio", withExtension: "mp3")!
        let audio = try Data(contentsOf: url)
        let task = Task {
            await Api.qwen3AsrFlash(audio: audio)
        }
        try await Task.sleep(nanoseconds: 100_000_000)
        task.cancel()
        let start = Date()
        let result = await task.value
        let elapsed = Date().timeIntervalSince(start)
        #expect(result == "Could not process lyrics")
        #expect(elapsed < 1.0, "cancel should resolve in <1s, took \(elapsed)s")
    }
}
