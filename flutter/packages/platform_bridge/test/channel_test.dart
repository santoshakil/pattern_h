import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:platform_bridge/platform_bridge.dart';

void main() {
  setUp(() {
    TestWidgetsFlutterBinding.ensureInitialized();
    PlatformChannel.clearCache();
  });

  group('PlatformChannel', () {
    test('factory returns same instance for same name', () {
      final a = PlatformChannel('com.test/nfc');
      final b = PlatformChannel('com.test/nfc');
      expect(identical(a, b), isTrue);
    });

    test('factory returns different instances for different names', () {
      final a = PlatformChannel('com.test/nfc');
      final b = PlatformChannel('com.test/gps');
      expect(identical(a, b), isFalse);
    });

    test('clearCache removes all cached instances', () {
      final a = PlatformChannel('com.test/x');
      PlatformChannel.clearCache();
      final b = PlatformChannel('com.test/x');
      expect(identical(a, b), isFalse);
    });

    test('name property matches constructor arg', () {
      final ch = PlatformChannel('com.test/name');
      expect(ch.name, 'com.test/name');
    });
  });
}
