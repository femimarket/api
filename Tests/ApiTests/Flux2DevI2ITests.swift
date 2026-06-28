import Testing
import Foundation
@testable import Api

struct Flux2DevI2ITests {
    @Test func fundedUserReturnsRealImage() async throws {
        let url = Bundle.module.url(forResource: "cactus_man", withExtension: "png")!
        let image = try Data(contentsOf: url)
        let img = await Api.flux2DevI2I(
            user: testUser,
            password: testPassword,
            image: image,
            prompt: "place him in a sunlit room"
        )
        #expect(!img.isEmpty)
        #expect(img != Api.fallbackImage)
        #expect(img != Api.topupImage)
    }
}
