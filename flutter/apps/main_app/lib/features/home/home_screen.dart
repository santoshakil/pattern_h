import 'dart:async';
import 'dart:typed_data';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:native_bridge/native_bridge.dart';
import '../../platform/device_info.dart';
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
    WidgetsBinding.instance.addPostFrameCallback((_) => _runTests());
  }

  @override
  void dispose() {
    _eventSub?.cancel();
    super.dispose();
  }

  Future<void> _runTests() async {
    _addLog('=== FFI Tests ===');
    _testFfiInit();
    _testPing();
    _testEvent();
    _addLog('');
    _addLog('=== Method Channel Tests ===');
    await _testMethodChannel();
    _addLog('');
    _addLog('=== All Tests Complete ===');
  }

  void _testFfiInit() {
    try {
      final ffi = ref.read(ffiClientProvider);
      _addLog('PASS: FFI init OK (v${ffi.version})');
    } catch (e) {
      _addLog('FAIL: FFI init - $e');
    }
  }

  void _testPing() {
    try {
      final ffi = ref.read(ffiClientProvider);
      final req = PingRequest(message: 'hello from Flutter');
      final resBytes = ffi.ping(Uint8List.fromList(req.writeToBuffer()));
      final res = PingResponse.fromBuffer(resBytes);
      _addLog('PASS: Ping - ${res.message}');
    } catch (e) {
      _addLog('FAIL: Ping - $e');
    }
  }

  void _testEvent() {
    try {
      final receiver = ref.read(eventReceiverProvider);
      _eventSub = receiver.events.listen((event) {
        _addLog('PASS: Event received - $event');
      });
      ref.read(ffiClientProvider).sendTestEvent();
      _addLog('PASS: Test event sent');
    } catch (e) {
      _addLog('FAIL: Event - $e');
    }
  }

  Future<void> _testMethodChannel() async {
    try {
      final info = await DeviceInfo.get();
      if (info.isEmpty) {
        _addLog('FAIL: Method channel - empty response');
        return;
      }
      _addLog('PASS: Method channel - getDeviceInfo');
      for (final entry in info.entries) {
        _addLog('  ${entry.key}: ${entry.value}');
      }
    } catch (e) {
      _addLog('FAIL: Method channel - $e');
    }
  }

  void _addLog(String msg) {
    debugPrint('PATTERN_H: $msg');
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
              'Flutter + Rust + Platform Test',
              style: text.titleLarge?.copyWith(color: colors.primary),
            ),
            const SizedBox(height: 16),
            Expanded(
              child: ListView.builder(
                itemCount: _log.length,
                itemBuilder: (_, i) {
                  final line = _log[i];
                  final color = line.startsWith('PASS')
                      ? Colors.green
                      : line.startsWith('FAIL')
                          ? Colors.red
                          : colors.onSurface;
                  return Padding(
                    padding: const EdgeInsets.symmetric(vertical: 2),
                    child: Text(
                      line,
                      style: text.bodyMedium?.copyWith(color: color),
                    ),
                  );
                },
              ),
            ),
          ],
        ),
      ),
    );
  }
}
