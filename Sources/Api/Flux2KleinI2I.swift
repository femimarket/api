import Foundation
import RustFFI

extension Api {
    public static func flux2KleinI2I(
        user: String,
        password: String,
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
                var status: UInt16 = 0
                var len = 0
                let ptr: UnsafeMutablePointer<UInt8>? = user.withCString { u in
                    password.withCString { pw in
                        imageB64.withCString { i1 in
                            image2B64.withCString { i2 in
                                prompt.withCString { pr in
                                    rust_ffi_flux2_klein_i2i(u, pw, i1, i2, pr, p, &status, &len)
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
                return status == 402 ? Api.topupImage : Api.fallbackImage
            }.value
        } onCancel: {
            UnsafeMutablePointer<UInt8>(bitPattern: flagAddr)?.pointee = 1
        }
    }
}
