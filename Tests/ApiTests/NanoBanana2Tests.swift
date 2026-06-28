import Testing
import Foundation
@testable import Api

struct NanoBanana2Tests {
    @Test func fundedUserReturnsRealImage() async throws {
        let img = await Api.nanoBanana2(
            user: testUser,
            password: testPassword,
            prompt: "a red apple on a wooden table"
        )
        #expect(!img.isEmpty)
    }
}
