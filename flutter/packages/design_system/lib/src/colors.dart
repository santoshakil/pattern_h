import 'package:flutter/material.dart';

class AppColors {
  AppColors._();

  static const seed = Color(0xFF1A73E8);

  static ColorScheme light() =>
      ColorScheme.fromSeed(seedColor: seed, brightness: Brightness.light);

  static ColorScheme dark() =>
      ColorScheme.fromSeed(seedColor: seed, brightness: Brightness.dark);
}
