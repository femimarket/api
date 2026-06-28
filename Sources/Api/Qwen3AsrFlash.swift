import Foundation
import RustFFI

extension Api {
    public static func qwen3AsrFlash(
        user: String,
        password: String,
        audio: Data
    ) async -> String {
        let flag = UnsafeMutablePointer<UInt8>.allocate(capacity: 1)
        flag.initialize(to: 0)
        defer { flag.deinitialize(count: 1); flag.deallocate() }
        let flagAddr = UInt(bitPattern: flag)
        let audioB64 = audio.base64EncodedString()
        return await withTaskCancellationHandler {
            await Task.detached(priority: .userInitiated) {
                let p = UnsafePointer<UInt8>(bitPattern: flagAddr)
                var status: UInt16 = 0
                var len = 0
                let ptr: UnsafeMutablePointer<UInt8>? = user.withCString { u in
                    password.withCString { pw in
                        audioB64.withCString { a in
                            rust_ffi_qwen3_asr_flash(u, pw, a, p, &status, &len)
                        }
                    }
                }
                let body: Data = (ptr != nil && len > 0)
                    ? Data(bytesNoCopy: ptr!, count: len, deallocator: .free)
                    : Data()
                if status == 200,
                   let json = try? JSONSerialization.jsonObject(with: body) as? [String: Any],
                   let action = json["action"] as? [String: Any],
                   let lyrics = action["lyrics"] as? String,
                   !lyrics.isEmpty {
                    return lyrics
                }
                return status == 402 ? "Top up to transcribe lyrics" : "Could not process lyrics"
            }.value
        } onCancel: {
            UnsafeMutablePointer<UInt8>(bitPattern: flagAddr)?.pointee = 1
        }
    }
}
