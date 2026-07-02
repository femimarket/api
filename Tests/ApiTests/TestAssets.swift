import Foundation

enum TestAssets {
    static let fallbackImage = try! Data(contentsOf: Bundle.module.url(forResource: "fallback", withExtension: "png")!)
    static let topupImage = try! Data(contentsOf: Bundle.module.url(forResource: "topup", withExtension: "jpg")!)
    static let fallbackVideo = try! Data(contentsOf: Bundle.module.url(forResource: "could-not-generate", withExtension: "mp4")!)
    static let topupVideo = try! Data(contentsOf: Bundle.module.url(forResource: "topup-video", withExtension: "mp4")!)
}
