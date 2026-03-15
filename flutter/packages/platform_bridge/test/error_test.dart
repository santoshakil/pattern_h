import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:platform_bridge/platform_bridge.dart';

void main() {
  group('PlatformBridgeException.from', () {
    test('UNAVAILABLE maps to FeatureUnavailableException', () {
      final e = PlatformBridgeException.from(
        PlatformException(code: 'UNAVAILABLE', message: 'no hw'),
      );
      expect(e, isA<FeatureUnavailableException>());
      expect(e.code, 'UNAVAILABLE');
      expect(e.message, 'no hw');
    });

    test('PERMISSION_DENIED maps to PermissionDeniedException', () {
      final e = PlatformBridgeException.from(
        PlatformException(code: 'PERMISSION_DENIED', message: 'denied'),
      );
      expect(e, isA<PermissionDeniedException>());
      expect(e.code, 'PERMISSION_DENIED');
    });

    test('INVALID_ARGUMENT maps to InvalidArgumentException', () {
      final e = PlatformBridgeException.from(
        PlatformException(code: 'INVALID_ARGUMENT', message: 'bad arg'),
      );
      expect(e, isA<InvalidArgumentException>());
      expect(e.code, 'INVALID_ARGUMENT');
    });

    test('NOT_FOUND maps to MethodNotFoundException', () {
      final e = PlatformBridgeException.from(
        PlatformException(code: 'NOT_FOUND', message: 'missing'),
      );
      expect(e, isA<MethodNotFoundException>());
      expect(e.code, 'NOT_FOUND');
    });

    test('ALREADY_IN_USE maps to ResourceInUseException', () {
      final e = PlatformBridgeException.from(
        PlatformException(code: 'ALREADY_IN_USE', message: 'busy'),
      );
      expect(e, isA<ResourceInUseException>());
      expect(e.code, 'ALREADY_IN_USE');
    });

    test('TIMEOUT maps to ChannelTimeoutException', () {
      final e = PlatformBridgeException.from(
        PlatformException(code: 'TIMEOUT', message: 'slow'),
      );
      expect(e, isA<ChannelTimeoutException>());
      expect(e.code, 'TIMEOUT');
    });

    test('CANCELLED maps to CancelledException', () {
      final e = PlatformBridgeException.from(
        PlatformException(code: 'CANCELLED', message: 'abort'),
      );
      expect(e, isA<CancelledException>());
      expect(e.code, 'CANCELLED');
    });

    test('unknown code maps to UnknownPlatformException', () {
      final e = PlatformBridgeException.from(
        PlatformException(code: 'CUSTOM_ERR', message: 'wat'),
      );
      expect(e, isA<UnknownPlatformException>());
      expect(e.code, 'CUSTOM_ERR');
      expect(e.message, 'wat');
    });

    test('preserves details', () {
      final e = PlatformBridgeException.from(
        PlatformException(code: 'TIMEOUT', message: 'x', details: {'k': 1}),
      );
      expect(e.details, {'k': 1});
    });

    test('null message becomes empty string', () {
      final e = PlatformBridgeException.from(
        PlatformException(code: 'TIMEOUT'),
      );
      expect(e.message, '');
    });
  });

  group('PlatformBridgeException.toString', () {
    test('contains code and message', () {
      final e = PlatformBridgeException.from(
        PlatformException(code: 'TIMEOUT', message: 'too slow'),
      );
      expect(e.toString(), 'PlatformBridgeException(TIMEOUT): too slow');
    });
  });
}
