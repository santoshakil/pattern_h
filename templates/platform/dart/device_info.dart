import 'package:platform_bridge/platform_bridge.dart';

class DeviceInfo {
  static final _channel = PlatformChannel('{{channel_prefix}}/device');

  static Future<Map<String, dynamic>> get() async {
    final result = await _channel.callMap<String, dynamic>('getDeviceInfo');
    return result ?? {};
  }
}
