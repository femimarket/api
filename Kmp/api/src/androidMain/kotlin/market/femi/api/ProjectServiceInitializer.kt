package market.femi.api

import android.content.Context
import androidx.startup.Initializer

/// Self-configuration of the ProjectService documents root on Android.
///
/// Apple and web resolve their sandboxed root inside Rust (`dirs::document_dir`,
/// OPFS); Android's sandbox path is only knowable at runtime via a Context, so
/// the OS delivers one here at process start (androidx.startup, registered in
/// this library's manifest) and we point Rust at `filesDir`. Consumers write no
/// setup code on any platform. `initDocuments` stays public as an override for
/// apps that want a custom root.
class ProjectServiceInitializer : Initializer<Unit> {
    override fun create(context: Context) {
        ProjectService.initDocuments(context.filesDir.absolutePath)
    }

    override fun dependencies(): List<Class<out Initializer<*>>> = emptyList()
}
