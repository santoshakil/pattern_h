import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ui_kit/ui_kit.dart';

Widget _wrap(Widget child) => MaterialApp(home: Scaffold(body: child));

void main() {
  group('AppButton', () {
    testWidgets('renders label text', (t) async {
      await t.pumpWidget(_wrap(AppButton(label: 'Save', onPressed: () {})));
      expect(find.text('Save'), findsOneWidget);
    });

    testWidgets('calls onPressed when tapped', (t) async {
      var called = false;
      await t.pumpWidget(
        _wrap(AppButton(label: 'Tap', onPressed: () => called = true)),
      );
      await t.tap(find.text('Tap'));
      expect(called, isTrue);
    });

    testWidgets('disables button when loading', (t) async {
      var called = false;
      await t.pumpWidget(
        _wrap(
          AppButton(
            label: 'Load',
            onPressed: () => called = true,
            loading: true,
          ),
        ),
      );
      await t.tap(find.byType(ElevatedButton));
      expect(called, isFalse);
    });

    testWidgets('shows CircularProgressIndicator when loading', (t) async {
      await t.pumpWidget(
        _wrap(AppButton(label: 'Load', onPressed: () {}, loading: true)),
      );
      expect(find.byType(CircularProgressIndicator), findsOneWidget);
      expect(find.text('Load'), findsNothing);
    });

    testWidgets('hides label when loading', (t) async {
      await t.pumpWidget(
        _wrap(AppButton(label: 'Go', onPressed: () {}, loading: true)),
      );
      expect(find.text('Go'), findsNothing);
    });

    testWidgets('shows label when not loading', (t) async {
      await t.pumpWidget(
        _wrap(AppButton(label: 'Go', onPressed: () {}, loading: false)),
      );
      expect(find.text('Go'), findsOneWidget);
      expect(find.byType(CircularProgressIndicator), findsNothing);
    });

    testWidgets('uses theme colors not hardcoded', (t) async {
      final theme = ThemeData(
        useMaterial3: true,
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.red,
          brightness: Brightness.light,
        ),
      );
      await t.pumpWidget(
        MaterialApp(
          theme: theme,
          home: Scaffold(
            body: AppButton(label: 'X', onPressed: () {}),
          ),
        ),
      );
      final btn = t.widget<ElevatedButton>(find.byType(ElevatedButton));
      final style = btn.style!;
      final bg = style.backgroundColor!.resolve({});
      expect(bg, equals(theme.colorScheme.primary));
    });
  });
}
