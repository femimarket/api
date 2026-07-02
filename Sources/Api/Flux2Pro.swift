import Foundation
import RustFFI

extension Api {
    public static func flux2Pro(
        user: String,
        password: String,
        prompt: String
    ) async -> Data {
        let flag = UnsafeMutablePointer<UInt8>.allocate(capacity: 1)
        flag.initialize(to: 0)
        defer { flag.deinitialize(count: 1); flag.deallocate() }
        let flagAddr = UInt(bitPattern: flag)
        return await withTaskCancellationHandler {
            await Task.detached(priority: .userInitiated) {
                let p = UnsafePointer<UInt8>(bitPattern: flagAddr)
                var len = 0
                let ptr = user.withCString { u in
                    password.withCString { pw in
                        prompt.withCString { pr in
                            rust_ffi_flux2_pro(u, pw, pr, p, &len)!
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
