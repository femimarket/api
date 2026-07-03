/* tslint:disable */
/* eslint-disable */

/**
 * Built-in XMP namespaces (enum for JavaScript)
 *
 * These enum values can be used in JavaScript to reference standard XMP namespaces.
 *
 * # Example
 *
 * ```javascript
 * import { Namespace, namespace_uri } from './pkg/xmpkit.js';
 * meta.set_property(namespace_uri(Namespace.Xmp), "CreatorTool", "MyApp");
 * ```
 */
export enum Namespace {
    /**
     * XMP Basic namespace
     */
    Xmp = 0,
    /**
     * Dublin Core namespace
     */
    Dc = 1,
    /**
     * EXIF namespace
     */
    Exif = 2,
    /**
     * EXIF Aux namespace
     */
    ExifAux = 3,
    /**
     * EXIF 2.32 Extension namespace
     */
    ExifEx = 4,
    /**
     * IPTC Core namespace
     */
    IptcCore = 5,
    /**
     * IPTC Extension namespace
     */
    IptcExt = 6,
    /**
     * Photoshop namespace
     */
    Photoshop = 7,
    /**
     * Camera Raw namespace
     */
    CameraRaw = 8,
    /**
     * XMP Rights namespace
     */
    XmpRights = 9,
    /**
     * XMP Media Management namespace
     */
    XmpMm = 10,
    /**
     * XMP Basic Job Ticket namespace
     */
    XmpBj = 11,
    /**
     * TIFF namespace
     */
    Tiff = 12,
    /**
     * PDF namespace
     */
    Pdf = 13,
    /**
     * PDF/X namespace
     */
    Pdfx = 14,
    /**
     * PDF/A namespace
     */
    Pdfa = 15,
    /**
     * XMP Dynamic Media namespace
     */
    XmpDm = 16,
    /**
     * XMP PagedText namespace
     */
    XmpPaged = 17,
    /**
     * XMP Graphics namespace
     */
    XmpGraphics = 18,
    /**
     * XMP Image namespace
     */
    XmpImage = 19,
    /**
     * RDF namespace
     */
    Rdf = 20,
    /**
     * XML namespace
     */
    Xml = 21,
}

/**
 * A qualifier for an XMP property
 *
 * Qualifiers provide additional information about XMP properties.
 * They can be used to add language information, type information, etc.
 */
export class Qualifier {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Create a new qualifier
     *
     * # Arguments
     * * `namespace` - Namespace URI (e.g., "http://ns.adobe.com/xap/1.0/")
     * * `name` - Qualifier name (e.g., "lang")
     * * `value` - Qualifier value (e.g., "en-US")
     */
    constructor(namespace: string, name: string, value: string);
    /**
     * Get the full path of the qualifier (namespace:name)
     */
    path(): string;
    /**
     * Get the name of the qualifier
     */
    readonly name: string;
    /**
     * Get the namespace URI of the qualifier
     */
    readonly namespace: string;
    /**
     * Get the value of the qualifier
     */
    readonly value: string;
}

/**
 * XMP Date/Time structure
 *
 * Represents a date/time value with optional components.
 * XMP supports partial dates (e.g., just year, or year-month).
 */
export class XmpDateTime {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Format as XMP date/time string
     */
    format(): string;
    /**
     * Create a new empty XMP date/time
     */
    constructor();
    /**
     * Parse an XMP date/time string
     *
     * XMP date/time format:
     * - `YYYY` - year only
     * - `YYYY-MM` - year and month
     * - `YYYY-MM-DD` - date only
     * - `YYYY-MM-DDThh:mm:ss` - date and time
     * - `YYYY-MM-DDThh:mm:ss.sss` - with fractional seconds
     * - `YYYY-MM-DDThh:mm:ssZ` - UTC timezone
     * - `YYYY-MM-DDThh:mm:ss+hh:mm` - timezone offset
     *
     * # Example
     *
     * ```javascript
     * import { XmpDateTime } from './pkg/xmpkit.js';
     * const dt = XmpDateTime.parse("2023-12-25T10:30:00Z");
     * console.log(dt.year); // 2023
     * ```
     */
    static parse(s: string): XmpDateTime;
    /**
     * Get the day (1-31, 0 means not set)
     */
    readonly day: number;
    /**
     * Whether date components are present
     */
    readonly has_date: boolean;
    /**
     * Whether time components are present
     */
    readonly has_time: boolean;
    /**
     * Whether timezone is present
     */
    readonly has_timezone: boolean;
    /**
     * Get the hour (0-23)
     */
    readonly hour: number;
    /**
     * Get the minute (0-59)
     */
    readonly minute: number;
    /**
     * Get the month (1-12, 0 means not set)
     */
    readonly month: number;
    /**
     * Get the nanoseconds (0-999999999)
     */
    readonly nanosecond: number;
    /**
     * Get the second (0-59)
     */
    readonly second: number;
    /**
     * Timezone hour offset (0-23)
     */
    readonly tz_hour: number;
    /**
     * Timezone minute offset (0-59)
     */
    readonly tz_minute: number;
    /**
     * Timezone sign: -1 (west), 0 (UTC), +1 (east)
     */
    readonly tz_sign: number;
    /**
     * Get the year
     */
    readonly year: number;
}

