import 'dart:async';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:native_bridge/native_bridge.dart';
import 'package:proto_models/proto_models.dart';
import 'package:ui_kit/ui_kit.dart';

import 'home_provider.dart';

class HomeScreen extends ConsumerStatefulWidget {
  const HomeScreen({super.key});

  @override
  ConsumerState<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends ConsumerState<HomeScreen> {
  StreamSubscription<NativeEvent>? _eventSub;

  @override
  void dispose() {
    _eventSub?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final pingResult = ref.watch(pingResultProvider);
    final isLoading = ref.watch(isLoadingProvider);
    final lastEvent = ref.watch(lastEventProvider);
    final colors = Theme.of(context).colorScheme;
    final text = Theme.of(context).textTheme;

    return AppScaffold(
      title: 'Pattern H',
      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(Icons.hexagon_outlined, size: 80, color: colors.primary),
              const SizedBox(height: 24),
              Text(
                'Hexagonal Architecture',
                style: text.headlineMedium?.copyWith(color: colors.onSurface),
              ),
              const SizedBox(height: 8),
              Text(
                'Flutter UI + Rust Core + Protobuf FFI',
                style: text.bodyLarge?.copyWith(color: colors.onSurfaceVariant),
              ),
              const SizedBox(height: 48),
              AppButton(
                label: 'Ping Rust Core',
                loading: isLoading,
                onPressed: _onPing,
              ),
              if (pingResult != null) ...[
                const SizedBox(height: 24),
                Card(
                  color: colors.primaryContainer,
                  child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: Text(
                      pingResult,
                      style: text.bodyLarge?.copyWith(
                        color: colors.onPrimaryContainer,
                      ),
                    ),
                  ),
                ),
              ],
              const SizedBox(height: 24),
              AppButton(label: 'Send Test Event', onPressed: _onSendEvent),
              if (lastEvent != null) ...[
                const SizedBox(height: 16),
                Card(
                  color: colors.tertiaryContainer,
                  child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: Text(
                      lastEvent,
                      style: text.bodyMedium?.copyWith(
                        color: colors.onTertiaryContainer,
                      ),
                    ),
                  ),
                ),
              ],
            ],
          ),
        ),
      ),
    );
  }

  void _onPing() {
    final ffi = ref.read(ffiClientProvider);
    ref.read(isLoadingProvider.notifier).state = true;
    try {
      final request = PingRequest(message: 'hello');
      final responseBytes = ffi.ping(
        Uint8List.fromList(request.writeToBuffer()),
      );
      final response = PingResponse.fromBuffer(responseBytes);
      var result = response.message;
      if (response.hasTimestamp()) {
        result += ' (ts: ${response.timestamp.seconds})';
      }
      ref.read(pingResultProvider.notifier).state = result;
    } on FfiException catch (e) {
      ref.read(pingResultProvider.notifier).state = 'Error: $e';
    } finally {
      ref.read(isLoadingProvider.notifier).state = false;
    }
  }

  void _onSendEvent() {
    _eventSub ??= ref.read(eventReceiverProvider).events.listen((event) {
      ref.read(lastEventProvider.notifier).state = event.toString();
    });
    ref.read(ffiClientProvider).sendTestEvent();
  }
}
