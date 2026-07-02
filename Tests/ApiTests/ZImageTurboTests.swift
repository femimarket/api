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
        #expect(img != Api.fallbackImage)
        #expect(img != Api.topupImage)
    }

    @Test func cancellationReturnsFallback() async throws {
        let task = Task {
            await Api.zImageTurbo(user: testUser, password: testPassword, prompt: "a red apple on a wooden table")
        }
        try await Task.sleep(nanoseconds: 100_000_000)
        task.cancel()
        let start = Date()
        let result = await task.value
        let elapsed = Date().timeIntervalSince(start)
        #expect(result == Api.fallbackImage)
        #expect(elapsed < 1.0, "cancel should resolve in <1s, took \(elapsed)s")
    }
}
