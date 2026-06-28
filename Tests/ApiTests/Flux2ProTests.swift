import Testing
import Foundation
@testable import Api

struct Flux2ProTests {
    @Test func fundedUserReturnsRealImage() async throws {
        let img = await Api.flux2Pro(
            user: testUser,
            password: testPassword,
            prompt: "a red apple on a wooden table"
        )
        #expect(!img.isEmpty)
    }
}
