class Question {
  final int id;
  final String text;
  final String? facet;
  final bool? reverse;
  final String? dichotomy;
  final String? pole;
  final int? type;
  final String? dimension;
  final String? trait;

  Question({
    required this.id,
    required this.text,
    this.facet,
    this.reverse,
    this.dichotomy,
    this.pole,
    this.type,
    this.dimension,
    this.trait,
  });

  factory Question.fromJson(Map<String, dynamic> json) {
    return Question(
      id: json['id'],
      text: json['text'],
      facet: json['facet'],
      reverse: json['reverse'],
      dichotomy: json['dichotomy'],
      pole: json['pole'],
      type: json['type'],
      dimension: json['dimension'],
      trait: json['trait'],
    );
  }
}

class TestData {
  final String testType;
  final String name;
  final String description;
  final List<Question> questions;
  final String testMode;
  final Map<String, dynamic>? scale;
  final Map<String, dynamic>? factors;
  final Map<String, dynamic>? traits;

  TestData({
    required this.testType,
    required this.name,
    required this.description,
    required this.questions,
    required this.testMode,
    this.scale,
    this.factors,
    this.traits,
  });

  factory TestData.fromJson(Map<String, dynamic> json) {
    return TestData(
      testType: json['test_type'],
      name: json['name'] ?? '',
      description: json['description'] ?? '',
      questions: (json['questions'] as List? ?? []).map((q) => Question.fromJson(q)).toList(),
      testMode: json['test_mode'] ?? 'questions',
      scale: json['scale'],
      factors: json['factors'],
      traits: json['traits'],
    );
  }
}
