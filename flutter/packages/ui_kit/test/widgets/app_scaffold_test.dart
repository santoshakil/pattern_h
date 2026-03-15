import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:ui_kit/ui_kit.dart';

Widget _wrap(Widget child) => MaterialApp(home: child);

void main() {
  group('AppScaffold', () {
    testWidgets('renders title in AppBar', (t) async {
      await t.pumpWidget(
        _wrap(AppScaffold(title: 'Home', body: const SizedBox())),
      );
      expect(find.text('Home'), findsOneWidget);
      expect(find.byType(AppBar), findsOneWidget);
    });

    testWidgets('renders body content', (t) async {
      await t.pumpWidget(
        _wrap(
          AppScaffold(title: 'T', body: const Text('content')),
        ),
      );
      expect(find.text('content'), findsOneWidget);
    });

    testWidgets('renders FAB when provided', (t) async {
      await t.pumpWidget(
        _wrap(
          AppScaffold(
            title: 'T',
            body: const SizedBox(),
            floatingActionButton: FloatingActionButton(
              onPressed: () {},
              child: const Icon(Icons.add),
            ),
          ),
        ),
      );
      expect(find.byType(FloatingActionButton), findsOneWidget);
    });

    testWidgets('no FAB when not provided', (t) async {
      await t.pumpWidget(
        _wrap(AppScaffold(title: 'T', body: const SizedBox())),
      );
      expect(find.byType(FloatingActionButton), findsNothing);
    });

    testWidgets('renders actions when provided', (t) async {
      await t.pumpWidget(
        _wrap(
          AppScaffold(
            title: 'T',
            body: const SizedBox(),
            actions: [
              IconButton(
                onPressed: () {},
                icon: const Icon(Icons.settings),
              ),
            ],
          ),
        ),
      );
      expect(find.byIcon(Icons.settings), findsOneWidget);
    });

    testWidgets('no actions when not provided', (t) async {
      await t.pumpWidget(
        _wrap(AppScaffold(title: 'T', body: const SizedBox())),
      );
      expect(find.byType(IconButton), findsNothing);
    });

    testWidgets('wraps body in SafeArea', (t) async {
      await t.pumpWidget(
        _wrap(AppScaffold(title: 'T', body: const Text('safe'))),
      );
      expect(find.byType(SafeArea), findsWidgets);
      final scaffold = t.widget<Scaffold>(find.byType(Scaffold));
      expect(scaffold.body, isA<SafeArea>());
    });
  });
}
