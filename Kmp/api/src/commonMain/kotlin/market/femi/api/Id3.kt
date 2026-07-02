package market.femi.api

/// Reads the first SYLT (synchronised lyrics) frame from MP3 `bytes` and
/// returns the timed lines as a JSON array string (`"[]"` when there are none).
expect suspend fun extractSylt(bytes: ByteArray): String
