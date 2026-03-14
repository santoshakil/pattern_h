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
  static bool _dartApiInitialized = false;

  Stream<NativeEvent> get events {
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

    _receivePort!.listen((message) {
      if (message is List && message.isNotEmpty) {
        var id = message[0] as int;
        var data = message.sublist(1);
        _controller?.add(NativeEvent(id, data));
      }
    });
  }

  void _onLastListenerGone() {
    app_disconnect_dart_port();
    _receivePort?.close();
    _receivePort = null;
    _controller?.close();
    _controller = null;
  }

  Stream<NativeEvent> where(int eventId) =>
      events.where((e) => e.id == eventId);

  void dispose() {
    app_disconnect_dart_port();
    _receivePort?.close();
    _receivePort = null;
    _controller?.close();
    _controller = null;
  }
}
