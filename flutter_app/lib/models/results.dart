class BigFiveResult {
  final Map<String, double> scores;
  final Map<String, double> facets;
  final String profileSummary;
  final Map<String, double> percentiles;

  BigFiveResult({
    required this.scores,
    required this.facets,
    required this.profileSummary,
    required this.percentiles,
  });

  factory BigFiveResult.fromJson(Map<String, dynamic> json) {
    return BigFiveResult(
      scores: Map<String, double>.from(json['scores'] ?? {}),
      facets: Map<String, double>.from(json['facets'] ?? {}),
      profileSummary: json['profile_summary'] ?? '',
      percentiles: Map<String, double>.from(json['percentiles'] ?? {}),
    );
  }
}

class MBTIResult {
  final String typeCode;
  final Map<String, double> scores;
  final String profileSummary;
  final Map<String, double> percentages;

  MBTIResult({
    required this.typeCode,
    required this.scores,
    required this.profileSummary,
    required this.percentages,
  });

  factory MBTIResult.fromJson(Map<String, dynamic> json) {
    return MBTIResult(
      typeCode: json['type_code'] ?? '',
      scores: Map<String, double>.from(json['scores'] ?? {}),
      profileSummary: json['profile_summary'] ?? '',
      percentages: Map<String, double>.from(json['percentages'] ?? {}),
    );
  }
}

class EnneagramResult {
  final int dominantType;
  final int wing;
  final Map<String, double> scores;
  final String profileSummary;

  EnneagramResult({
    required this.dominantType,
    required this.wing,
    required this.scores,
    required this.profileSummary,
  });

  factory EnneagramResult.fromJson(Map<String, dynamic> json) {
    return EnneagramResult(
      dominantType: json['dominant_type'] ?? 1,
      wing: json['wing'] ?? 1,
      scores: Map<String, double>.from(json['scores'] ?? {}),
      profileSummary: json['profile_summary'] ?? '',
    );
  }
}

class DISCResult {
  final String primaryStyle;
  final String secondaryStyle;
  final Map<String, double> scores;
  final String profileSummary;

  DISCResult({
    required this.primaryStyle,
    required this.secondaryStyle,
    required this.scores,
    required this.profileSummary,
  });

  factory DISCResult.fromJson(Map<String, dynamic> json) {
    return DISCResult(
      primaryStyle: json['primary_style'] ?? '',
      secondaryStyle: json['secondary_style'] ?? '',
      scores: Map<String, double>.from(json['scores'] ?? {}),
      profileSummary: json['profile_summary'] ?? '',
    );
  }
}

class DarkTriadResult {
  final Map<String, double> scores;
  final double darkCore;
  final String riskLevel;
  final String profileSummary;

  DarkTriadResult({
    required this.scores,
    required this.darkCore,
    required this.riskLevel,
    required this.profileSummary,
  });

  factory DarkTriadResult.fromJson(Map<String, dynamic> json) {
    return DarkTriadResult(
      scores: Map<String, double>.from(json['scores'] ?? {}),
      darkCore: (json['dark_core'] ?? 0).toDouble(),
      riskLevel: json['risk_level'] ?? 'minimal',
      profileSummary: json['profile_summary'] ?? '',
    );
  }
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
    );
  }
}
