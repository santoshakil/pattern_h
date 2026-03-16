abstract final class InputValidator {
  static final _emailRegex = RegExp(r'^[\w\-.+]+@[\w\-]+\.[\w\-.]+$');
  static final _phoneRegex = RegExp(r'^\+?[\d\s\-()]{7,15}$');

  static String? required(String? value, [String field = 'Field']) =>
      (value == null || value.trim().isEmpty) ? '$field is required' : null;

  static String? email(String? value) {
    if (value == null || value.isEmpty) return 'Email is required';
    return _emailRegex.hasMatch(value) ? null : 'Invalid email';
  }

  static String? phone(String? value) {
    if (value == null || value.isEmpty) return null;
    return _phoneRegex.hasMatch(value) ? null : 'Invalid phone number';
  }

  static String? minLength(String? value, int min, [String field = 'Field']) {
    if (value == null || value.length < min) {
      return '$field must be at least $min characters';
    }
    return null;
  }

  static String? maxLength(String? value, int max, [String field = 'Field']) {
    if (value != null && value.length > max) {
      return '$field must be at most $max characters';
    }
    return null;
  }
}
