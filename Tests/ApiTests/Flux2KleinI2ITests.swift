import Testing
import Foundation
@testable import Api

struct Flux2KleinI2ITests {
    @Test func fundedUserReturnsRealImage() async throws {
        let chairUrl = Bundle.module.url(forResource: "pink_tone_chair", withExtension: "png")!
        let carUrl = Bundle.module.url(forResource: "car_interior_white", withExtension: "jpeg")!
        let chair = try Data(contentsOf: chairUrl)
        let car = try Data(contentsOf: carUrl)
        let img = await Api.flux2KleinI2I(
            user: testUser,
            password: testPassword,
            image: chair,
            image2: car,
            prompt: "place the chair into the car interior"
        )
        #expect(!img.isEmpty)
        #expect(img != Api.fallbackImage)
        #expect(img != Api.topupImage)
    }

    @Test func cancellationReturnsFallback() async throws {
        let chairUrl = Bundle.module.url(forResource: "pink_tone_chair", withExtension: "png")!
        let carUrl = Bundle.module.url(forResource: "car_interior_white", withExtension: "jpeg")!
        let chair = try Data(contentsOf: chairUrl)
        let car = try Data(contentsOf: carUrl)
        let task = Task {
            await Api.flux2KleinI2I(
                user: testUser,
                password: testPassword,
                image: chair,
                image2: car,
                prompt: "place the chair into the car interior"
            )
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
