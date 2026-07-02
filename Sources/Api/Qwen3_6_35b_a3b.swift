import Foundation
import RustFFI

extension Api {
    /// Role of a chat turn for qwen3_6_35b_a3b. Raw value matches the wire
    /// format the server expects (`"User"` / `"Assistant"`).
    public enum Role: String, Sendable {
        case user = "User"
        case assistant = "Assistant"
    }

    /// Each tuple is one chat turn — `role` is `.user` or `.assistant`,
    /// `content` is the message text. Returns `messages` with the new
    /// assistant turn appended: the model's reply on success, or
    /// `"Could not respond"` on any failure. Always safe to feed the result
    /// straight back as the next call's `messages`.
    public static func qwen3_6_35b_a3b(
        user: String,
        password: String,
        messages: [(role: Role, content: String)]
    ) async -> [(role: Role, content: String)] {
        let flag = UnsafeMutablePointer<UInt8>.allocate(capacity: 1)
        flag.initialize(to: 0)
        defer { flag.deinitialize(count: 1); flag.deallocate() }
        let flagAddr = UInt(bitPattern: flag)
        let wire = messages.map { ["role": $0.role.rawValue, "content": $0.content] }
        let messagesJson = String(data: try! JSONSerialization.data(withJSONObject: wire), encoding: .utf8)!
        let original = messages
        return await withTaskCancellationHandler {
            await Task.detached(priority: .userInitiated) {
                let p = UnsafePointer<UInt8>(bitPattern: flagAddr)
                var len = 0
                let ptr = user.withCString { u in
                    password.withCString { pw in
                        messagesJson.withCString { m in
                            rust_ffi_qwen3_6_35b_a3b(u, pw, m, p, &len)!
                        }
                    }
                }
                let body = Data(bytesNoCopy: ptr, count: len, deallocator: .free)
                if let arr = try? JSONSerialization.jsonObject(with: body) as? [[String: String]],
                   let last = arr.last,
                   let role = last["role"].flatMap(Role.init(rawValue:)),
                   let content = last["content"] {
                    return original + [(role: role, content: content)]
                }
                return original + [(role: .assistant, content: "Could not respond")]
            }.value
        } onCancel: {
            UnsafeMutablePointer<UInt8>(bitPattern: flagAddr)?.pointee = 1
        }
    }
}