/**
 * WebAssembly error type for XMP operations
 *
 * This provides structured error information that JavaScript can inspect.
 *
 * # Example
 *
 * ```javascript
 * import { XmpErrorKind } from './pkg/xmpkit.js';
 * try {
 *     file.from_bytes(data);
 * } catch (error) {
 *     if (error instanceof XmpError) {
 *         if (error.kind === XmpErrorKind.BadParam) {
 *             console.log("Bad parameter error:", error.message);
 *         }
 *     }
 * }
 * ```
 */
export class XmpError {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Get the error kind enum value
     */
    readonly kind: XmpErrorKind;
    /**
     * Get the error message
     */
    readonly message: string;
}

/**
 * XMP Error kinds (exported enum for JavaScript)
 *
 * These enum values can be used in JavaScript to check error types:
 *
 * ```javascript
 * import { XmpErrorKind } from './pkg/xmpkit.js';
 * try {
 *     file.from_bytes(data);
 * } catch (error) {
 *     if (error.kind() === XmpErrorKind.BadParam) {
 *         console.log("Bad parameter error");
 *     }
 * }
 * ```
 */
export enum XmpErrorKind {
    /**
     * Bad parameter error
     */
    BadParam = 0,
    /**
     * Bad value error
     */
    BadValue = 1,
    /**
     * Bad schema error
     */
    BadSchema = 2,
    /**
     * Bad XPath error
     */
    BadXPath = 3,
    /**
     * Parse error
     */
    ParseError = 4,
    /**
     * Serialization error
     */
    SerializationError = 5,
    /**
     * IO error
     */
    IoError = 6,
    /**
     * Internal error
     */
    InternalError = 7,
    /**
     * Not found error
     */
    NotFound = 8,
    /**
     * Not supported error
     */
    NotSupported = 9,
}

/**
 * XmpFile for WebAssembly
 *
 * Provides the same API as Rust's `XmpFile`.
 *
 * # Example
 *
 * ```javascript
 * import init, { XmpFile, XmpOptions } from './pkg/xmpkit.js';
 * await init();
 *
 * // Read-only mode (memory efficient)
 * const file = new XmpFile();
 * file.from_bytes(fileData);
 * const meta = file.get_xmp();
 *
 * // Read and write mode
 * const file2 = new XmpFile();
 * const options = new XmpOptions();
 * options.for_update();  // Required for write_to_bytes()
 * file2.from_bytes_with(fileData, options);
 * const meta2 = file2.get_xmp();
 * if (meta2) {
 *     meta2.set_property("http://ns.adobe.com/xap/1.0/", "CreatorTool", "MyApp");
 *     file2.put_xmp(meta2);
 * }
 * const modifiedData = file2.write_to_bytes();
 * ```
 */
export class XmpFile {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Load XMP from file bytes
     */
    from_bytes(data: Uint8Array): void;
    /**
     * Load XMP from file bytes with options
     *
     * # Arguments
     * * `data` - File data as Uint8Array
     * * `options` - Opening options (e.g., use_packet_scanning, limited_scanning)
     *
     * # Example
     *
     * ```javascript
     * const options = new XmpOptions();
     * options.use_packet_scanning();
     * file.from_bytes_with(data, options);
     * ```
     */
    from_bytes_with(data: Uint8Array, options: XmpOptions): void;
    /**
     * Get XMP metadata (returns an XmpMeta instance)
     */
    get_xmp(): XmpMeta | undefined;
    /**
     * Create a new XmpFile instance
     */
    constructor();
    /**
     * Set XMP metadata
     */
    put_xmp(meta: XmpMeta): void;
    /**
     * Write file to bytes
     */
    write_to_bytes(): Uint8Array;
}

