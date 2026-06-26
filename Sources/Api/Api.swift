import Foundation
import RustFFI

public enum Api {
    public static func zImageTurbo(
        token: String,
        prompt: String
    ) async -> (status: UInt16, body: Data) {
        let flag = UnsafeMutablePointer<UInt8>.allocate(capacity: 1)
        flag.initialize(to: 0)
        defer { flag.deinitialize(count: 1); flag.deallocate() }
        let flagAddr = UInt(bitPattern: flag)
        return await withTaskCancellationHandler {
            await Task.detached(priority: .userInitiated) {
                let p = UnsafePointer<UInt8>(bitPattern: flagAddr)
                var status: UInt16 = 0
                var len = 0
                let ptr: UnsafeMutablePointer<UInt8>? = token.withCString { t in
                    prompt.withCString { pr in
                        rust_ffi_z_image_turbo(t, pr, p, &status, &len)
                    }
                }
                guard let ptr, len > 0 else { return (status, Data()) }
                return (status, Data(bytesNoCopy: ptr, count: len, deallocator: .free))
            }.value
        } onCancel: {
            // Single-byte store; atomic at the hardware level on AArch64/x86_64
            // since the FFI boundary acts as a compiler barrier. Rust reads via
            // AtomicU8 with Relaxed ordering — sufficient for a cancellation flag.
            UnsafeMutablePointer<UInt8>(bitPattern: flagAddr)?.pointee = 1
        }
    }
}
