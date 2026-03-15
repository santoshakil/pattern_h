import 'dart:io';

import 'package:args/args.dart';
import 'package:scaffold/src/generator.dart';
import 'package:scaffold/src/config.dart';

void main(List<String> args) {
  final parser = ArgParser()
    ..addOption('name', abbr: 'n', help: 'Project name (snake_case)')
    ..addOption('org', abbr: 'o', help: 'Organization domain', defaultsTo: 'com.example')
    ..addOption('output', abbr: 'd', help: 'Output directory', defaultsTo: '.')
    ..addOption('seed-color', help: 'Seed color hex (e.g. FF1A73E8)', defaultsTo: '1A73E8')
    ..addFlag('help', abbr: 'h', negatable: false);

  final results = parser.parse(args);

  if (results.flag('help') || results.rest.isEmpty && results.option('name') == null) {
    stderr.writeln('Usage: dart run scaffold <project_name> [options]');
    stderr.writeln('');
    stderr.writeln('Creates a new Flutter+Rust hexagonal architecture project');
    stderr.writeln('from the Pattern H skeleton.');
    stderr.writeln('');
    stderr.writeln(parser.usage);
    stderr.writeln('');
    stderr.writeln('Examples:');
    stderr.writeln('  dart run scaffold my_restaurant_app');
    stderr.writeln('  dart run scaffold my_app --org com.mycompany --seed-color FF6B35');
    exit(results.flag('help') ? 0 : 1);
  }

  final name = results.option('name') ?? results.rest.first;
  final org = results.option('org')!;
  final output = results.option('output')!;
  final seedColor = results.option('seed-color')!;

  final config = ScaffoldConfig(
    name: name,
    org: org,
    outputDir: output,
    seedColor: seedColor,
  );

  if (!config.isValidName) {
    stderr.writeln('Error: project name must be lowercase snake_case (a-z, 0-9, _)');
    exit(1);
  }

  try {
    final generator = ProjectGenerator(config);
    generator.run();
  } catch (e) {
    stderr.writeln('Error: $e');
    exit(1);
  }
}