/**
 * XmpMeta for WebAssembly
 *
 * Provides the same API as Rust's `XmpMeta`.
 *
 * # Example
 *
 * ```javascript
 * import init, { XmpMeta } from './pkg/xmpkit.js';
 * await init();
 *
 * const meta = XmpMeta.parse(xmpPacketString);
 * const creatorTool = meta.get_property("http://ns.adobe.com/xap/1.0/", "CreatorTool");
 * meta.set_property("http://ns.adobe.com/xap/1.0/", "CreatorTool", "MyApp");
 * const serialized = meta.serialize();
 * ```
 */
export class XmpMeta {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Get the about URI
     */
    about_uri(): string | undefined;
    /**
     * Append an item to an array property
     */
    append_array_item(namespace: string, path: string, value: string): void;
    /**
     * Delete an item from an array property
     */
    delete_array_item(namespace: string, path: string, index: number): void;
    /**
     * Delete a property
     */
    delete_property(namespace: string, path: string): void;
    /**
     * Delete a struct field
     */
    delete_struct_field(namespace: string, struct_path: string, field: string): void;
    /**
     * Get an array item by index
     */
    get_array_item(namespace: string, path: string, index: number): string | undefined;
    /**
     * Get the size of an array property
     */
    get_array_size(namespace: string, path: string): number | undefined;
    /**
     * Get a property value
     *
     * Returns the property value as a string, or null if not found.
     * For complex types, returns a JSON string representation.
     *
     * # Arguments
     * * `namespace` - Namespace URI (e.g., "http://ns.adobe.com/xap/1.0/")
     * * `property` - Property name (e.g., "CreatorTool", "title")
     */
    get_property(namespace: string, property: string): string | undefined;
    /**
     * Get a struct field value
     */
    get_struct_field(namespace: string, struct_path: string, field: string): string | undefined;
    /**
     * Check if a property exists
     */
    has_property(namespace: string, path: string): boolean;
    /**
     * Insert an item into an array property at a specific index
     */
    insert_array_item(namespace: string, path: string, index: number, value: string): void;
    /**
     * Create a new empty XmpMeta instance
     */
    constructor();
    /**
     * Parse XMP packet string
     */
    static parse(xmp_packet: string): XmpMeta;
    /**
     * Serialize to RDF/XML string
     */
    serialize(): string;
    /**
     * Serialize to XMP packet string (with <?xpacket> wrapper)
     */
    serialize_packet(): string;
    /**
     * Set the about URI
     */
    set_about_uri(uri: string): void;
    /**
     * Set a property value
     *
     * # Arguments
     * * `namespace` - Namespace URI (e.g., "http://ns.adobe.com/xap/1.0/")
     * * `property` - Property name (e.g., "CreatorTool", "title")
     * * `value` - Property value as string
     */
    set_property(namespace: string, property: string, value: string): void;
    /**
     * Set a struct field value
     */
    set_struct_field(namespace: string, struct_path: string, field: string, value: string): void;
}

/**
 * Options for reading XMP metadata from files or memory (WebAssembly)
 *
 * Configure how XMP data is read and processed.
 *
 * # Example
 *
 * ```javascript
 * // For read-only operations (memory efficient)
 * const file = new XmpFile();
 * file.from_bytes(data);
 * const meta = file.get_xmp();
 *
 * // For read and write operations
 * const options = new XmpOptions();
 * options.for_update();  // Required if you want to write changes
 * file.from_bytes_with(data, options);
 * // ... modify metadata ...
 * const modifiedData = file.write_to_bytes();
 * ```
 */
