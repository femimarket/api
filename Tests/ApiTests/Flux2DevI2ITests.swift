import Testing
import Foundation
@testable import Api

struct Flux2DevI2ITests {
    @Test func returnsRealImage() async throws {
        let url = Bundle.module.url(forResource: "cactus_man", withExtension: "png")!
        let image = try Data(contentsOf: url)
        let img = await Api.flux2DevI2I(
            image: image,
            prompt: "place him in a sunlit room"
        )
        #expect(!img.isEmpty)
        #expect(img != TestAssets.fallbackImage)
        #expect(img != TestAssets.topupImage)
    }

    @Test func cancellationReturnsFallback() async throws {
        let url = Bundle.module.url(forResource: "cactus_man", withExtension: "png")!
        let image = try Data(contentsOf: url)
        let task = Task {
            await Api.flux2DevI2I(image: image, prompt: "place him in a sunlit room")
        }
        try await Task.sleep(nanoseconds: 100_000_000)
        task.cancel()
        let start = Date()
        let result = await task.value
        let elapsed = Date().timeIntervalSince(start)
        #expect(result == TestAssets.fallbackImage)
        #expect(elapsed < 1.0, "cancel should resolve in <1s, took \(elapsed)s")
    }
}
