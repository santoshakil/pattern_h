sealed class FfiException implements Exception {
  final int code;
  final String message;

  const FfiException(this.code, this.message);

  factory FfiException.fromCode(int code, String message) => switch (code) {
    -1 => PanicException(message),
    1 => NullPointerException(message),
    2 => InvalidHandleException(message),
    3 => BufferOverflowException(message),
    4 => InvalidUtf8Exception(message),
    5 => DecodeException(message),
    6 => EncodeException(message),
    7 => NotInitializedException(message),
    8 => AlreadyInitializedException(message),
    9 => RuntimeInitException(message),
    100 => DomainException(message),
    200 => StorageException(message),
    _ => UnknownFfiException(code, message),
  };

  @override
  String toString() => 'FfiException($code): $message';
}

final class NullPointerException extends FfiException {
  const NullPointerException(String message) : super(1, message);
}

final class InvalidHandleException extends FfiException {
  const InvalidHandleException(String message) : super(2, message);
}

final class BufferOverflowException extends FfiException {
  const BufferOverflowException(String message) : super(3, message);
}

final class InvalidUtf8Exception extends FfiException {
  const InvalidUtf8Exception(String message) : super(4, message);
}

final class DecodeException extends FfiException {
  const DecodeException(String message) : super(5, message);
}

final class EncodeException extends FfiException {
  const EncodeException(String message) : super(6, message);
}

final class NotInitializedException extends FfiException {
  const NotInitializedException(String message) : super(7, message);
}

final class AlreadyInitializedException extends FfiException {
  const AlreadyInitializedException(String message) : super(8, message);
}

final class DomainException extends FfiException {
  const DomainException(String message) : super(100, message);
}

final class StorageException extends FfiException {
  const StorageException(String message) : super(200, message);
}

final class PanicException extends FfiException {
  const PanicException(String message) : super(-1, message);
}

final class RuntimeInitException extends FfiException {
  const RuntimeInitException(String message) : super(9, message);
}

final class UnknownFfiException extends FfiException {
  const UnknownFfiException(super.code, super.message);
}
