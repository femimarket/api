import Foundation
import RustFFI

extension Api {
    public static func qwen3AsrFlash(
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
                var len = 0
                let ptr = audioB64.withCString { a in
                    rust_ffi_qwen3_asr_flash(a, p, &len)!
                }
                let body = Data(bytesNoCopy: ptr, count: len, deallocator: .free)
                return String(data: body, encoding: .utf8)!
            }.value
        } onCancel: {
            UnsafeMutablePointer<UInt8>(bitPattern: flagAddr)?.pointee = 1
        }
    }
}
