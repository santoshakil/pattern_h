import 'package:design_system/design_system.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('AppTheme', () {
    test('light returns ThemeData with useMaterial3', () {
      final t = AppTheme.light();
      expect(t, isA<ThemeData>());
      expect(t.useMaterial3, isTrue);
    });

    test('light uses light color scheme', () {
      final t = AppTheme.light();
      expect(t.colorScheme.brightness, Brightness.light);
    });

    test('dark returns ThemeData with useMaterial3', () {
      final t = AppTheme.dark();
      expect(t, isA<ThemeData>());
      expect(t.useMaterial3, isTrue);
    });

    test('dark uses dark color scheme', () {
      final t = AppTheme.dark();
      expect(t.colorScheme.brightness, Brightness.dark);
    });

    test('light includes text theme', () {
      final t = AppTheme.light();
      expect(t.textTheme.bodyMedium, isNotNull);
    });

    test('dark includes text theme', () {
      final t = AppTheme.dark();
      expect(t.textTheme.bodyMedium, isNotNull);
    });
  });

  group('AppColors', () {
    test('light returns light ColorScheme', () {
      final cs = AppColors.light();
      expect(cs, isA<ColorScheme>());
      expect(cs.brightness, Brightness.light);
    });

    test('dark returns dark ColorScheme', () {
      final cs = AppColors.dark();
      expect(cs, isA<ColorScheme>());
      expect(cs.brightness, Brightness.dark);
    });

    test('seed color is defined', () {
      expect(AppColors.seed, isA<Color>());
    });

    test('light and dark have different surface colors', () {
      final l = AppColors.light();
      final d = AppColors.dark();
      expect(l.surface, isNot(equals(d.surface)));
    });
  });

  group('AppTypography', () {
    late TextTheme tt;

    setUp(() {
      tt = AppTypography.textTheme();
    });

    test('returns TextTheme', () {
      expect(tt, isA<TextTheme>());
    });

    test('has all 13 text styles', () {
      expect(tt.displayLarge, isNotNull);
      expect(tt.displayMedium, isNotNull);
      expect(tt.displaySmall, isNotNull);
      expect(tt.headlineLarge, isNotNull);
      expect(tt.headlineMedium, isNotNull);
      expect(tt.headlineSmall, isNotNull);
      expect(tt.titleLarge, isNotNull);
      expect(tt.titleMedium, isNotNull);
      expect(tt.titleSmall, isNotNull);
      expect(tt.bodyLarge, isNotNull);
      expect(tt.bodyMedium, isNotNull);
      expect(tt.bodySmall, isNotNull);
      expect(tt.labelLarge, isNotNull);
      expect(tt.labelMedium, isNotNull);
      expect(tt.labelSmall, isNotNull);
    });

    test('displayLarge has bold weight', () {
      expect(tt.displayLarge!.fontWeight, FontWeight.w700);
    });

    test('displayLarge has negative letter spacing', () {
      expect(tt.displayLarge!.letterSpacing, -0.5);
    });
  });
}
