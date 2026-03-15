import 'package:scaffold/src/config.dart';
import 'package:test/test.dart';

void main() {
  group('ScaffoldConfig', () {
    test('valid snake_case name', () {
      final c = ScaffoldConfig(
        name: 'my_app',
        org: 'com.example',
        outputDir: '.',
        seedColor: '1A73E8',
      );
      expect(c.isValidName, isTrue);
    });

    test('rejects uppercase name', () {
      final c = ScaffoldConfig(
        name: 'MyApp',
        org: 'com.example',
        outputDir: '.',
        seedColor: '1A73E8',
      );
      expect(c.isValidName, isFalse);
    });

    test('rejects name starting with number', () {
      final c = ScaffoldConfig(
        name: '1app',
        org: 'com.example',
        outputDir: '.',
        seedColor: '1A73E8',
      );
      expect(c.isValidName, isFalse);
    });

    test('rejects hyphenated name', () {
      final c = ScaffoldConfig(
        name: 'my-app',
        org: 'com.example',
        outputDir: '.',
        seedColor: '1A73E8',
      );
      expect(c.isValidName, isFalse);
    });

    test('pascalCase conversion', () {
      final c = ScaffoldConfig(
        name: 'my_restaurant_app',
        org: 'com.example',
        outputDir: '.',
        seedColor: '1A73E8',
      );
      expect(c.pascalCase, 'MyRestaurantApp');
    });

    test('titleCase conversion', () {
      final c = ScaffoldConfig(
        name: 'my_app',
        org: 'com.example',
        outputDir: '.',
        seedColor: '1A73E8',
      );
      expect(c.titleCase, 'My App');
    });

    test('channelPrefix uses org and name', () {
      final c = ScaffoldConfig(
        name: 'kiosk',
        org: 'com.mycompany',
        outputDir: '.',
        seedColor: '1A73E8',
      );
      expect(c.channelPrefix, 'com.mycompany/kiosk');
    });

    test('includeGuard is uppercase', () {
      final c = ScaffoldConfig(
        name: 'my_app',
        org: 'com.example',
        outputDir: '.',
        seedColor: '1A73E8',
      );
      expect(c.includeGuard, 'MY_APP_CORE_H');
    });

    test('cdylibName appends _core', () {
      final c = ScaffoldConfig(
        name: 'pos_system',
        org: 'com.example',
        outputDir: '.',
        seedColor: '1A73E8',
      );
      expect(c.cdylibName, 'pos_system_core');
    });

    test('single word name', () {
      final c = ScaffoldConfig(
        name: 'kiosk',
        org: 'com.example',
        outputDir: '.',
        seedColor: '1A73E8',
      );
      expect(c.isValidName, isTrue);
      expect(c.pascalCase, 'Kiosk');
      expect(c.titleCase, 'Kiosk');
    });
  });
}
