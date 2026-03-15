import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';

import 'generated/bindings.dart';

class NativeEvent {
  final int id;
  final List<dynamic> data;

  const NativeEvent(this.id, this.data);

  @override
  String toString() => 'NativeEvent($id, $data)';
}

class NativeEventReceiver {
  ReceivePort? _receivePort;
  StreamController<NativeEvent>? _controller;
  StreamSubscription<dynamic>? _portSub;
  static bool _dartApiInitialized = false;
  bool _disposed = false;

  Stream<NativeEvent> get events {
    if (_disposed) {
      throw StateError('NativeEventReceiver already disposed');
    }
    _ensureStarted();
    return _controller!.stream;
  }

  void _ensureStarted() {
    if (_controller != null) return;

    if (!_dartApiInitialized) {
      app_init_dart_api(NativeApi.initializeApiDLData);
      _dartApiInitialized = true;
    }

    _controller = StreamController<NativeEvent>.broadcast(
      onCancel: _onLastListenerGone,
    );

    _receivePort = ReceivePort();
    app_set_dart_port(_receivePort!.sendPort.nativePort);

    _portSub = _receivePort!.listen((message) {
      if (message is List && message.isNotEmpty) {
        var id = message[0] as int;
        var data = message.sublist(1);
        _controller?.add(NativeEvent(id, data));
      }
    });
  }

  void _onLastListenerGone() {
    _teardown();
  }

  void _teardown() {
    app_disconnect_dart_port();
    _portSub?.cancel();
    _portSub = null;
    _receivePort?.close();
    _receivePort = null;
    final ctrl = _controller;
    _controller = null;
    ctrl?.close();
  }

  Stream<NativeEvent> where(int eventId) =>
      events.where((e) => e.id == eventId);

  void dispose() {
    if (_disposed) return;
    _disposed = true;
    _teardown();
  }
}
