class BigFiveResult {
  final Map<String, double> scores;
  final Map<String, double> facets;
  final String profileSummary;
  final Map<String, double> percentiles;

  final Map<String, dynamic> raw;

  BigFiveResult({
    required this.scores,
    required this.facets,
    required this.profileSummary,
    required this.percentiles,
    required this.raw,
  });

  factory BigFiveResult.fromJson(Map<String, dynamic> json) {
    return BigFiveResult(
      scores: Map<String, double>.from(json['scores'] ?? {}),
      facets: Map<String, double>.from(json['facets'] ?? {}),
      profileSummary: json['profile_summary'] ?? '',
      percentiles: Map<String, double>.from(json['percentiles'] ?? {}),
      raw: json,
    );
  }

  Map<String, dynamic> toJson() => raw;
}

class MBTIResult {
  final String typeCode;
  final Map<String, double> scores;
  final String profileSummary;
  final Map<String, double> percentages;
  final Map<String, dynamic> raw;

  MBTIResult({
    required this.typeCode,
    required this.scores,
    required this.profileSummary,
    required this.percentages,
    required this.raw,
  });

  factory MBTIResult.fromJson(Map<String, dynamic> json) {
    return MBTIResult(
      typeCode: json['type_code'] ?? '',
      scores: Map<String, double>.from(json['scores'] ?? {}),
      profileSummary: json['profile_summary'] ?? '',
      percentages: Map<String, double>.from(json['percentages'] ?? {}),
      raw: json,
    );
  }

  Map<String, dynamic> toJson() => raw;
}

class EnneagramResult {
  final int dominantType;
  final int wing;
  final Map<String, double> scores;
  final String profileSummary;
  final Map<String, dynamic> raw;

  EnneagramResult({
    required this.dominantType,
    required this.wing,
    required this.scores,
    required this.profileSummary,
    required this.raw,
  });

  factory EnneagramResult.fromJson(Map<String, dynamic> json) {
    return EnneagramResult(
      dominantType: json['dominant_type'] ?? 1,
      wing: json['wing'] ?? 1,
      scores: Map<String, double>.from(json['scores'] ?? {}),
      profileSummary: json['profile_summary'] ?? '',
      raw: json,
    );
  }

  Map<String, dynamic> toJson() => raw;
}

class DISCResult {
  final String primaryStyle;
  final String secondaryStyle;
  final Map<String, double> scores;
  final String profileSummary;
  final Map<String, dynamic> raw;

  DISCResult({
    required this.primaryStyle,
    required this.secondaryStyle,
    required this.scores,
    required this.profileSummary,
    required this.raw,
  });

  factory DISCResult.fromJson(Map<String, dynamic> json) {
    return DISCResult(
      primaryStyle: json['primary_style'] ?? '',
      secondaryStyle: json['secondary_style'] ?? '',
      scores: Map<String, double>.from(json['scores'] ?? {}),
      profileSummary: json['profile_summary'] ?? '',
      raw: json,
    );
  }

  Map<String, dynamic> toJson() => raw;
}

class DarkTriadResult {
  final Map<String, double> scores;
  final double darkCore;
  final String riskLevel;
  final String profileSummary;
  final Map<String, dynamic> raw;

  DarkTriadResult({
    required this.scores,
    required this.darkCore,
    required this.riskLevel,
    required this.profileSummary,
    required this.raw,
  });

  factory DarkTriadResult.fromJson(Map<String, dynamic> json) {
    return DarkTriadResult(
      scores: Map<String, double>.from(json['scores'] ?? {}),
      darkCore: (json['dark_core'] ?? 0).toDouble(),
      riskLevel: json['risk_level'] ?? 'minimal',
      profileSummary: json['profile_summary'] ?? '',
      raw: json,
    );
  }

  Map<String, dynamic> toJson() => raw;
}

class HumanDesignResult {
  final String type;
  final Map<String, dynamic> typeInfo;
  final String strategy;
  final String authority;
  final Map<String, dynamic> authorityInfo;
  final String profile;
  final Map<String, dynamic> profileInfo;
  final Map<String, bool> centers;
  final List<int> personalityGates;
  final List<int> designGates;
  final String summary;
  final Map<String, dynamic> raw;

  HumanDesignResult({
    required this.type,
    required this.typeInfo,
    required this.strategy,
    required this.authority,
    required this.authorityInfo,
    required this.profile,
    required this.profileInfo,
    required this.centers,
    required this.personalityGates,
    required this.designGates,
    required this.summary,
    required this.raw,
  });

  factory HumanDesignResult.fromJson(Map<String, dynamic> json) {
    return HumanDesignResult(
      type: json['type'] ?? '',
      typeInfo: json['type_info'] ?? {},
      strategy: json['strategy'] ?? '',
      authority: json['authority'] ?? '',
      authorityInfo: json['authority_info'] ?? {},
      profile: json['profile'] ?? '',
      profileInfo: json['profile_info'] ?? {},
      centers: Map<String, bool>.from(json['centers'] ?? {}),
      personalityGates: List<int>.from(json['personality_gates'] ?? []),
      designGates: List<int>.from(json['design_gates'] ?? []),
      summary: json['summary'] ?? '',
      raw: json,
    );
  }

  Map<String, dynamic> toJson() => raw;
}

class ComparisonResult {
  final String testType;
  final double compatibilityScore;
  final String description;
  final Map<String, dynamic> factors;
  final String? typeCode1;
  final String? typeCode2;
  final String? matchCategory;

  ComparisonResult({
    required this.testType,
    required this.compatibilityScore,
    required this.description,
    required this.factors,
    this.typeCode1,
    this.typeCode2,
    this.matchCategory,
  });

  factory ComparisonResult.fromJson(Map<String, dynamic> json) {
    return ComparisonResult(
      testType: json['test_type'] ?? '',
      compatibilityScore: (json['compatibility_score'] ?? 0).toDouble(),
      description: json['description'] ?? '',
      factors: Map<String, dynamic>.from(json['factors'] ?? {}),
      typeCode1: json['type_code1']?.toString(),
      typeCode2: json['type_code2']?.toString(),
      matchCategory: json['match_category']?.toString(),
    );
  }
}
