import FlutterMacOS

struct DeviceInfoChannel {
  static let name = "{{channel_prefix}}/device"

  static func register(with messenger: FlutterBinaryMessenger) {
    let channel = FlutterMethodChannel(name: name, binaryMessenger: messenger)
    channel.setMethodCallHandler { call, result in
      switch call.method {
      case "getDeviceInfo":
        let info = ProcessInfo.processInfo
        result([
          "platform": "macOS",
          "version": info.operatingSystemVersionString,
          "hostname": Host.current().localizedName ?? "unknown",
          "processors": info.processorCount
        ])
      default:
        result(FlutterMethodNotImplemented)
      }
    }
  }
}