export class XmpOptions {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Open for reading and writing
     *
     * This option is **required** if you want to use `write_to_bytes()` later.
     * When enabled, the original file data is stored in memory for later writing.
     *
     * If you only need to read XMP metadata, you can skip this option to save memory.
     */
    for_update(): void;
    /**
     * Only packet scan files "known" to need scanning
     */
    limited_scanning(): void;
    /**
     * Create default options
     */
    constructor();
    /**
     * Only the XMP is wanted (allows optimizations)
     */
    only_xmp(): void;
    /**
     * Be strict about only attempting to use the designated file handler
     */
    strict(): void;
    /**
     * Force packet scanning (do not use smart handler)
     */
    use_packet_scanning(): void;
    /**
     * Require the use of a smart handler
     */
    use_smart_handler(): void;
}

/**
 * XMP property value types
 *
 * Represents different types of values that can be stored in XMP properties.
 */
export class XmpValue {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Get the value as a boolean, if it is a boolean type
     */
    as_boolean(): boolean | undefined;
    /**
     * Get the value as a date/time string, if it is a date/time type
     */
    as_date_time(): string | undefined;
    /**
     * Get the value as an integer, if it is an integer type
     */
    as_integer(): bigint | undefined;
    /**
     * Get the value as a string, if it is a string type
     */
    as_string(): string | undefined;
    /**
     * Create a boolean value
     */
    static boolean(b: boolean): XmpValue;
    /**
     * Create a date/time value
     */
    static date_time(dt: string): XmpValue;
    /**
     * Create an integer value
     */
    static integer(i: bigint): XmpValue;
    /**
     * Create a string value
     */
    constructor(s: string);
    /**
     * Get the value kind
     */
    readonly kind: XmpValueKind;
}

/**
 * XMP value type kind
 */
export enum XmpValueKind {
    /**
     * String value
     */
    String = 0,
    /**
     * Integer value
     */
    Integer = 1,
    /**
     * Boolean value
     */
    Boolean = 2,
    /**
     * Date/time value
     */
    DateTime = 3,
}

/**
 * Read the first SYLT frame from MP3 `bytes` and return the timed lines as a
 * JSON array string ("[]" when the file has none). Name kept as
 * `extract_sylt` so the existing id3-wasm JS wrapper binds unchanged.
 */
export function extract_sylt(bytes: Uint8Array): string;

/**
 * Get all registered namespaces
 *
 * Returns a JavaScript object mapping URI to prefix.
 * # Example
 *
 * ```javascript
 * import { get_all_registered_namespaces } from './pkg/xmpkit.js';
 * const namespaces = get_all_registered_namespaces();
 * // namespaces is an object like { "http://ns.adobe.com/xap/1.0/": "xmp", ... }
 * ```
 */
export function get_all_registered_namespaces(): any;

/**
 * Get all built-in namespace URIs
 *
 * Returns a JavaScript array of built-in namespace URIs.
 * # Example
 *
 * ```javascript
 * import { get_builtin_namespace_uris } from './pkg/xmpkit.js';
 * const builtinUris = get_builtin_namespace_uris();
 * // builtinUris is an array like ["http://ns.adobe.com/xap/1.0/", ...]
 * ```
 */
export function get_builtin_namespace_uris(): string[];

/**
 * Get the prefix for a namespace URI
 */
export function get_namespace_prefix(uri: string): string | undefined;

/**
 * Get the URI for a namespace prefix
 */
export function get_namespace_uri(prefix: string): string | undefined;

/**
 * Check if a namespace URI is registered
 */
export function is_namespace_registered(uri: string): boolean;

/**
 * Get the namespace prefix for a Namespace enum value
 */
export function namespace_prefix(ns: Namespace): string;

/**
 * Get the namespace URI for a Namespace enum value
 *
 * # Example
 *
 * ```javascript
 * import { Namespace, namespace_uri } from './pkg/xmpkit.js';
 * meta.set_property(namespace_uri(Namespace.Xmp), "CreatorTool", "MyApp");
 * ```
 */
export function namespace_uri(ns: Namespace): string;

export function psxmp_clear_character_cast(): void;

export function psxmp_clear_image_edit(): void;

export function psxmp_get_all_generations(): Promise<string>;

export function psxmp_get_audio(): Promise<string | undefined>;

export function psxmp_get_character_cast(): string | undefined;

export function psxmp_get_image_edit(): string | undefined;

export function psxmp_get_like(file: string): Promise<boolean>;

export function psxmp_get_model(file: string): Promise<string | undefined>;

export function psxmp_get_prompt(file: string): Promise<string | undefined>;

export function psxmp_get_subject(file: string): Promise<string | undefined>;

