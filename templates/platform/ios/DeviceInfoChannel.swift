import Flutter

struct DeviceInfoChannel {
  static let name = "{{channel_prefix}}/device"

  static func register(with messenger: FlutterBinaryMessenger) {
    let channel = FlutterMethodChannel(name: name, binaryMessenger: messenger)
    channel.setMethodCallHandler { call, result in
      switch call.method {
      case "getDeviceInfo":
        let device = UIDevice.current
        result([
          "platform": "iOS",
          "version": device.systemVersion,
          "model": device.model,
          "name": device.name
        ])
      default:
        result(FlutterMethodNotImplemented)
      }
    }
  }
}
