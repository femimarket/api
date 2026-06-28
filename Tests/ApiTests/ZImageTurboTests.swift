import Testing
import Foundation
@testable import Api

struct ZImageTurboTests {
    @Test func fundedUserReturnsRealImage() async throws {
        let img = await Api.zImageTurbo(
            user: testUser,
            password: testPassword,
            prompt: "a red apple on a wooden table"
        )
        #expect(!img.isEmpty)
    }

    @Test func emptyPromptReturnsRealImage() async throws {
        let img = await Api.zImageTurbo(user: testUser, password: testPassword, prompt: "")
        #expect(!img.isEmpty)
    }

    @Test func unicodePromptReturnsRealImage() async throws {
        let img = await Api.zImageTurbo(
            user: testUser,
            password: testPassword,
            prompt: "日本語の猫 🐈 \"quoted\" \\backslash"
        )
        #expect(!img.isEmpty)
    }
}
