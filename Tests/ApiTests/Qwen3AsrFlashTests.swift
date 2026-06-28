import Testing
import Foundation
@testable import Api

struct Qwen3AsrFlashTests {
    @Test func fundedUserReturnsLyrics() async throws {
        let url = Bundle.module.url(forResource: "test_audio", withExtension: "mp3")!
        let audio = try Data(contentsOf: url)
        let lyrics = await Api.qwen3AsrFlash(user: testUser, password: testPassword, audio: audio)
        #expect(lyrics != "Could not process lyrics")
        #expect(lyrics != "Top up to transcribe lyrics")
        #expect(!lyrics.isEmpty)
    }

    @Test func emptyAudioReturnsFallback() async throws {
        let lyrics = await Api.qwen3AsrFlash(user: testUser, password: testPassword, audio: Data())
        #expect(lyrics == "Could not process lyrics")
    }
}
