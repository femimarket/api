package market.femi.api

import kotlinx.coroutines.suspendCancellableCoroutine
import java.nio.Buffer
import java.nio.ByteBuffer
import kotlin.coroutines.resume
import kotlin.coroutines.resumeWithException

private val addressField = Buffer::class.java.getDeclaredField("address").apply { isAccessible = true }

/**
 * Runs a blocking native FFI call, passing the address of an off-heap `cancel_flag` byte.
 * If the coroutine is cancelled, the byte is flipped to 1 so the native request aborts early
 * and returns the fallback — mirroring Swift's withTaskCancellationHandler.
 */
internal suspend fun runCancelable(call: (cancelAddr: Long) -> ByteArray): ByteArray =
    suspendCancellableCoroutine { cont ->
        val flag = ByteBuffer.allocateDirect(1)
        flag.put(0, 0)
        val addr = addressField.getLong(flag) // native address of the direct buffer's byte
        cont.invokeOnCancellation { flag.put(0, 1) }
        Thread {
            try {
                val result = call(addr)
                flag.get(0) // keep `flag` reachable until the native call has returned
                if (cont.isActive) cont.resume(result)
            } catch (e: Throwable) {
                if (cont.isActive) cont.resumeWithException(e)
            }
        }.apply { isDaemon = true; start() }
    }
