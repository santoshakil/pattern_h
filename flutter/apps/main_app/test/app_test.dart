import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:main_app/app.dart';

void main() {
  testWidgets('app renders home screen', (tester) async {
    await tester.pumpWidget(const ProviderScope(child: App()));
    await tester.pumpAndSettle();

    expect(find.text('Pattern H'), findsOneWidget);
    expect(find.text('Hexagonal Architecture'), findsOneWidget);
    expect(find.text('Ping Rust Core'), findsOneWidget);
    expect(find.text('Send Test Event'), findsOneWidget);
  });
}
