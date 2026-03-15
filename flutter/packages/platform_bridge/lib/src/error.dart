import 'package:flutter/services.dart';

sealed class PlatformBridgeException implements Exception {
  final String code;
  final String message;
  final dynamic details;

  const PlatformBridgeException(this.code, this.message, [this.details]);

  factory PlatformBridgeException.from(PlatformException e) => switch (e.code) {
    'UNAVAILABLE' => FeatureUnavailableException(e.message ?? '', e.details),
    'PERMISSION_DENIED' => PermissionDeniedException(
      e.message ?? '',
      e.details,
    ),
    'INVALID_ARGUMENT' => InvalidArgumentException(e.message ?? '', e.details),
    'NOT_FOUND' => MethodNotFoundException(e.message ?? '', e.details),
    'ALREADY_IN_USE' => ResourceInUseException(e.message ?? '', e.details),
    'TIMEOUT' => ChannelTimeoutException(e.message ?? '', e.details),
    'CANCELLED' => CancelledException(e.message ?? '', e.details),
    _ => UnknownPlatformException(e.code, e.message ?? '', e.details),
  };

  @override
  String toString() => 'PlatformBridgeException($code): $message';
}

final class FeatureUnavailableException extends PlatformBridgeException {
  const FeatureUnavailableException(String message, [dynamic details])
    : super('UNAVAILABLE', message, details);
}

final class PermissionDeniedException extends PlatformBridgeException {
  const PermissionDeniedException(String message, [dynamic details])
    : super('PERMISSION_DENIED', message, details);
}

final class InvalidArgumentException extends PlatformBridgeException {
  const InvalidArgumentException(String message, [dynamic details])
    : super('INVALID_ARGUMENT', message, details);
}

final class MethodNotFoundException extends PlatformBridgeException {
  const MethodNotFoundException(String message, [dynamic details])
    : super('NOT_FOUND', message, details);
}

final class ResourceInUseException extends PlatformBridgeException {
  const ResourceInUseException(String message, [dynamic details])
    : super('ALREADY_IN_USE', message, details);
}

final class ChannelTimeoutException extends PlatformBridgeException {
  const ChannelTimeoutException(String message, [dynamic details])
    : super('TIMEOUT', message, details);
}

final class CancelledException extends PlatformBridgeException {
  const CancelledException(String message, [dynamic details])
    : super('CANCELLED', message, details);
}

final class UnknownPlatformException extends PlatformBridgeException {
  const UnknownPlatformException(super.code, super.message, [super.details]);
}
