import 'package:flutter/material.dart';

class AppTypography {
  AppTypography._();

  static TextTheme textTheme() => const TextTheme(
        displayLarge:
            TextStyle(fontWeight: FontWeight.w700, letterSpacing: -0.5),
        displayMedium: TextStyle(fontWeight: FontWeight.w600),
        displaySmall: TextStyle(fontWeight: FontWeight.w600),
        headlineLarge: TextStyle(fontWeight: FontWeight.w600),
        headlineMedium: TextStyle(fontWeight: FontWeight.w600),
        headlineSmall: TextStyle(fontWeight: FontWeight.w500),
        titleLarge: TextStyle(fontWeight: FontWeight.w500, letterSpacing: 0.15),
        titleMedium: TextStyle(fontWeight: FontWeight.w500, letterSpacing: 0.1),
        titleSmall: TextStyle(fontWeight: FontWeight.w500, letterSpacing: 0.1),
        bodyLarge: TextStyle(letterSpacing: 0.15),
        bodyMedium: TextStyle(letterSpacing: 0.25),
        bodySmall: TextStyle(letterSpacing: 0.4),
        labelLarge: TextStyle(fontWeight: FontWeight.w500, letterSpacing: 0.1),
        labelMedium: TextStyle(fontWeight: FontWeight.w500, letterSpacing: 0.5),
        labelSmall: TextStyle(fontWeight: FontWeight.w500, letterSpacing: 0.5),
      );
}
