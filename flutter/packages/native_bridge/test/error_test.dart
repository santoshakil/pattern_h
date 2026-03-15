import 'package:test/test.dart';
import 'package:native_bridge/src/error.dart';

void main() {
  group('FfiException.fromCode', () {
    test('-1 maps to PanicException', () {
      final e = FfiException.fromCode(-1, 'panic');
      expect(e, isA<PanicException>());
      expect(e.code, -1);
      expect(e.message, 'panic');
    });

    test('1 maps to NullPointerException', () {
      final e = FfiException.fromCode(1, 'null ptr');
      expect(e, isA<NullPointerException>());
      expect(e.code, 1);
    });

    test('2 maps to InvalidHandleException', () {
      final e = FfiException.fromCode(2, 'bad handle');
      expect(e, isA<InvalidHandleException>());
      expect(e.code, 2);
    });

    test('3 maps to BufferOverflowException', () {
      final e = FfiException.fromCode(3, 'overflow');
      expect(e, isA<BufferOverflowException>());
      expect(e.code, 3);
    });

    test('4 maps to InvalidUtf8Exception', () {
      final e = FfiException.fromCode(4, 'bad utf8');
      expect(e, isA<InvalidUtf8Exception>());
      expect(e.code, 4);
    });

    test('5 maps to DecodeException', () {
      final e = FfiException.fromCode(5, 'decode fail');
      expect(e, isA<DecodeException>());
      expect(e.code, 5);
    });

    test('6 maps to EncodeException', () {
      final e = FfiException.fromCode(6, 'encode fail');
      expect(e, isA<EncodeException>());
      expect(e.code, 6);
    });

    test('7 maps to NotInitializedException', () {
      final e = FfiException.fromCode(7, 'not init');
      expect(e, isA<NotInitializedException>());
      expect(e.code, 7);
    });

    test('8 maps to AlreadyInitializedException', () {
      final e = FfiException.fromCode(8, 'already init');
      expect(e, isA<AlreadyInitializedException>());
      expect(e.code, 8);
    });

    test('9 maps to RuntimeInitException', () {
      final e = FfiException.fromCode(9, 'runtime fail');
      expect(e, isA<RuntimeInitException>());
      expect(e.code, 9);
    });

    test('100 maps to DomainException', () {
      final e = FfiException.fromCode(100, 'domain err');
      expect(e, isA<DomainException>());
      expect(e.code, 100);
    });

    test('200 maps to StorageException', () {
      final e = FfiException.fromCode(200, 'storage err');
      expect(e, isA<StorageException>());
      expect(e.code, 200);
    });

    test('unknown code maps to UnknownFfiException', () {
      final e = FfiException.fromCode(999, 'unknown');
      expect(e, isA<UnknownFfiException>());
      expect(e.code, 999);
      expect(e.message, 'unknown');
    });

    test('another unknown code maps to UnknownFfiException', () {
      final e = FfiException.fromCode(42, 'mystery');
      expect(e, isA<UnknownFfiException>());
      expect(e.code, 42);
    });
  });

  group('FfiException.toString', () {
    test('format is FfiException(code): message', () {
      final e = FfiException.fromCode(5, 'decode fail');
      expect(e.toString(), 'FfiException(5): decode fail');
    });

    test('panic format', () {
      final e = FfiException.fromCode(-1, 'rust panicked');
      expect(e.toString(), 'FfiException(-1): rust panicked');
    });

    test('unknown format preserves code', () {
      final e = FfiException.fromCode(999, 'oops');
      expect(e.toString(), 'FfiException(999): oops');
    });
  });
}
