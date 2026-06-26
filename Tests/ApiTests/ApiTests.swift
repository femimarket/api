import Testing
import Foundation
@testable import Api

struct ZImageTurboTests {
    @Test func fundedUserReturns200() async throws {
        let (status, body) = await Api.zImageTurbo(
            token: "abc123",
            prompt: "a red apple on a wooden table"
        )
        #expect(status == 200)
        #expect(!body.isEmpty)
    }

    @Test func emptyPromptReturns200() async throws {
        let (status, body) = await Api.zImageTurbo(
            token: "abc123",
            prompt: ""
        )
        #expect(status == 200)
        #expect(!body.isEmpty)
    }

    @Test func unicodePromptReturns200() async throws {
        let (status, body) = await Api.zImageTurbo(
            token: "abc123",
            prompt: "日本語の猫 🐈 \"quoted\" \\backslash"
        )
        #expect(status == 200)
        #expect(!body.isEmpty)
    }

    @Test func unfundedUserReturns402WithMessage() async throws {
        let (status, body) = await Api.zImageTurbo(
            token: "unfunded-test-\(UUID().uuidString)",
            prompt: "hello"
        )
        #expect(status == 402)
        #expect(!body.isEmpty)
    }

    @Test func missingBearerReturns401WithMessage() async throws {
        let (status, body) = await Api.zImageTurbo(
            token: "",
            prompt: "hello"
        )
        #expect(status == 401)
        #expect(!body.isEmpty)
    }

    @Test func cancellationAbortsCall() async throws {
        let task = Task {
            await Api.zImageTurbo(
                token: "abc123",
                prompt: "a red apple on a wooden table"
            )
        }
        try await Task.sleep(nanoseconds: 100_000_000) // 100ms — request is in flight
        task.cancel()
        let start = Date()
        let (status, _) = await task.value
        let elapsed = Date().timeIntervalSince(start)
        #expect(status == 0, "expected 0 (cancelled) but got \(status)")
        #expect(elapsed < 1.0, "cancel should resolve in <1s, took \(elapsed)s")
    }
}
