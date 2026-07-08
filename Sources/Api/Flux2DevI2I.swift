import Foundation
import RustFFI

extension Api {
    public static func flux2DevI2I(
        image: Data,
        prompt: String
    ) async -> Data {
        let flag = UnsafeMutablePointer<UInt8>.allocate(capacity: 1)
        flag.initialize(to: 0)
        defer { flag.deinitialize(count: 1); flag.deallocate() }
        let flagAddr = UInt(bitPattern: flag)
        let imageB64 = image.base64EncodedString()
        return await withTaskCancellationHandler {
            await Task.detached(priority: .userInitiated) {
                let p = UnsafePointer<UInt8>(bitPattern: flagAddr)
                var len = 0
                let ptr = imageB64.withCString { i in
                    prompt.withCString { pr in
                        rust_ffi_flux2_dev_i2i(i, pr, p, &len)!
                    }
                }
                return Data(bytesNoCopy: ptr, count: len, deallocator: .free)
            }.value
        } onCancel: {
            UnsafeMutablePointer<UInt8>(bitPattern: flagAddr)?.pointee = 1
        }
    }
}
