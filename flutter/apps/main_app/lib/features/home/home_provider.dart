import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:native_bridge/native_bridge.dart';

final ffiClientProvider = Provider<FfiClient>((ref) {
  final client = FfiClient()..init();
  ref.onDispose(client.shutdown);
  return client;
});

final eventReceiverProvider = Provider<NativeEventReceiver>((ref) {
  final receiver = NativeEventReceiver();
  ref.onDispose(receiver.dispose);
  return receiver;
});

final pingResultProvider = StateProvider<String?>((ref) => null);

final isLoadingProvider = StateProvider<bool>((ref) => false);

final lastEventProvider = StateProvider<String?>((ref) => null);
