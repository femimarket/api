import Foundation
import RustFFI

extension Api {
    /// Generates a short video from `image` + `audio` + `prompt` via the
    /// Ltx2_3A2V endpoint. `image` and/or `audio` may be empty `Data()` —
    /// the server treats them as unused. Returns the generated MP4 bytes on
    /// success, `Api.topupVideo` on 402, `Api.fallbackVideo` on any other failure.
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
                var status: UInt16 = 0
                var len = 0
                let ptr: UnsafeMutablePointer<UInt8>? = user.withCString { u in
                    password.withCString { pw in
                        imageB64.withCString { i in
                            audioB64.withCString { a in
                                prompt.withCString { pr in
                                    rust_ffi_ltx2_3a2v(u, pw, i, a, pr, p, &status, &len)
                                }
                            }
                        }
                    }
                }
                let body: Data = (ptr != nil && len > 0)
                    ? Data(bytesNoCopy: ptr!, count: len, deallocator: .free)
                    : Data()
                if status == 200,
                   let json = try? JSONSerialization.jsonObject(with: body) as? [String: Any],
                   let action = json["action"] as? [String: Any],
                   let file = action["file"] as? String,
                   let data = Data(base64Encoded: file),
                   !data.isEmpty {
                    return data
                }
                return status == 402 ? Api.topupVideo : Api.fallbackVideo
            }.value
        } onCancel: {
            UnsafeMutablePointer<UInt8>(bitPattern: flagAddr)?.pointee = 1
        }
    }
}
