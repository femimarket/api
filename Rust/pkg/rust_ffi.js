/* @ts-self-types="./rust_ffi.d.ts" */

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
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21}
 */
export const Namespace = Object.freeze({
    /**
     * XMP Basic namespace
     */
    Xmp: 0, "0": "Xmp",
    /**
     * Dublin Core namespace
     */
    Dc: 1, "1": "Dc",
    /**
     * EXIF namespace
     */
    Exif: 2, "2": "Exif",
    /**
     * EXIF Aux namespace
     */
    ExifAux: 3, "3": "ExifAux",
    /**
     * EXIF 2.32 Extension namespace
     */
    ExifEx: 4, "4": "ExifEx",
    /**
     * IPTC Core namespace
     */
    IptcCore: 5, "5": "IptcCore",
    /**
     * IPTC Extension namespace
     */
    IptcExt: 6, "6": "IptcExt",
    /**
     * Photoshop namespace
     */
    Photoshop: 7, "7": "Photoshop",
    /**
     * Camera Raw namespace
     */
    CameraRaw: 8, "8": "CameraRaw",
    /**
     * XMP Rights namespace
     */
    XmpRights: 9, "9": "XmpRights",
    /**
     * XMP Media Management namespace
     */
    XmpMm: 10, "10": "XmpMm",
    /**
     * XMP Basic Job Ticket namespace
     */
    XmpBj: 11, "11": "XmpBj",
    /**
     * TIFF namespace
     */
    Tiff: 12, "12": "Tiff",
    /**
     * PDF namespace
     */
    Pdf: 13, "13": "Pdf",
    /**
     * PDF/X namespace
     */
    Pdfx: 14, "14": "Pdfx",
    /**
     * PDF/A namespace
     */
    Pdfa: 15, "15": "Pdfa",
    /**
     * XMP Dynamic Media namespace
     */
    XmpDm: 16, "16": "XmpDm",
    /**
     * XMP PagedText namespace
     */
    XmpPaged: 17, "17": "XmpPaged",
    /**
     * XMP Graphics namespace
     */
    XmpGraphics: 18, "18": "XmpGraphics",
    /**
     * XMP Image namespace
     */
    XmpImage: 19, "19": "XmpImage",
    /**
     * RDF namespace
     */
    Rdf: 20, "20": "Rdf",
    /**
     * XML namespace
     */
    Xml: 21, "21": "Xml",
});

/**
 * A qualifier for an XMP property
 *
 * Qualifiers provide additional information about XMP properties.
 * They can be used to add language information, type information, etc.
 */
