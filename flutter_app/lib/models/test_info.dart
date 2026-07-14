class TestInfo {
  final String testType;
  final String name;
  final String description;
  final int itemCount;
  final String testMode;

  TestInfo({
    required this.testType,
    required this.name,
    required this.description,
    required this.itemCount,
    required this.testMode,
  });

  factory TestInfo.fromJson(Map<String, dynamic> json) {
    return TestInfo(
      testType: json['test_type'],
      name: json['name'] is Map ? json['name']['es'] ?? json['name']['en'] : json['name'],
      description: json['description'] is Map ? json['description']['es'] ?? json['description']['en'] : json['description'],
      itemCount: json['item_count'] ?? 0,
      testMode: json['test_mode'] ?? 'questions',
    );
  }
}
