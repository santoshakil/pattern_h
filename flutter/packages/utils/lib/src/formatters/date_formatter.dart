abstract final class DateFormatter {
  static String ymd(DateTime dt) =>
      '${dt.year}-${_pad(dt.month)}-${_pad(dt.day)}';

  static String hm(DateTime dt) => '${_pad(dt.hour)}:${_pad(dt.minute)}';

  static String ymdHm(DateTime dt) => '${ymd(dt)} ${hm(dt)}';

  static String relative(DateTime dt) {
    final diff = DateTime.now().difference(dt);
    if (diff.inSeconds < 60) return 'just now';
    if (diff.inMinutes < 60) return '${diff.inMinutes}m ago';
    if (diff.inHours < 24) return '${diff.inHours}h ago';
    if (diff.inDays < 7) return '${diff.inDays}d ago';
    return ymd(dt);
  }

  static String _pad(int n) => n.toString().padLeft(2, '0');
}
