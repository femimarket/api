import Foundation
import RustFFI

extension Api {
    /// Generates a short video from `image` + `audio` + `prompt` via the
    /// Ltx2_3A2V endpoint. `image` and/or `audio` may be empty `Data()` —
    /// the server treats them as unused. Returns the generated MP4 bytes on
    /// success, or the embedded topup/fallback MP4 the Rust FFI resolves for
    /// any failure.
    public static func ltx2_3a2v(
        user: String,
        password: String,
        image: Data,
        audio: Data,
        prompt: String
    ) async -> Data {
        let flag = UnsafeMutablePointer<UInt8>.allocate(capacity: 1)
        flag.initialize(to: 0)
        defer { flag.deinitialize(count: 1); flag.deallocate() }
        let flagAddr = UInt(bitPattern: flag)
        let imageB64 = image.base64EncodedString()
        let audioB64 = audio.base64EncodedString()
        return await withTaskCancellationHandler {
            await Task.detached(priority: .userInitiated) {
                let p = UnsafePointer<UInt8>(bitPattern: flagAddr)
                var len = 0
                let ptr = user.withCString { u in
                    password.withCString { pw in
                        imageB64.withCString { i in
                            audioB64.withCString { a in
                                prompt.withCString { pr in
                                    rust_ffi_ltx2_3a2v(u, pw, i, a, pr, p, &len)!
                                }
                            }
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
