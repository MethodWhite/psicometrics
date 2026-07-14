class ApiConfig {
  static String baseUrl = const String.fromEnvironment(
    'API_URL',
    defaultValue: 'http://localhost:8000',
  );

  static const Duration timeout = Duration(seconds: 30);
}
