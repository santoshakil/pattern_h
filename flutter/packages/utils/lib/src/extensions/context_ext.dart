import 'package:flutter/material.dart';

extension ContextExt on BuildContext {
  ThemeData get theme => Theme.of(this);
  ColorScheme get colors => theme.colorScheme;
  TextTheme get textTheme => theme.textTheme;
  MediaQueryData get mq => MediaQuery.of(this);
  double get screenWidth => mq.size.width;
  double get screenHeight => mq.size.height;
  bool get isLandscape => mq.orientation == Orientation.landscape;
  EdgeInsets get viewPadding => mq.viewPadding;

  void showSnack(String msg) =>
      ScaffoldMessenger.of(this).showSnackBar(SnackBar(content: Text(msg)));
}