export function psxmp_get_url(file: string): string;

export function psxmp_like(file: string, liked: boolean): Promise<void>;

export function psxmp_save_audio(name: string, bytes: Uint8Array): Promise<void>;

export function psxmp_save_file(name: string, bytes: Uint8Array, prompt: string | null | undefined, model: string | null | undefined, subjects: string[]): Promise<void>;

export function psxmp_set_character_cast(a: string, b: string): void;

export function psxmp_set_image_edit(file: string): void;

/**
 * Register a namespace URI with a prefix
 *
 * # Arguments
 * * `uri` - Namespace URI (e.g., "http://ns.adobe.com/xap/1.0/")
 * * `prefix` - Namespace prefix (e.g., "xmp")
 *
 * # Example
 *
 * ```javascript
 * import { register_namespace } from './pkg/xmpkit.js';
 * register_namespace("http://ns.adobe.com/xap/1.0/", "xmp");
 * ```
 */
export function register_namespace(uri: string, prefix: string): void;

export function wasm_flux2_dev_i2i(user: string, password: string, image_b64: string, prompt: string): Promise<Uint8Array>;

export function wasm_flux2_klein_i2i(user: string, password: string, image_b64: string, image2_b64: string, prompt: string): Promise<Uint8Array>;

export function wasm_flux2_pro(user: string, password: string, prompt: string): Promise<Uint8Array>;

export function wasm_ltx2_3a2v(user: string, password: string, image_b64: string, audio_b64: string, prompt: string): Promise<Uint8Array>;

export function wasm_nano_banana2(user: string, password: string, prompt: string): Promise<Uint8Array>;

export function wasm_qwen3_6_35b_a3b(user: string, password: string, messages_json: string): Promise<string>;

export function wasm_qwen3_asr_flash(user: string, password: string, audio_b64: string): Promise<string>;

