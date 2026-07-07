import Foundation
import RustFFI

extension Api {
    /// Generates a short video from `image` + `audio` + `prompt` by calling
    /// ComfyUI Cloud directly with the caller's `comfyKey` (no femi.market
    /// server, no charging). `image` and/or `audio` may be empty `Data()`.
    /// Returns the generated MP4 bytes on success, or the embedded fallback
    /// MP4 the Rust FFI resolves for any failure.
    public static func ltx2_3a2v_comfyui(
        comfyKey: String,
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
                let ptr = comfyKey.withCString { k in
                    imageB64.withCString { i in
                        audioB64.withCString { a in
                            prompt.withCString { pr in
                                rust_ffi_ltx2_3a2v_comfyui(k, i, a, pr, p, &len)!
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
