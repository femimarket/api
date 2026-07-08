import Foundation
import RustFFI

extension Api {
    public static func flux2KleinI2I(
        image: Data,
        image2: Data,
        prompt: String
    ) async -> Data {
        let flag = UnsafeMutablePointer<UInt8>.allocate(capacity: 1)
        flag.initialize(to: 0)
        defer { flag.deinitialize(count: 1); flag.deallocate() }
        let flagAddr = UInt(bitPattern: flag)
        let imageB64 = image.base64EncodedString()
        let image2B64 = image2.base64EncodedString()
        return await withTaskCancellationHandler {
            await Task.detached(priority: .userInitiated) {
                let p = UnsafePointer<UInt8>(bitPattern: flagAddr)
                var len = 0
                let ptr = imageB64.withCString { i1 in
                    image2B64.withCString { i2 in
                        prompt.withCString { pr in
                            rust_ffi_flux2_klein_i2i(i1, i2, pr, p, &len)!
                        }
                    }
                }
                return Data(bytesNoCopy: ptr, count: len, deallocator: .free)
            }.value
        } onCancel: {
            UnsafeMutablePointer<UInt8>(bitPattern: flagAddr)?.pointee = 1
        }
    }
}