export function wasm_z_image_turbo(user: string, password: string, prompt: string): Promise<Uint8Array>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly extract_sylt: (a: number, b: number, c: number) => void;
    readonly psxmp_clear_character_cast: () => void;
    readonly psxmp_clear_image_edit: () => void;
    readonly psxmp_get_all_generations: () => number;
    readonly psxmp_get_audio: () => number;
    readonly psxmp_get_character_cast: (a: number) => void;
    readonly psxmp_get_image_edit: (a: number) => void;
    readonly psxmp_get_like: (a: number, b: number) => number;
    readonly psxmp_get_model: (a: number, b: number) => number;
    readonly psxmp_get_prompt: (a: number, b: number) => number;
    readonly psxmp_get_subject: (a: number, b: number) => number;
    readonly psxmp_get_url: (a: number, b: number, c: number) => void;
    readonly psxmp_like: (a: number, b: number, c: number) => number;
    readonly psxmp_save_audio: (a: number, b: number, c: number, d: number) => number;
    readonly psxmp_save_file: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => number;
    readonly psxmp_set_character_cast: (a: number, b: number, c: number, d: number) => void;
    readonly psxmp_set_image_edit: (a: number, b: number) => void;
    readonly wasm_flux2_dev_i2i: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
    readonly wasm_flux2_klein_i2i: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => number;
    readonly wasm_flux2_pro: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
    readonly wasm_ltx2_3a2v: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => number;
    readonly wasm_nano_banana2: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
    readonly wasm_qwen3_6_35b_a3b: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
    readonly wasm_qwen3_asr_flash: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
    readonly wasm_z_image_turbo: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
    readonly __wbg_qualifier_free: (a: number, b: number) => void;
    readonly __wbg_xmpdatetime_free: (a: number, b: number) => void;
    readonly __wbg_xmperror_free: (a: number, b: number) => void;
    readonly __wbg_xmpfile_free: (a: number, b: number) => void;
    readonly __wbg_xmpmeta_free: (a: number, b: number) => void;
    readonly __wbg_xmpoptions_free: (a: number, b: number) => void;
    readonly __wbg_xmpvalue_free: (a: number, b: number) => void;
    readonly get_all_registered_namespaces: () => number;
    readonly get_builtin_namespace_uris: (a: number) => void;
    readonly get_namespace_prefix: (a: number, b: number, c: number) => void;
    readonly get_namespace_uri: (a: number, b: number, c: number) => void;
    readonly is_namespace_registered: (a: number, b: number) => number;
    readonly namespace_prefix: (a: number, b: number) => void;
    readonly namespace_uri: (a: number, b: number) => void;
    readonly qualifier_name: (a: number, b: number) => void;
    readonly qualifier_namespace: (a: number, b: number) => void;
    readonly qualifier_new: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
    readonly qualifier_path: (a: number, b: number) => void;
    readonly qualifier_value: (a: number, b: number) => void;
    readonly register_namespace: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly xmpdatetime_day: (a: number) => number;
    readonly xmpdatetime_format: (a: number, b: number) => void;
    readonly xmpdatetime_has_date: (a: number) => number;
    readonly xmpdatetime_has_time: (a: number) => number;
    readonly xmpdatetime_has_timezone: (a: number) => number;
    readonly xmpdatetime_hour: (a: number) => number;
    readonly xmpdatetime_minute: (a: number) => number;
    readonly xmpdatetime_month: (a: number) => number;
    readonly xmpdatetime_nanosecond: (a: number) => number;
    readonly xmpdatetime_new: () => number;
    readonly xmpdatetime_parse: (a: number, b: number, c: number) => void;
    readonly xmpdatetime_second: (a: number) => number;
    readonly xmpdatetime_tz_hour: (a: number) => number;
    readonly xmpdatetime_tz_minute: (a: number) => number;
    readonly xmpdatetime_tz_sign: (a: number) => number;
    readonly xmpdatetime_year: (a: number) => number;
    readonly xmperror_kind: (a: number) => number;
    readonly xmperror_message: (a: number, b: number) => void;
    readonly xmpfile_from_bytes: (a: number, b: number, c: number, d: number) => void;
    readonly xmpfile_from_bytes_with: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly xmpfile_get_xmp: (a: number) => number;
    readonly xmpfile_new: () => number;
    readonly xmpfile_put_xmp: (a: number, b: number) => void;
    readonly xmpfile_write_to_bytes: (a: number, b: number) => void;
    readonly xmpmeta_about_uri: (a: number, b: number) => void;
    readonly xmpmeta_append_array_item: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
    readonly xmpmeta_delete_array_item: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly xmpmeta_delete_property: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly xmpmeta_delete_struct_field: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
    readonly xmpmeta_get_array_item: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly xmpmeta_get_array_size: (a: number, b: number, c: number, d: number, e: number) => number;
    readonly xmpmeta_get_property: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly xmpmeta_get_struct_field: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
    readonly xmpmeta_has_property: (a: number, b: number, c: number, d: number, e: number) => number;
    readonly xmpmeta_insert_array_item: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
    readonly xmpmeta_new: () => number;
    readonly xmpmeta_parse: (a: number, b: number, c: number) => void;
    readonly xmpmeta_serialize: (a: number, b: number) => void;
    readonly xmpmeta_serialize_packet: (a: number, b: number) => void;
    readonly xmpmeta_set_about_uri: (a: number, b: number, c: number) => void;
    readonly xmpmeta_set_property: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
    readonly xmpmeta_set_struct_field: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => void;
    readonly xmpoptions_for_update: (a: number) => void;
    readonly xmpoptions_limited_scanning: (a: number) => void;
    readonly xmpoptions_new: () => number;
    readonly xmpoptions_only_xmp: (a: number) => void;
    readonly xmpoptions_strict: (a: number) => void;
    readonly xmpoptions_use_packet_scanning: (a: number) => void;
    readonly xmpoptions_use_smart_handler: (a: number) => void;
    readonly xmpvalue_as_boolean: (a: number) => number;
    readonly xmpvalue_as_date_time: (a: number, b: number) => void;
    readonly xmpvalue_as_integer: (a: number, b: number) => void;
    readonly xmpvalue_as_string: (a: number, b: number) => void;
    readonly xmpvalue_boolean: (a: number) => number;
    readonly xmpvalue_date_time: (a: number, b: number) => number;
    readonly xmpvalue_integer: (a: bigint) => number;
    readonly xmpvalue_kind: (a: number) => number;
    readonly xmpvalue_string: (a: number, b: number) => number;
    readonly __wasm_bindgen_func_elem_2858: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_2871: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_812: (a: number, b: number) => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number) => void;
    readonly __wbindgen_export4: (a: number, b: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_export5: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
