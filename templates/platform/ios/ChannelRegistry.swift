import Flutter

struct ChannelRegistry {
  static func registerAll(with messenger: FlutterBinaryMessenger) {
    DeviceInfoChannel.register(with: messenger)
  }
}
