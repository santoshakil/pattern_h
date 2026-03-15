package {{package}}.channels

import io.flutter.plugin.common.BinaryMessenger

object ChannelRegistry {
    fun registerAll(messenger: BinaryMessenger) {
        DeviceInfoChannel.register(messenger)
    }
}
