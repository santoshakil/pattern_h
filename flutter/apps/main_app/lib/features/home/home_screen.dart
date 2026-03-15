import 'dart:async';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:native_bridge/native_bridge.dart';
import 'package:proto_models/proto_models.dart';

import 'home_provider.dart';

class HomeScreen extends ConsumerStatefulWidget {
  const HomeScreen({super.key});

  @override
  ConsumerState<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends ConsumerState<HomeScreen> {
  final _log = <String>[];
  StreamSubscription<NativeEvent>? _eventSub;

  @override
  void initState() {
    super.initState();
    _runTests();
  }

  @override
  void dispose() {
    _eventSub?.cancel();
    super.dispose();
  }

  Future<void> _runTests() async {
    _addLog('Starting tests...');

    try {
      final ffi = ref.read(ffiClientProvider);
      _addLog('FFI init: OK');
      _addLog('Version: ${ffi.version}');

      final req = PingRequest(message: 'hello from Flutter');
      final resBytes = ffi.ping(Uint8List.fromList(req.writeToBuffer()));
      final res = PingResponse.fromBuffer(resBytes);
      _addLog('Ping: ${res.message}');
      if (res.hasTimestamp()) {
        _addLog('Timestamp: ${res.timestamp.seconds}');
      }
    } on FfiException catch (e) {
      _addLog('FFI Error: $e');
    } catch (e) {
      _addLog('Error: $e');
    }

    try {
      final receiver = ref.read(eventReceiverProvider);
      _eventSub = receiver.events.listen((event) {
        _addLog('Event received: $event');
      });
      ref.read(ffiClientProvider).sendTestEvent();
      _addLog('Test event sent');
    } catch (e) {
      _addLog('Event error: $e');
    }
  }

  void _addLog(String msg) {
    setState(() => _log.add(msg));
  }

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final text = Theme.of(context).textTheme;

    return Scaffold(
      appBar: AppBar(title: const Text('Pattern H')),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Flutter + Rust FFI Test',
              style: text.titleLarge?.copyWith(color: colors.primary),
            ),
            const SizedBox(height: 16),
            Expanded(
              child: ListView.builder(
                itemCount: _log.length,
                itemBuilder: (_, i) => Padding(
                  padding: const EdgeInsets.symmetric(vertical: 2),
                  child: Text(_log[i], style: text.bodyMedium),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
