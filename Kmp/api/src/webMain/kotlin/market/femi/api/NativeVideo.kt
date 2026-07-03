package market.femi.api

import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.drawBehind
import androidx.compose.ui.graphics.BlendMode
import androidx.compose.ui.graphics.Color
import kotlinx.browser.document
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import org.w3c.dom.HTMLElement
import org.w3c.dom.HTMLVideoElement
import org.w3c.dom.events.Event
import androidx.compose.ui.viewinterop.WebElementView


class NativeVideoState {
    var play: () -> Unit = {}
    var pause: () -> Unit = {}
    var seekTo: (Long) -> Unit = {}
    var mute: (Boolean) -> Unit = {}

    internal val _currentPosition = MutableStateFlow(0L)
    val currentPosition: StateFlow<Long> = _currentPosition.asStateFlow()

    internal val _duration = MutableStateFlow(0L)
    val duration: StateFlow<Long> = _duration.asStateFlow()

    internal val _isPlaying = MutableStateFlow(false)
    val isPlaying: StateFlow<Boolean> = _isPlaying.asStateFlow()

    internal val _isMuted = MutableStateFlow(true)
    val isMuted: StateFlow<Boolean> = _isMuted.asStateFlow()
}


@OptIn(ExperimentalComposeUiApi::class)
@Composable
fun NativeVideo(
    url: String?,
    state: NativeVideoState,
    modifier: Modifier,
    showControls: Boolean,
    autoPlay: Boolean = true,
    poster: String? = null,
) {

    val videoElement = remember { document.createElement("video") as HTMLVideoElement }
    var validUrl by remember { mutableStateOf<String?>(
        null
    )}
    LaunchedEffect(url) {
        url?.let {
            validUrl = it
        }
    }

    DisposableEffect(validUrl, autoPlay, poster) {
        if (validUrl == null) {
            return@DisposableEffect onDispose {}
        }
        val url = validUrl!!

        videoElement.src = url
        videoElement.style.objectFit = "cover"
        videoElement.muted = true
        videoElement.loop = true
        videoElement.preload = "auto"
        videoElement.poster = poster ?: ""
        videoElement.setAttribute("playsinline", "true")
        videoElement.autoplay = autoPlay
        if (!autoPlay) videoElement.pause()

        state.play = { videoElement.play() }
        state.pause = { videoElement.pause() }
        state.seekTo = { videoElement.currentTime = it / 1000.0 }
        state.mute = {
            videoElement.muted = it
            state._isMuted.value = it
        }

        // Event Listeners
        val onTimeUpdate = { _: Event ->
            state._currentPosition.value = (videoElement.currentTime * 1000).toLong()
        }
        val onDurationChange = { _: Event ->
            state._duration.value = (videoElement.duration * 1000).toLong()
        }
        val onPlay = { _: Event -> state._isPlaying.value = true }
        val onPause = { _: Event -> state._isPlaying.value = false }

        videoElement.addEventListener("timeupdate", onTimeUpdate)
        videoElement.addEventListener("durationchange", onDurationChange)
        videoElement.addEventListener("play", onPlay)
        videoElement.addEventListener("pause", onPause)

        onDispose {
            videoElement.pause()
            videoElement.removeEventListener("timeupdate", onTimeUpdate)
            videoElement.removeEventListener("durationchange", onDurationChange)
            videoElement.removeEventListener("play", onPlay)
            videoElement.removeEventListener("pause", onPause)
            videoElement.src = ""
        }
    }

    WebElementView(
        factory = { videoElement },
        // 1. THE HOLE PUNCH: Clear the canvas pixels here
        modifier = modifier.drawBehind {
            drawRect(
                color = Color.Transparent,
                blendMode = BlendMode.Clear
            )
        },
        update = {
            it.controls = showControls
            // Ensure video fills the container
            it.style.width = "100%"
            it.style.height = "100%"

            it.onwheel = { event -> event.preventDefault() }

            // 2. THE FIX: Push the DOM Wrapper behind the Canvas
            val wrapper = it.parentElement as? HTMLElement
            if (wrapper != null) {
                wrapper.style.zIndex = "-1"
                // Ensure absolute positioning is set on the wrapper if needed
                if (wrapper.style.position.isEmpty()) {
                    wrapper.style.position = "absolute"
                }
            }
        }
    )
}

/**
 * Resolves a media URL the <video> element can load. A same-origin / served URL
 * (e.g. "splash.mp4" from resources, or a CDN URL) works as-is. Swap this for a
 * fetch + URL.createObjectURL(blob) only if you need cross-origin or in-memory media.
 */
suspend fun readUrl(url: String): String = url