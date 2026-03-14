import 'package:flutter/services.dart';

import 'error.dart';

class PlatformChannel {
  static final _cache = <String, PlatformChannel>{};

  final String name;
  final MethodChannel _method;
  final Map<String, Stream<dynamic>> _eventStreams = {};

  factory PlatformChannel(String name) =>
      _cache.putIfAbsent(name, () => PlatformChannel._(name));

  PlatformChannel._(this.name) : _method = MethodChannel(name);

  Future<T?> call<T>(String method, [dynamic args]) async {
    try {
      return await _method.invokeMethod<T>(method, args);
    } on PlatformException catch (e) {
      throw PlatformBridgeException.from(e);
    }
  }

  Future<List<T>?> callList<T>(String method, [dynamic args]) async {
    try {
      return await _method.invokeListMethod<T>(method, args);
    } on PlatformException catch (e) {
      throw PlatformBridgeException.from(e);
    }
  }

  Future<Map<K, V>?> callMap<K, V>(String method, [dynamic args]) async {
    try {
      return await _method.invokeMapMethod<K, V>(method, args);
    } on PlatformException catch (e) {
      throw PlatformBridgeException.from(e);
    }
  }

  Stream<dynamic> events([String? qualifier]) {
    final channelName = qualifier != null ? '$name/$qualifier' : '$name/events';
    return _eventStreams.putIfAbsent(
      channelName,
      () => EventChannel(channelName).receiveBroadcastStream(),
    );
  }

  void setHandler(Future<dynamic> Function(MethodCall call)? handler) {
    if (handler == null) {
      _method.setMethodCallHandler(null);
      return;
    }
    _method.setMethodCallHandler((call) async {
      try {
        return await handler(call);
      } on PlatformBridgeException {
        rethrow;
      } on PlatformException catch (e) {
        throw PlatformBridgeException.from(e);
      }
    });
  }

  static void clearCache() {
    for (final channel in _cache.values) {
      channel._method.setMethodCallHandler(null);
      channel._eventStreams.clear();
    }
    _cache.clear();
  }
}
