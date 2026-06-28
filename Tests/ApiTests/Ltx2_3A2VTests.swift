import Testing
import Foundation
@testable import Api

struct Ltx2_3A2VTests {
    @Test func fundedUserReturnsRealVideo() async throws {
        let audioUrl = Bundle.module.url(forResource: "ltx_audio", withExtension: "mp3")!
        let imageUrl = Bundle.module.url(forResource: "man-walking", withExtension: "png")!
        let audio = try Data(contentsOf: audioUrl)
        let image = try Data(contentsOf: imageUrl)
        let video = await Api.ltx2_3a2v(
            user: testUser,
            password: testPassword,
            image: image,
            audio: audio,
            prompt: "the man walks forward in time with the music"
        )
        #expect(!video.isEmpty)
        #expect(video != Api.fallbackVideo)
        #expect(video != Api.topupVideo)
    }
}
