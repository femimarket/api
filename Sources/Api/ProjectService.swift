//
//  ProjectService.swift
//  femi
//
//  Thin passthrough to the Rust `psxmp_*` FFI. Rust owns the Documents/
//  root, all file I/O, all XMP metadata handling, and the in-memory
//  character-cast / image-edit state. Swift just marshals arguments.
//

import Foundation
import RustFFI

public enum ProjectService {
    /// App sandbox `Documents/` URL.
    public static var documents: URL {
        URL(fileURLWithPath: readString(fill: { psxmp_get_url("", $0, $1) }) ?? "")
    }

    /// Embed XMP metadata into `data` and write to `Documents/<file>`.
    public static func saveFile(
        _ data: Data,
        named file: String,
        prompt: String? = nil,
        model: String? = nil,
        subject: [String]? = nil,
        projectName: String? = nil,
        lyrics: String? = nil,
        shotNumber: String? = nil
    ) {
        file.withCString { name in
            withOptionalCString(prompt) { pr in
                withOptionalCString(model) { md in
                    withOptionalCString(projectName) { ti in
                        withOptionalCString(lyrics) { ly in
                            withOptionalCString(shotNumber) { sn in
                                withCStringArray(subject ?? []) { arr, count in
                                    data.withUnsafeBytes { buf in
                                        let base = buf.baseAddress?.assumingMemoryBound(to: UInt8.self)
                                        psxmp_save_file(name, base, buf.count, pr, md, arr, count, ti, ly, sn)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    public static func getProjectName(_ file: String) -> String? {
        file.withCString { name in readString(fill: { psxmp_get_project_name(name, $0, $1) }) }
    }

    public static func getLyrics(_ file: String) -> String? {
        file.withCString { name in readString(fill: { psxmp_get_lyrics(name, $0, $1) }) }
    }

    public static func getShotNumber(_ file: String) -> String? {
        file.withCString { name in readString(fill: { psxmp_get_shot_number(name, $0, $1) }) }
    }

    /// Replace the audio file in `Documents/`.
    public static func saveAudio(_ data: Data, named file: String) {
        file.withCString { name in
            data.withUnsafeBytes { buf in
                let base = buf.baseAddress?.assumingMemoryBound(to: UInt8.self)
                psxmp_save_audio(name, base, buf.count)
            }
        }
    }

    public static func like(_ file: String, _ liked: Bool) {
        file.withCString { psxmp_like($0, liked ? 1 : 0) }
    }

    public static func getAllGenerations() -> [URL] {
        guard let json = readString(fill: { psxmp_get_all_generations($0, $1) }),
              let data = json.data(using: .utf8),
              let names = try? JSONDecoder().decode([String].self, from: data) else { return [] }
        return names.map { getUrl(for: $0) }
    }

    public static func getAudio() -> URL? {
        guard let name = readString(fill: { psxmp_get_audio($0, $1) }), !name.isEmpty else { return nil }
        return getUrl(for: name)
    }

    public static func getPrompt(_ file: String) -> String? {
        file.withCString { name in readString(fill: { psxmp_get_prompt(name, $0, $1) }) }
    }

    public static func getModel(_ file: String) -> String? {
        file.withCString { name in readString(fill: { psxmp_get_model(name, $0, $1) }) }
    }

    public static func getSubject(_ file: String) -> [String]? {
        guard let json = file.withCString({ name in readString(fill: { psxmp_get_subject(name, $0, $1) }) }),
              let data = json.data(using: .utf8),
              let items = try? JSONDecoder().decode([String].self, from: data),
              !items.isEmpty else { return nil }
        return items
    }

    public static func getLike(_ file: String) -> Bool {
        file.withCString { psxmp_get_like($0) == 1 }
    }

    public static func getUrl(for file: String) -> URL {
        let path = file.withCString { name in readString(fill: { psxmp_get_url(name, $0, $1) }) } ?? ""
        return URL(fileURLWithPath: path)
    }

    // MARK: - Operation arguments (in-memory, process-lifetime)

    public static func setCharacterCast(_ a: String, _ b: String) {
        a.withCString { pa in b.withCString { pb in psxmp_set_character_cast(pa, pb) } }
    }

    public static func getCharacterCast() -> (String, String)? {
        guard let json = readString(fill: { psxmp_get_character_cast($0, $1) }),
              let data = json.data(using: .utf8),
              let pair = try? JSONDecoder().decode([String].self, from: data),
              pair.count == 2 else { return nil }
        return (pair[0], pair[1])
    }

    public static func clearCharacterCast() {
        psxmp_clear_character_cast()
    }

    public static func setImageEdit(_ file: String) {
        file.withCString { psxmp_set_image_edit($0) }
    }

    public static func getImageEdit() -> String? {
        readString(fill: { psxmp_get_image_edit($0, $1) })
    }

    public static func clearImageEdit() {
        psxmp_clear_image_edit()
    }

    // MARK: - Internals

    private static func readString(
        fill: (UnsafeMutablePointer<CChar>?, Int32) -> Int32
    ) -> String? {
        // First call sizes the result (buffer = nil/0 → "how big?"), then we
        // allocate exactly that and fill it. Never truncates, however long.
        let needed = fill(nil, 0)
        guard needed > 0 else { return nil }
        var buf = [CChar](repeating: 0, count: Int(needed) + 1)
        let written = buf.withUnsafeMutableBufferPointer { bp in
            fill(bp.baseAddress, Int32(bp.count))
        }
        guard written > 0 else { return nil }
        let n = min(Int(written), Int(needed))
        let bytes = buf.prefix(n).map { UInt8(bitPattern: $0) }
        return String(decoding: bytes, as: UTF8.self)
    }

    private static func withOptionalCString<R>(
        _ s: String?, _ body: (UnsafePointer<CChar>?) -> R
    ) -> R {
        if let s { return s.withCString { body($0) } }
        return body(nil)
    }

    private static func withCStringArray<R>(
        _ strings: [String],
        _ body: (UnsafeMutablePointer<UnsafePointer<CChar>?>?, Int32) -> R
    ) -> R {
        if strings.isEmpty { return body(nil, 0) }
        let dupped = strings.map { strdup($0)! }
        defer { dupped.forEach { free($0) } }
        var pointers: [UnsafePointer<CChar>?] = dupped.map { UnsafePointer($0) }
        return pointers.withUnsafeMutableBufferPointer { buf in
            body(buf.baseAddress, Int32(strings.count))
        }
    }
}
