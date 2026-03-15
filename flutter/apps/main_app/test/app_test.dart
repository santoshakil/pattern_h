import 'dart:async';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:main_app/app.dart';
import 'package:main_app/features/home/home_provider.dart';
import 'package:native_bridge/native_bridge.dart';

class MockFfiClient implements FfiClient {
  bool _initialized = false;

  @override
  void init() => _initialized = true;

  @override
  void shutdown() => _initialized = false;

  @override
  bool get isInitialized => _initialized;

  @override
  int get version => 1;

  @override
  Uint8List ping(Uint8List requestBytes) => Uint8List(0);

  @override
  void sendTestEvent() {}

  @override
  Uint8List callNative(ProtoFfiFn fn, Uint8List requestBytes) => Uint8List(0);
}

class MockEventReceiver implements NativeEventReceiver {
  final _controller = StreamController<NativeEvent>.broadcast();

  @override
  Stream<NativeEvent> get events => _controller.stream;

  @override
  Stream<NativeEvent> where(int eventId) =>
      events.where((e) => e.id == eventId);

  @override
  void dispose() => _controller.close();
}

void main() {
  testWidgets('app renders home screen', (tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          ffiClientProvider.overrideWithValue(MockFfiClient()),
          eventReceiverProvider.overrideWithValue(MockEventReceiver()),
        ],
        child: const App(),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Pattern H'), findsOneWidget);
    expect(find.text('Flutter + Rust + Platform Test'), findsOneWidget);
  });

  testWidgets('app uses correct theme', (tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          ffiClientProvider.overrideWithValue(MockFfiClient()),
          eventReceiverProvider.overrideWithValue(MockEventReceiver()),
        ],
        child: const App(),
      ),
    );
    await tester.pumpAndSettle();

    final materialApp = tester.widget<MaterialApp>(find.byType(MaterialApp));
    expect(materialApp.theme?.useMaterial3, isTrue);
    expect(materialApp.darkTheme, isNotNull);
    expect(materialApp.themeMode, ThemeMode.dark);
  });
}