export class Qualifier {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        QualifierFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_qualifier_free(ptr, 0);
    }
    /**
     * Get the name of the qualifier
     * @returns {string}
     */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.qualifier_name(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export5(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get the namespace URI of the qualifier
     * @returns {string}
     */
    get namespace() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.qualifier_namespace(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export5(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Create a new qualifier
     *
     * # Arguments
     * * `namespace` - Namespace URI (e.g., "http://ns.adobe.com/xap/1.0/")
     * * `name` - Qualifier name (e.g., "lang")
     * * `value` - Qualifier value (e.g., "en-US")
     * @param {string} namespace
     * @param {string} name
     * @param {string} value
     */
    constructor(namespace, name, value) {
        const ptr0 = passStringToWasm0(namespace, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(name, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(value, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len2 = WASM_VECTOR_LEN;
        const ret = wasm.qualifier_new(ptr0, len0, ptr1, len1, ptr2, len2);
        this.__wbg_ptr = ret;
        QualifierFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Get the full path of the qualifier (namespace:name)
     * @returns {string}
     */
    path() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.qualifier_path(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export5(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get the value of the qualifier
     * @returns {string}
     */
    get value() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.qualifier_value(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export5(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) Qualifier.prototype[Symbol.dispose] = Qualifier.prototype.free;

/**
 * XMP Date/Time structure
 *
 * Represents a date/time value with optional components.
 * XMP supports partial dates (e.g., just year, or year-month).
 */
export class XmpDateTime {
    static __wrap(ptr) {
        const obj = Object.create(XmpDateTime.prototype);
        obj.__wbg_ptr = ptr;
        XmpDateTimeFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        XmpDateTimeFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_xmpdatetime_free(ptr, 0);
    }
    /**
     * Get the day (1-31, 0 means not set)
     * @returns {number}
     */
    get day() {
        const ret = wasm.xmpdatetime_day(this.__wbg_ptr);
        return ret;
    }
    /**
     * Format as XMP date/time string
     * @returns {string}
     */
    format() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.xmpdatetime_format(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export5(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Whether date components are present
     * @returns {boolean}
     */
    get has_date() {
        const ret = wasm.xmpdatetime_has_date(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Whether time components are present
     * @returns {boolean}
     */
    get has_time() {
        const ret = wasm.xmpdatetime_has_time(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Whether timezone is present
     * @returns {boolean}
     */
    get has_timezone() {
        const ret = wasm.xmpdatetime_has_timezone(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Get the hour (0-23)
     * @returns {number}
     */
    get hour() {
        const ret = wasm.xmpdatetime_hour(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the minute (0-59)
     * @returns {number}
     */
    get minute() {
        const ret = wasm.xmpdatetime_minute(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the month (1-12, 0 means not set)
     * @returns {number}
     */
    get month() {
        const ret = wasm.xmpdatetime_month(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the nanoseconds (0-999999999)
     * @returns {number}
     */
    get nanosecond() {
        const ret = wasm.xmpdatetime_nanosecond(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Create a new empty XMP date/time
     */
    constructor() {
        const ret = wasm.xmpdatetime_new();
        this.__wbg_ptr = ret;
        XmpDateTimeFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
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
     * @param {string} s
     * @returns {XmpDateTime}
     */
    static parse(s) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(s, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len0 = WASM_VECTOR_LEN;
            wasm.xmpdatetime_parse(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return XmpDateTime.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get the second (0-59)
     * @returns {number}
     */
    get second() {
        const ret = wasm.xmpdatetime_second(this.__wbg_ptr);
        return ret;
    }
    /**
     * Timezone hour offset (0-23)
     * @returns {number}
     */
    get tz_hour() {
        const ret = wasm.xmpdatetime_tz_hour(this.__wbg_ptr);
        return ret;
    }
    /**
     * Timezone minute offset (0-59)
     * @returns {number}
     */
    get tz_minute() {
        const ret = wasm.xmpdatetime_tz_minute(this.__wbg_ptr);
        return ret;
    }
    /**
     * Timezone sign: -1 (west), 0 (UTC), +1 (east)
     * @returns {number}
     */
    get tz_sign() {
        const ret = wasm.xmpdatetime_tz_sign(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the year
     * @returns {number}
     */
    get year() {
        const ret = wasm.xmpdatetime_year(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) XmpDateTime.prototype[Symbol.dispose] = XmpDateTime.prototype.free;

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
    static __wrap(ptr) {
        const obj = Object.create(XmpError.prototype);
        obj.__wbg_ptr = ptr;
        XmpErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        XmpErrorFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_xmperror_free(ptr, 0);
    }
    /**
     * Get the error kind enum value
     * @returns {XmpErrorKind}
     */
    get kind() {
        const ret = wasm.xmperror_kind(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the error message
     * @returns {string}
     */
    get message() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.xmperror_message(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export5(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) XmpError.prototype[Symbol.dispose] = XmpError.prototype.free;

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
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}
 */
export const XmpErrorKind = Object.freeze({
    /**
     * Bad parameter error
     */
    BadParam: 0, "0": "BadParam",
    /**
     * Bad value error
     */
    BadValue: 1, "1": "BadValue",
    /**
     * Bad schema error
     */
    BadSchema: 2, "2": "BadSchema",
    /**
     * Bad XPath error
     */
    BadXPath: 3, "3": "BadXPath",
    /**
     * Parse error
     */
    ParseError: 4, "4": "ParseError",
    /**
     * Serialization error
     */
    SerializationError: 5, "5": "SerializationError",
    /**
     * IO error
     */
    IoError: 6, "6": "IoError",
    /**
     * Internal error
     */
    InternalError: 7, "7": "InternalError",
    /**
     * Not found error
     */
    NotFound: 8, "8": "NotFound",
    /**
     * Not supported error
     */
    NotSupported: 9, "9": "NotSupported",
});

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
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        XmpFileFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_xmpfile_free(ptr, 0);
    }
    /**
     * Load XMP from file bytes
     * @param {Uint8Array} data
     */
    from_bytes(data) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.xmpfile_from_bytes(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
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
     * @param {Uint8Array} data
     * @param {XmpOptions} options
     */
    from_bytes_with(data, options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(options, XmpOptions);
            wasm.xmpfile_from_bytes_with(retptr, this.__wbg_ptr, ptr0, len0, options.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get XMP metadata (returns an XmpMeta instance)
     * @returns {XmpMeta | undefined}
     */
    get_xmp() {
        const ret = wasm.xmpfile_get_xmp(this.__wbg_ptr);
        return ret === 0 ? undefined : XmpMeta.__wrap(ret);
    }
    /**
     * Create a new XmpFile instance
     */
    constructor() {
        const ret = wasm.xmpfile_new();
        this.__wbg_ptr = ret;
        XmpFileFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Set XMP metadata
     * @param {XmpMeta} meta
     */
    put_xmp(meta) {
        _assertClass(meta, XmpMeta);
        var ptr0 = meta.__destroy_into_raw();
        wasm.xmpfile_put_xmp(this.__wbg_ptr, ptr0);
    }
    /**
     * Write file to bytes
     * @returns {Uint8Array}
     */
    write_to_bytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.xmpfile_write_to_bytes(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export5(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
if (Symbol.dispose) XmpFile.prototype[Symbol.dispose] = XmpFile.prototype.free;

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
    static __wrap(ptr) {
        const obj = Object.create(XmpMeta.prototype);
        obj.__wbg_ptr = ptr;
        XmpMetaFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        XmpMetaFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_xmpmeta_free(ptr, 0);
    }
    /**
     * Get the about URI
     * @returns {string | undefined}
     */
    about_uri() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.xmpmeta_about_uri(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_export5(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Append an item to an array property
     * @param {string} namespace
     * @param {string} path
     * @param {string} value
     */
    append_array_item(namespace, path, value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(namespace, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passStringToWasm0(value, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len2 = WASM_VECTOR_LEN;
            wasm.xmpmeta_append_array_item(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Delete an item from an array property
     * @param {string} namespace
     * @param {string} path
     * @param {number} index
     */
    delete_array_item(namespace, path, index) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(namespace, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            wasm.xmpmeta_delete_array_item(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1, index);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Delete a property
     * @param {string} namespace
     * @param {string} path
     */
    delete_property(namespace, path) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(namespace, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            wasm.xmpmeta_delete_property(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Delete a struct field
     * @param {string} namespace
     * @param {string} struct_path
     * @param {string} field
     */
    delete_struct_field(namespace, struct_path, field) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(namespace, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(struct_path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passStringToWasm0(field, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len2 = WASM_VECTOR_LEN;
            wasm.xmpmeta_delete_struct_field(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get an array item by index
     * @param {string} namespace
     * @param {string} path
     * @param {number} index
     * @returns {string | undefined}
     */
    get_array_item(namespace, path, index) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(namespace, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            wasm.xmpmeta_get_array_item(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1, index);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v3;
            if (r0 !== 0) {
                v3 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_export5(r0, r1 * 1, 1);
            }
            return v3;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get the size of an array property
     * @param {string} namespace
     * @param {string} path
     * @returns {number | undefined}
     */
    get_array_size(namespace, path) {
        const ptr0 = passStringToWasm0(namespace, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.xmpmeta_get_array_size(this.__wbg_ptr, ptr0, len0, ptr1, len1);
        return ret === Number.MAX_SAFE_INTEGER ? undefined : ret;
    }
    /**
     * Get a property value
     *
     * Returns the property value as a string, or null if not found.
     * For complex types, returns a JSON string representation.
     *
     * # Arguments
     * * `namespace` - Namespace URI (e.g., "http://ns.adobe.com/xap/1.0/")
     * * `property` - Property name (e.g., "CreatorTool", "title")
     * @param {string} namespace
     * @param {string} property
     * @returns {string | undefined}
     */
    get_property(namespace, property) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(namespace, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(property, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            wasm.xmpmeta_get_property(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v3;
            if (r0 !== 0) {
                v3 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_export5(r0, r1 * 1, 1);
            }
            return v3;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get a struct field value
     * @param {string} namespace
     * @param {string} struct_path
     * @param {string} field
     * @returns {string | undefined}
     */
    get_struct_field(namespace, struct_path, field) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(namespace, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(struct_path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passStringToWasm0(field, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len2 = WASM_VECTOR_LEN;
            wasm.xmpmeta_get_struct_field(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v4;
            if (r0 !== 0) {
                v4 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_export5(r0, r1 * 1, 1);
            }
            return v4;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Check if a property exists
     * @param {string} namespace
     * @param {string} path
     * @returns {boolean}
     */
    has_property(namespace, path) {
        const ptr0 = passStringToWasm0(namespace, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.xmpmeta_has_property(this.__wbg_ptr, ptr0, len0, ptr1, len1);
        return ret !== 0;
    }
    /**
     * Insert an item into an array property at a specific index
     * @param {string} namespace
     * @param {string} path
     * @param {number} index
     * @param {string} value
     */
    insert_array_item(namespace, path, index, value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(namespace, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passStringToWasm0(value, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len2 = WASM_VECTOR_LEN;
            wasm.xmpmeta_insert_array_item(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1, index, ptr2, len2);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Create a new empty XmpMeta instance
     */
    constructor() {
        const ret = wasm.xmpmeta_new();
        this.__wbg_ptr = ret;
        XmpMetaFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Parse XMP packet string
     * @param {string} xmp_packet
     * @returns {XmpMeta}
     */
    static parse(xmp_packet) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(xmp_packet, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len0 = WASM_VECTOR_LEN;
            wasm.xmpmeta_parse(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return XmpMeta.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Serialize to RDF/XML string
     * @returns {string}
     */
    serialize() {
        let deferred2_0;
        let deferred2_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.xmpmeta_serialize(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            var ptr1 = r0;
            var len1 = r1;
            if (r3) {
                ptr1 = 0; len1 = 0;
                throw takeObject(r2);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export5(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * Serialize to XMP packet string (with <?xpacket> wrapper)
     * @returns {string}
     */
    serialize_packet() {
        let deferred2_0;
        let deferred2_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.xmpmeta_serialize_packet(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            var ptr1 = r0;
            var len1 = r1;
            if (r3) {
                ptr1 = 0; len1 = 0;
                throw takeObject(r2);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export5(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * Set the about URI
     * @param {string} uri
     */
    set_about_uri(uri) {
        const ptr0 = passStringToWasm0(uri, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.xmpmeta_set_about_uri(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * Set a property value
     *
     * # Arguments
     * * `namespace` - Namespace URI (e.g., "http://ns.adobe.com/xap/1.0/")
     * * `property` - Property name (e.g., "CreatorTool", "title")
     * * `value` - Property value as string
     * @param {string} namespace
     * @param {string} property
     * @param {string} value
     */
    set_property(namespace, property, value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(namespace, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(property, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passStringToWasm0(value, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len2 = WASM_VECTOR_LEN;
            wasm.xmpmeta_set_property(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Set a struct field value
     * @param {string} namespace
     * @param {string} struct_path
     * @param {string} field
     * @param {string} value
     */
    set_struct_field(namespace, struct_path, field, value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(namespace, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(struct_path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passStringToWasm0(field, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len2 = WASM_VECTOR_LEN;
            const ptr3 = passStringToWasm0(value, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len3 = WASM_VECTOR_LEN;
            wasm.xmpmeta_set_struct_field(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
if (Symbol.dispose) XmpMeta.prototype[Symbol.dispose] = XmpMeta.prototype.free;

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
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        XmpOptionsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_xmpoptions_free(ptr, 0);
    }
    /**
     * Open for reading and writing
     *
     * This option is **required** if you want to use `write_to_bytes()` later.
     * When enabled, the original file data is stored in memory for later writing.
     *
     * If you only need to read XMP metadata, you can skip this option to save memory.
     */
    for_update() {
        wasm.xmpoptions_for_update(this.__wbg_ptr);
    }
    /**
     * Only packet scan files "known" to need scanning
     */
    limited_scanning() {
        wasm.xmpoptions_limited_scanning(this.__wbg_ptr);
    }
    /**
     * Create default options
     */
    constructor() {
        const ret = wasm.xmpoptions_new();
        this.__wbg_ptr = ret;
        XmpOptionsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Only the XMP is wanted (allows optimizations)
     */
    only_xmp() {
        wasm.xmpoptions_only_xmp(this.__wbg_ptr);
    }
    /**
     * Be strict about only attempting to use the designated file handler
     */
    strict() {
        wasm.xmpoptions_strict(this.__wbg_ptr);
    }
    /**
     * Force packet scanning (do not use smart handler)
     */
    use_packet_scanning() {
        wasm.xmpoptions_use_packet_scanning(this.__wbg_ptr);
    }
    /**
     * Require the use of a smart handler
     */
    use_smart_handler() {
        wasm.xmpoptions_use_smart_handler(this.__wbg_ptr);
    }
}
if (Symbol.dispose) XmpOptions.prototype[Symbol.dispose] = XmpOptions.prototype.free;

/**
 * XMP property value types
 *
 * Represents different types of values that can be stored in XMP properties.
 */
export class XmpValue {
    static __wrap(ptr) {
        const obj = Object.create(XmpValue.prototype);
        obj.__wbg_ptr = ptr;
        XmpValueFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        XmpValueFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_xmpvalue_free(ptr, 0);
    }
    /**
     * Get the value as a boolean, if it is a boolean type
     * @returns {boolean | undefined}
     */
    as_boolean() {
        const ret = wasm.xmpvalue_as_boolean(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret !== 0;
    }
    /**
     * Get the value as a date/time string, if it is a date/time type
     * @returns {string | undefined}
     */
    as_date_time() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.xmpvalue_as_date_time(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_export5(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get the value as an integer, if it is an integer type
     * @returns {bigint | undefined}
     */
    as_integer() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.xmpvalue_as_integer(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r2 = getDataViewMemory0().getBigInt64(retptr + 8 * 1, true);
            return r0 === 0 ? undefined : r2;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get the value as a string, if it is a string type
     * @returns {string | undefined}
     */
    as_string() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.xmpvalue_as_string(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_export5(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Create a boolean value
     * @param {boolean} b
     * @returns {XmpValue}
     */
    static boolean(b) {
        const ret = wasm.xmpvalue_boolean(b);
        return XmpValue.__wrap(ret);
    }
    /**
     * Create a date/time value
     * @param {string} dt
     * @returns {XmpValue}
     */
    static date_time(dt) {
        const ptr0 = passStringToWasm0(dt, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.xmpvalue_date_time(ptr0, len0);
        return XmpValue.__wrap(ret);
    }
    /**
     * Create an integer value
     * @param {bigint} i
     * @returns {XmpValue}
     */
    static integer(i) {
        const ret = wasm.xmpvalue_integer(i);
        return XmpValue.__wrap(ret);
    }
    /**
     * Get the value kind
     * @returns {XmpValueKind}
     */
    get kind() {
        const ret = wasm.xmpvalue_kind(this.__wbg_ptr);
        return ret;
    }
    /**
     * Create a string value
     * @param {string} s
     */
    constructor(s) {
        const ptr0 = passStringToWasm0(s, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.xmpvalue_string(ptr0, len0);
        this.__wbg_ptr = ret;
        XmpValueFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) XmpValue.prototype[Symbol.dispose] = XmpValue.prototype.free;

/**
 * XMP value type kind
 * @enum {0 | 1 | 2 | 3}
 */
export const XmpValueKind = Object.freeze({
    /**
     * String value
     */
    String: 0, "0": "String",
    /**
     * Integer value
     */
    Integer: 1, "1": "Integer",
    /**
     * Boolean value
     */
    Boolean: 2, "2": "Boolean",
    /**
     * Date/time value
     */
    DateTime: 3, "3": "DateTime",
});

/**
 * Read the first SYLT frame from MP3 `bytes` and return the timed lines as a
 * JSON array string ("[]" when the file has none). Name kept as
 * `extract_sylt` so the existing id3-wasm JS wrapper binds unchanged.
 * @param {Uint8Array} bytes
 * @returns {string}
 */
export function extract_sylt(bytes) {
    let deferred2_0;
    let deferred2_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        wasm.extract_sylt(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred2_0 = r0;
        deferred2_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export5(deferred2_0, deferred2_1, 1);
    }
}

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
 * @returns {any}
 */
export function get_all_registered_namespaces() {
    const ret = wasm.get_all_registered_namespaces();
    return takeObject(ret);
}

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
 * @returns {string[]}
 */
export function get_builtin_namespace_uris() {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.get_builtin_namespace_uris(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
        wasm.__wbindgen_export5(r0, r1 * 4, 4);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Get the prefix for a namespace URI
 * @param {string} uri
 * @returns {string | undefined}
 */
export function get_namespace_prefix(uri) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(uri, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.get_namespace_prefix(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        let v2;
        if (r0 !== 0) {
            v2 = getStringFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export5(r0, r1 * 1, 1);
        }
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Get the URI for a namespace prefix
 * @param {string} prefix
 * @returns {string | undefined}
 */
export function get_namespace_uri(prefix) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(prefix, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.get_namespace_uri(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        let v2;
        if (r0 !== 0) {
            v2 = getStringFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export5(r0, r1 * 1, 1);
        }
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * Check if a namespace URI is registered
 * @param {string} uri
 * @returns {boolean}
 */
export function is_namespace_registered(uri) {
    const ptr0 = passStringToWasm0(uri, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.is_namespace_registered(ptr0, len0);
    return ret !== 0;
}

/**
 * Get the namespace prefix for a Namespace enum value
 * @param {Namespace} ns
 * @returns {string}
 */
export function namespace_prefix(ns) {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.namespace_prefix(retptr, ns);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export5(deferred1_0, deferred1_1, 1);
    }
}

/**
 * Get the namespace URI for a Namespace enum value
 *
 * # Example
 *
 * ```javascript
 * import { Namespace, namespace_uri } from './pkg/xmpkit.js';
 * meta.set_property(namespace_uri(Namespace.Xmp), "CreatorTool", "MyApp");
 * ```
 * @param {Namespace} ns
 * @returns {string}
 */
export function namespace_uri(ns) {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.namespace_uri(retptr, ns);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export5(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @param {string} path
 * @param {string | null | undefined} prompt
 * @param {string | null | undefined} model
 * @param {string[]} subjects
 * @returns {Promise<number>}
 */
export function psxmp_embed(path, prompt, model, subjects) {
    const ptr0 = passStringToWasm0(path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    var ptr1 = isLikeNone(prompt) ? 0 : passStringToWasm0(prompt, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    var len1 = WASM_VECTOR_LEN;
    var ptr2 = isLikeNone(model) ? 0 : passStringToWasm0(model, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    var len2 = WASM_VECTOR_LEN;
    const ptr3 = passArrayJsValueToWasm0(subjects, wasm.__wbindgen_export);
    const len3 = WASM_VECTOR_LEN;
    const ret = wasm.psxmp_embed(ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
    return takeObject(ret);
}

/**
 * @param {string} path
 * @returns {Promise<string | undefined>}
 */
export function psxmp_read_model(path) {
    const ptr0 = passStringToWasm0(path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.psxmp_read_model(ptr0, len0);
    return takeObject(ret);
}

/**
 * @param {string} path
 * @returns {Promise<string | undefined>}
 */
export function psxmp_read_prompt(path) {
    const ptr0 = passStringToWasm0(path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.psxmp_read_prompt(ptr0, len0);
    return takeObject(ret);
}

/**
 * @param {string} path
 * @param {string} namespace_uri
 * @param {string} property_name
 * @returns {Promise<string | undefined>}
 */
export function psxmp_read_property(path, namespace_uri, property_name) {
    const ptr0 = passStringToWasm0(path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(namespace_uri, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(property_name, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.psxmp_read_property(ptr0, len0, ptr1, len1, ptr2, len2);
    return takeObject(ret);
}

/**
 * @param {string} path
 * @returns {Promise<number>}
 */
export function psxmp_read_rating(path) {
    const ptr0 = passStringToWasm0(path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.psxmp_read_rating(ptr0, len0);
    return takeObject(ret);
}

/**
 * @param {string} path
 * @param {number} index
 * @returns {Promise<string | undefined>}
 */
export function psxmp_read_subject_at(path, index) {
    const ptr0 = passStringToWasm0(path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.psxmp_read_subject_at(ptr0, len0, index);
    return takeObject(ret);
}

/**
 * @param {string} path
 * @returns {Promise<number>}
 */
export function psxmp_read_subject_count(path) {
    const ptr0 = passStringToWasm0(path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.psxmp_read_subject_count(ptr0, len0);
    return takeObject(ret);
}

/**
 * @param {string} path
 * @param {number} rating
 * @returns {Promise<number>}
 */
export function psxmp_set_rating(path, rating) {
    const ptr0 = passStringToWasm0(path, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.psxmp_set_rating(ptr0, len0, rating);
    return takeObject(ret);
}

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
 * @param {string} uri
 * @param {string} prefix
 */
export function register_namespace(uri, prefix) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(uri, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(prefix, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len1 = WASM_VECTOR_LEN;
        wasm.register_namespace(retptr, ptr0, len0, ptr1, len1);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {string} user
 * @param {string} password
 * @param {string} image_b64
 * @param {string} prompt
 * @returns {Promise<Uint8Array>}
 */
export function wasm_flux2_dev_i2i(user, password, image_b64, prompt) {
    const ptr0 = passStringToWasm0(user, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(password, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(image_b64, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passStringToWasm0(prompt, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len3 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_flux2_dev_i2i(ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
    return takeObject(ret);
}

/**
 * @param {string} user
 * @param {string} password
 * @param {string} image_b64
 * @param {string} image2_b64
 * @param {string} prompt
 * @returns {Promise<Uint8Array>}
 */
export function wasm_flux2_klein_i2i(user, password, image_b64, image2_b64, prompt) {
    const ptr0 = passStringToWasm0(user, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(password, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(image_b64, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passStringToWasm0(image2_b64, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len3 = WASM_VECTOR_LEN;
    const ptr4 = passStringToWasm0(prompt, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len4 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_flux2_klein_i2i(ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4);
    return takeObject(ret);
}

/**
 * @param {string} user
 * @param {string} password
 * @param {string} prompt
 * @returns {Promise<Uint8Array>}
 */
export function wasm_flux2_pro(user, password, prompt) {
    const ptr0 = passStringToWasm0(user, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(password, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(prompt, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_flux2_pro(ptr0, len0, ptr1, len1, ptr2, len2);
    return takeObject(ret);
}

/**
 * @param {string} user
 * @param {string} password
 * @param {string} image_b64
 * @param {string} audio_b64
 * @param {string} prompt
 * @returns {Promise<Uint8Array>}
 */
export function wasm_ltx2_3a2v(user, password, image_b64, audio_b64, prompt) {
    const ptr0 = passStringToWasm0(user, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(password, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(image_b64, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passStringToWasm0(audio_b64, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len3 = WASM_VECTOR_LEN;
    const ptr4 = passStringToWasm0(prompt, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len4 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_ltx2_3a2v(ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4);
    return takeObject(ret);
}

/**
 * @param {string} user
 * @param {string} password
 * @param {string} prompt
 * @returns {Promise<Uint8Array>}
 */
export function wasm_nano_banana2(user, password, prompt) {
    const ptr0 = passStringToWasm0(user, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(password, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(prompt, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_nano_banana2(ptr0, len0, ptr1, len1, ptr2, len2);
    return takeObject(ret);
}

/**
 * @param {string} user
 * @param {string} password
 * @param {string} messages_json
 * @returns {Promise<string>}
 */
export function wasm_qwen3_6_35b_a3b(user, password, messages_json) {
    const ptr0 = passStringToWasm0(user, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(password, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(messages_json, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_qwen3_6_35b_a3b(ptr0, len0, ptr1, len1, ptr2, len2);
    return takeObject(ret);
}

/**
 * @param {string} user
 * @param {string} password
 * @param {string} audio_b64
 * @returns {Promise<string>}
 */
export function wasm_qwen3_asr_flash(user, password, audio_b64) {
    const ptr0 = passStringToWasm0(user, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(password, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(audio_b64, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_qwen3_asr_flash(ptr0, len0, ptr1, len1, ptr2, len2);
    return takeObject(ret);
}

/**
 * @param {string} user
 * @param {string} password
 * @param {string} prompt
 * @returns {Promise<Uint8Array>}
 */
export function wasm_z_image_turbo(user, password, prompt) {
    const ptr0 = passStringToWasm0(user, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(password, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(prompt, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len2 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_z_image_turbo(ptr0, len0, ptr1, len1, ptr2, len2);
    return takeObject(ret);
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_debug_string_c25d447a39f5578f: function(arg0, arg1) {
            const ret = debugString(getObject(arg1));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_is_function_1ff95bcc5517c252: function(arg0) {
            const ret = typeof(getObject(arg0)) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_object_a27215656b807791: function(arg0) {
            const val = getObject(arg0);
            const ret = typeof(val) === 'object' && val !== null;
            return ret;
        },
        __wbg___wbindgen_is_undefined_c05833b95a3cf397: function(arg0) {
            const ret = getObject(arg0) === undefined;
            return ret;
        },
        __wbg___wbindgen_string_get_b0ca35b86a603356: function(arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_344f42d3211c4765: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg__wbg_cb_unref_fffb441def202758: function(arg0) {
            getObject(arg0)._wbg_cb_unref();
        },
        __wbg_abort_8bae0f33e7833997: function(arg0) {
            getObject(arg0).abort();
        },
        __wbg_abort_eee9248a6d680839: function(arg0, arg1) {
            getObject(arg0).abort(getObject(arg1));
        },
        __wbg_append_01c74e5c6b58aa64: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).append(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_arrayBuffer_3b637f0fa65c5351: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).arrayBuffer();
            return addHeapObject(ret);
        }, arguments); },
        __wbg_arrayBuffer_a158e423a87ee756: function(arg0) {
            const ret = getObject(arg0).arrayBuffer();
            return addHeapObject(ret);
        },
        __wbg_call_8a2dd23819f8a60a: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg0).call(getObject(arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_call_a6e5c5dce5018821: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_clearTimeout_6b8d9a38b9263d65: function(arg0) {
            const ret = clearTimeout(takeObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_close_175f8e039af34f61: function(arg0) {
            const ret = getObject(arg0).close();
            return addHeapObject(ret);
        },
        __wbg_createWritable_659288d5245034d3: function(arg0) {
            const ret = getObject(arg0).createWritable();
            return addHeapObject(ret);
        },
        __wbg_done_89b2b13e91a60321: function(arg0) {
            const ret = getObject(arg0).done;
            return ret;
        },
        __wbg_fetch_9dad4fe911207b37: function(arg0) {
            const ret = fetch(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_fetch_b5951fc96f52f786: function(arg0, arg1) {
            const ret = getObject(arg0).fetch(getObject(arg1));
            return addHeapObject(ret);
        },
        __wbg_getDirectory_389283588dfb8117: function(arg0) {
            const ret = getObject(arg0).getDirectory();
            return addHeapObject(ret);
        },
        __wbg_getFileHandle_72de55ab3ca9ad57: function(arg0, arg1, arg2, arg3) {
            const ret = getObject(arg0).getFileHandle(getStringFromWasm0(arg1, arg2), getObject(arg3));
            return addHeapObject(ret);
        },
        __wbg_getFileHandle_96903ab38e634823: function(arg0, arg1, arg2) {
            const ret = getObject(arg0).getFileHandle(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        },
        __wbg_getFile_bdc0144baa662031: function(arg0) {
            const ret = getObject(arg0).getFile();
            return addHeapObject(ret);
        },
        __wbg_getRandomValues_bf16787eede473f5: function() { return handleError(function (arg0, arg1) {
            globalThis.crypto.getRandomValues(getArrayU8FromWasm0(arg0, arg1));
        }, arguments); },
        __wbg_get_c7eb1f358a7654df: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(getObject(arg0), getObject(arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_has_8374cf06984d8bfc: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.has(getObject(arg0), getObject(arg1));
            return ret;
        }, arguments); },
        __wbg_headers_cf9c80f30e2a4eff: function(arg0) {
            const ret = getObject(arg0).headers;
            return addHeapObject(ret);
        },
        __wbg_instanceof_FileSystemDirectoryHandle_c9ab7c5cdb7a7c30: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof FileSystemDirectoryHandle;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_FileSystemFileHandle_68e80b30532d5f04: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof FileSystemFileHandle;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_FileSystemWritableFileStream_bbd33ec1789b2714: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof FileSystemWritableFileStream;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_File_ee62de53bca2e697: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof File;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Response_c8b64b2256f01bec: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Response;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Window_05ba1ee4f6781663: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_iterator_6f722e4a93058b71: function() {
            const ret = Symbol.iterator;
            return addHeapObject(ret);
        },
        __wbg_length_1f0964f4a5e2c6d8: function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        },
        __wbg_navigator_99621db14b3f1099: function(arg0) {
            const ret = getObject(arg0).navigator;
            return addHeapObject(ret);
        },
        __wbg_new_0d809930cd1354c6: function() { return handleError(function () {
            const ret = new Headers();
            return addHeapObject(ret);
        }, arguments); },
        __wbg_new_4339b2a2675a03e3: function() { return handleError(function () {
            const ret = new AbortController();
            return addHeapObject(ret);
        }, arguments); },
        __wbg_new_cd45aabdf6073e84: function(arg0) {
            const ret = new Uint8Array(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_new_da52cf8fe3429cb2: function() {
            const ret = new Object();
            return addHeapObject(ret);
        },
        __wbg_new_from_slice_77cdfb7977362f3c: function(arg0, arg1) {
            const ret = new Uint8Array(getArrayU8FromWasm0(arg0, arg1));
            return addHeapObject(ret);
        },
        __wbg_new_typed_1824d93f294193e5: function(arg0, arg1) {
            try {
                var state0 = {a: arg0, b: arg1};
                var cb0 = (arg0, arg1) => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return __wasm_bindgen_func_elem_2822(a, state0.b, arg0, arg1);
                    } finally {
                        state0.a = a;
                    }
                };
                const ret = new Promise(cb0);
                return addHeapObject(ret);
            } finally {
                state0.a = 0;
            }
        },
        __wbg_new_with_str_and_init_d95cbe11ce28e65e: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = new Request(getStringFromWasm0(arg0, arg1), getObject(arg2));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_next_6dbf2c0ac8cde20f: function(arg0) {
            const ret = getObject(arg0).next;
            return addHeapObject(ret);
        },
        __wbg_next_71f2aa1cb3d1e37e: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).next();
            return addHeapObject(ret);
        }, arguments); },
        __wbg_now_8e942e0f83109abc: function() { return handleError(function () {
            const ret = Date.now();
            return ret;
        }, arguments); },
        __wbg_prototypesetcall_4770620bbe4688a0: function(arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), getObject(arg2));
        },
        __wbg_queueMicrotask_0ab5b2d2393e99b9: function(arg0) {
            const ret = getObject(arg0).queueMicrotask;
            return addHeapObject(ret);
        },
        __wbg_queueMicrotask_6a09b7bc46549209: function(arg0) {
            queueMicrotask(getObject(arg0));
        },
        __wbg_resolve_2191a4dfe481c25b: function(arg0) {
            const ret = Promise.resolve(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_setTimeout_f757f00851f76c42: function(arg0, arg1) {
            const ret = setTimeout(getObject(arg0), arg1);
            return addHeapObject(ret);
        },
        __wbg_set_8535240470bf2500: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
            return ret;
        }, arguments); },
        __wbg_set_body_029f2d171e0a005f: function(arg0, arg1) {
            getObject(arg0).body = getObject(arg1);
        },
        __wbg_set_cache_b4a740b195c051f4: function(arg0, arg1) {
            getObject(arg0).cache = __wbindgen_enum_RequestCache[arg1];
        },
        __wbg_set_create_a807a6e9ac628698: function(arg0, arg1) {
            getObject(arg0).create = arg1 !== 0;
        },
        __wbg_set_credentials_bb34a40189e3b43b: function(arg0, arg1) {
            getObject(arg0).credentials = __wbindgen_enum_RequestCredentials[arg1];
        },
        __wbg_set_headers_9c61d123c3ee1f10: function(arg0, arg1) {
            getObject(arg0).headers = getObject(arg1);
        },
        __wbg_set_method_5532d59b92d76467: function(arg0, arg1, arg2) {
            getObject(arg0).method = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_mode_66c79886ad78fc05: function(arg0, arg1) {
            getObject(arg0).mode = __wbindgen_enum_RequestMode[arg1];
        },
        __wbg_set_signal_c4ef8faddb4c1446: function(arg0, arg1) {
            getObject(arg0).signal = getObject(arg1);
        },
        __wbg_signal_dad7cb35193abd31: function(arg0) {
            const ret = getObject(arg0).signal;
            return addHeapObject(ret);
        },
        __wbg_static_accessor_GLOBAL_4ef717fb391d88b7: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_GLOBAL_THIS_8d1badc68b5a74f4: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_SELF_146583524fe1469b: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_WINDOW_f2829a2234d7819e: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_status_c45b3b9b3033184a: function(arg0) {
            const ret = getObject(arg0).status;
            return ret;
        },
        __wbg_storage_756400487605531a: function(arg0) {
            const ret = getObject(arg0).storage;
            return addHeapObject(ret);
        },
        __wbg_stringify_b54333f60f1e4dad: function() { return handleError(function (arg0) {
            const ret = JSON.stringify(getObject(arg0));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_then_16d107c451e9905d: function(arg0, arg1, arg2) {
            const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        },
        __wbg_then_6ec10ae38b3e92f7: function(arg0, arg1) {
            const ret = getObject(arg0).then(getObject(arg1));
            return addHeapObject(ret);
        },
        __wbg_url_abdb8fb08377f8c0: function(arg0, arg1) {
            const ret = getObject(arg1).url;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_value_a5d5488a9589444a: function(arg0) {
            const ret = getObject(arg0).value;
            return addHeapObject(ret);
        },
        __wbg_write_5000d1e9282930ef: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg0).write(getObject(arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_xmperror_new: function(arg0) {
            const ret = XmpError.__wrap(arg0);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 305, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, __wasm_bindgen_func_elem_2809);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000002: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [], shim_idx: 151, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, __wasm_bindgen_func_elem_763);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000003: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000004: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        },
        __wbindgen_object_clone_ref: function(arg0) {
            const ret = getObject(arg0);
            return addHeapObject(ret);
        },
        __wbindgen_object_drop_ref: function(arg0) {
            takeObject(arg0);
        },
    };
    return {
        __proto__: null,
        "./rust_ffi_bg.js": import0,
    };
}

function __wasm_bindgen_func_elem_763(arg0, arg1) {
    wasm.__wasm_bindgen_func_elem_763(arg0, arg1);
}

function __wasm_bindgen_func_elem_2809(arg0, arg1, arg2) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.__wasm_bindgen_func_elem_2809(retptr, arg0, arg1, addHeapObject(arg2));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

function __wasm_bindgen_func_elem_2822(arg0, arg1, arg2, arg3) {
    wasm.__wasm_bindgen_func_elem_2822(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}


const __wbindgen_enum_RequestCache = ["default", "no-store", "reload", "no-cache", "force-cache", "only-if-cached"];


const __wbindgen_enum_RequestCredentials = ["omit", "same-origin", "include"];


const __wbindgen_enum_RequestMode = ["same-origin", "no-cors", "cors", "navigate"];
const QualifierFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_qualifier_free(ptr, 1));
const XmpDateTimeFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_xmpdatetime_free(ptr, 1));
const XmpErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_xmperror_free(ptr, 1));
const XmpFileFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_xmpfile_free(ptr, 1));
const XmpMetaFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_xmpmeta_free(ptr, 1));
const XmpOptionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_xmpoptions_free(ptr, 1));
const XmpValueFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_xmpvalue_free(ptr, 1));

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => wasm.__wbindgen_export4(state.a, state.b));

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function dropObject(idx) {
    if (idx < 1028) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(takeObject(mem.getUint32(i, true)));
    }
    return result;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getObject(idx) { return heap[idx]; }

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_export3(addHeapObject(e));
    }
}

let heap = new Array(1024).fill(undefined);
heap.push(undefined, null, true, false);

let heap_next = heap.length;

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeMutClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_export4(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    const mem = getDataViewMemory0();
    for (let i = 0; i < array.length; i++) {
        mem.setUint32(ptr + 4 * i, addHeapObject(array[i]), true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('rust_ffi_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
