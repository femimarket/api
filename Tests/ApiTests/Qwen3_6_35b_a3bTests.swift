import Testing
import Foundation
@testable import Api

struct Qwen3_6_35b_a3bTests {
    @Test func fundedUserReturnsReply() async throws {
        let result = await Api.qwen3_6_35b_a3b(
            user: testUser,
            password: testPassword,
            messages: [(role: .user, content: "say hi in one word")]
        )
        #expect(result.count == 2)
        #expect(result.last?.role == .assistant)
        let reply = result.last?.content ?? ""
        #expect(reply != "Could not respond")
        #expect(!reply.isEmpty)
    }

    @Test func cancellationReturnsFallback() async throws {
        let task = Task {
            await Api.qwen3_6_35b_a3b(
                user: testUser,
                password: testPassword,
                messages: [(role: .user, content: "write a long story")]
            )
        }
        try await Task.sleep(nanoseconds: 100_000_000)
        task.cancel()
        let start = Date()
        let result = await task.value
        let elapsed = Date().timeIntervalSince(start)
        #expect(result.last?.content == "Could not respond")
        #expect(elapsed < 1.0, "cancel should resolve in <1s, took \(elapsed)s")
    }
}
