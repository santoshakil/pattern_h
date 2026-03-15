package {{package}}.channels

import android.os.Build
import io.flutter.plugin.common.BinaryMessenger
import io.flutter.plugin.common.MethodChannel

object DeviceInfoChannel {
    private const val NAME = "{{channel_prefix}}/device"

    fun register(messenger: BinaryMessenger) {
        MethodChannel(messenger, NAME).setMethodCallHandler { call, result ->
            when (call.method) {
                "getDeviceInfo" -> result.success(
                    mapOf(
                        "platform" to "Android",
                        "version" to Build.VERSION.RELEASE,
                        "model" to Build.MODEL,
                        "sdk" to Build.VERSION.SDK_INT
                    )
                )
                else -> result.notImplemented()
            }
        }
    }
}
