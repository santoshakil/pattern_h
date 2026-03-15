import 'dart:convert';
import 'dart:ffi';
import 'dart:typed_data';

import 'package:ffi/ffi.dart';

import 'error.dart';
import 'generated/bindings.dart';

typedef ProtoFfiFn = FfiResult Function(Pointer<Uint8>, int);

class FfiClient {
  void init() => _checkResult(app_init());

  void shutdown() => _checkResult(app_shutdown());

  bool get isInitialized => app_is_initialized();

  int get version => app_version();

  Uint8List ping(Uint8List requestBytes) => callNative(app_ping, requestBytes);

  void sendTestEvent() {
    assert(() {
      app_send_test_event();
      return true;
    }());
  }

  Uint8List callNative(ProtoFfiFn fn, Uint8List requestBytes) {
    final ptr = _allocateAndCopy(requestBytes);
    try {
      var result = fn(ptr, requestBytes.length);
      try {
        return _extractResult(result);
      } finally {
        free_result(result);
      }
    } finally {
      if (ptr != nullptr) malloc.free(ptr);
    }
  }

  Uint8List _extractResult(FfiResult result) {
    if (!result.success) {
      var msg = utf8.decode(_bytesFromBuffer(result.data));
      throw FfiException.fromCode(result.error_code, msg);
    }
    return _bytesFromBuffer(result.data);
  }

  void _checkResult(FfiResult result) {
    try {
      if (!result.success) {
        var msg = utf8.decode(_bytesFromBuffer(result.data));
        throw FfiException.fromCode(result.error_code, msg);
      }
    } finally {
      free_result(result);
    }
  }

  Uint8List _bytesFromBuffer(ByteBuffer buf) {
    if (buf.ptr == nullptr || buf.len == 0) return Uint8List(0);
    return Uint8List.fromList(buf.ptr.asTypedList(buf.len));
  }

  Pointer<Uint8> _allocateAndCopy(Uint8List bytes) {
    if (bytes.isEmpty) return nullptr;
    final ptr = malloc<Uint8>(bytes.length);
    ptr.asTypedList(bytes.length).setAll(0, bytes);
    return ptr;
  }
}
