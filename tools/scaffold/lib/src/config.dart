class ScaffoldConfig {
  final String name;
  final String org;
  final String outputDir;
  final String seedColor;

  ScaffoldConfig({
    required this.name,
    required this.org,
    required this.outputDir,
    required this.seedColor,
  });

  bool get isValidName => RegExp(r'^[a-z][a-z0-9_]*$').hasMatch(name);

  String get pascalCase {
    return name.split('_').map((w) {
      if (w.isEmpty) return '';
      return w[0].toUpperCase() + w.substring(1);
    }).join();
  }

  String get titleCase => pascalCase.replaceAllMapped(
        RegExp(r'([A-Z])'),
        (m) => ' ${m.group(1)}',
      ).trim();

  String get channelPrefix => '$org/$name';

  String get includeGuard => '${name.toUpperCase()}_CORE_H';

  String get cdylibName => '${name}_core';

  String get dartAssetId => 'package:native_bridge/native_bridge.dart';
}
