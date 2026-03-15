import 'dart:io';

import 'package:path/path.dart' as p;

import 'config.dart';

class ProjectGenerator {
  final ScaffoldConfig config;
  late final String _skeletonDir;
  late final String _outputDir;

  ProjectGenerator(this.config);

  void run() {
    _skeletonDir = _findSkeletonRoot();
    _outputDir = p.join(config.outputDir, config.name);

    if (Directory(_outputDir).existsSync()) {
      throw StateError('Directory already exists: $_outputDir');
    }

    stdout.writeln('Creating ${config.name} from Pattern H skeleton...');
    stdout.writeln('  Output: ${p.absolute(_outputDir)}');
    stdout.writeln('  Org: ${config.org}');
    stdout.writeln('  Seed color: 0x${config.seedColor}');
    stdout.writeln('');

    _copyTree();
    _replaceInAllFiles();
    _renameClaudeMd();
    _initGit();
    _printSuccess();
  }

  String _findSkeletonRoot() {
    var dir = p.dirname(p.dirname(Platform.script.toFilePath()));
    if (!File(p.join(dir, 'CLAUDE.md')).existsSync()) {
      dir = p.dirname(p.dirname(dir));
    }
    if (!File(p.join(dir, 'CLAUDE.md')).existsSync()) {
      throw StateError('Cannot find Pattern H skeleton root from $dir');
    }
    return dir;
  }

  void _copyTree() {
    stdout.writeln('Copying skeleton...');
    final src = Directory(_skeletonDir);
    for (final entity in src.listSync(recursive: true)) {
      final rel = p.relative(entity.path, from: _skeletonDir);

      if (_shouldSkip(rel)) continue;

      final dest = p.join(_outputDir, rel);

      if (entity is Directory) {
        Directory(dest).createSync(recursive: true);
      } else if (entity is File) {
        Directory(p.dirname(dest)).createSync(recursive: true);
        entity.copySync(dest);
      }
    }
  }

  bool _shouldSkip(String rel) {
    final parts = p.split(rel);
    if (parts.any((p) => p == '.git')) return true;
    if (parts.any((p) => p == 'target')) return true;
    if (parts.any((p) => p == '.dart_tool')) return true;
    if (parts.any((p) => p == 'build')) return true;
    if (parts.any((p) => p == 'generated')) return true;
    if (parts.any((p) => p == 'tools')) return true;
    if (parts.any((p) => p == '.github')) return true;
    if (rel == 'pubspec.lock') return true;
    if (rel.startsWith('flutter') && rel.endsWith('pubspec.lock')) return true;
    if (rel.startsWith('rust') && rel.contains('Cargo.lock')) return true;
    if (rel.startsWith('docs/analysis')) return true;
    return false;
  }

  void _replaceInAllFiles() {
    stdout.writeln('Replacing pattern_h references...');
    final dir = Directory(_outputDir);
    var count = 0;

    for (final entity in dir.listSync(recursive: true)) {
      if (entity is! File) continue;
      if (_isBinaryFile(entity.path)) continue;

      try {
        var content = entity.readAsStringSync();
        final original = content;

        content = _applyReplacements(content);

        if (content != original) {
          entity.writeAsStringSync(content);
          count++;
        }
      } catch (_) {}
    }

    stdout.writeln('  Updated $count files');
  }

  String _applyReplacements(String content) {
    content = content
        .replaceAll('pattern_h', config.name)
        .replaceAll('Pattern H', config.titleCase)
        .replaceAll('PatternH', config.pascalCase)
        .replaceAll('PATTERN_H', config.name.toUpperCase())
        .replaceAll('_pattern_h', '_${config.name}')
        .replaceAll('pattern-h', config.name.replaceAll('_', '-'))
        .replaceAll('com.pattern_h', config.channelPrefix)
        .replaceAll('APP_CORE_H', config.includeGuard)
        .replaceAll('app_core', config.cdylibName)
        .replaceAll('app-core', config.cdylibName.replaceAll('_', '-'));

    if (config.seedColor != '1A73E8') {
      content = content.replaceAll('0xFF1A73E8', '0xFF${config.seedColor}');
    }

    return content;
  }

  bool _isBinaryFile(String path) {
    final ext = p.extension(path).toLowerCase();
    return {'.lock', '.png', '.jpg', '.jpeg', '.gif', '.ico', '.ttf', '.otf'}
        .contains(ext);
  }

  void _renameClaudeMd() {
    final claudeMd = File(p.join(_outputDir, 'CLAUDE.md'));
    if (claudeMd.existsSync()) {
      var content = claudeMd.readAsStringSync();
      content = _applyReplacements(content);
      claudeMd.writeAsStringSync(content);
    }
  }

  void _initGit() {
    stdout.writeln('Initializing git...');
    Process.runSync('git', ['init'], workingDirectory: _outputDir);
    Process.runSync('git', ['add', '.'], workingDirectory: _outputDir);
    Process.runSync(
      'git',
      ['commit', '-m', 'Initial commit from Pattern H skeleton'],
      workingDirectory: _outputDir,
    );
  }

  void _printSuccess() {
    stdout.writeln('');
    stdout.writeln('Project ${config.name} created successfully!');
    stdout.writeln('');
    stdout.writeln('Next steps:');
    stdout.writeln('  cd ${config.name}');
    stdout.writeln('  ./scripts/setup.sh');
    stdout.writeln('  just check');
    stdout.writeln('');
  }
}
