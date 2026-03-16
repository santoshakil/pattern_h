extension StringExt on String {
  String get capitalized =>
      isEmpty ? this : '${this[0].toUpperCase()}${substring(1)}';

  String get titleCase =>
      split(' ').map((w) => w.capitalized).join(' ');

  String truncate(int max, [String suffix = '...']) =>
      length <= max ? this : '${substring(0, max)}$suffix';

  bool get isBlank => trim().isEmpty;

  bool get isNotBlank => !isBlank;
}
