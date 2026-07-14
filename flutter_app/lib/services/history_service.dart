import 'dart:convert';
import 'package:shared_preferences/shared_preferences.dart';

class TestHistory {
  static const String _key = 'test_history';

  static Future<void> saveResult({
    required String testType,
    required String testName,
    required dynamic result,
    required String lang,
  }) async {
    final prefs = await SharedPreferences.getInstance();
    final history = await getHistory();

    final entry = {
      'testType': testType,
      'testName': testName,
      'lang': lang,
      'timestamp': DateTime.now().toIso8601String(),
      'result': _resultToMap(result),
    };

    history.insert(0, entry);

    // Keep only last 50 results
    if (history.length > 50) {
      history.removeLast();
    }

    await prefs.setString(_key, jsonEncode(history));
  }

  static Future<List<Map<String, dynamic>>> getHistory() async {
    final prefs = await SharedPreferences.getInstance();
    final data = prefs.getString(_key);
    if (data == null) return [];
    final List decoded = jsonDecode(data);
    return decoded.cast<Map<String, dynamic>>();
  }

  static Future<void> clearHistory() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.remove(_key);
  }

  static Map<String, dynamic> _resultToMap(dynamic result) {
    if (result == null) return {};
    // Convert result object to map using toJson or manual conversion
    try {
      return result.toJson() as Map<String, dynamic>;
    } catch (e) {
      return {'raw': result.toString()};
    }
  }

  static String getTestDisplayName(String testType) {
    switch (testType) {
      case 'big_five': return 'Big Five / OCEAN';
      case 'mbti': return 'MBTI';
      case 'enneagram': return 'Enneagrama';
      case 'disc': return 'DISC';
      case 'dark_triad': return 'Triada Oscura';
      case 'human_design': return 'Diseño Humano';
      default: return testType;
    }
  }

  static String getTestIcon(String testType) {
    switch (testType) {
      case 'big_five': return '🧠';
      case 'mbti': return '🎭';
      case 'enneagram': return '🔵';
      case 'disc': return '📊';
      case 'dark_triad': return '🌑';
      case 'human_design': return '⭐';
      default: return '📋';
    }
  }

  static dynamic restoreResult(Map<String, dynamic> entry) {
    final testType = entry['testType'];
    final result = entry['result'];
    if (result == null) return null;

    switch (testType) {
      case 'big_five':
        return BigFiveResultData(
          scores: Map<String, double>.from(result['scores'] ?? {}),
          facets: Map<String, double>.from(result['facets'] ?? {}),
          profileSummary: result['profile_summary'] ?? '',
          percentiles: Map<String, double>.from(result['percentiles'] ?? {}),
        );
      case 'mbti':
        return MBTIResultData(
          typeCode: result['type_code'] ?? '',
          scores: Map<String, double>.from(result['scores'] ?? {}),
          profileSummary: result['profile_summary'] ?? '',
          percentages: Map<String, double>.from(result['percentages'] ?? {}),
        );
      case 'enneagram':
        return EnneagramResultData(
          dominantType: result['dominant_type'] ?? 1,
          wing: result['wing'] ?? 1,
          scores: Map<String, double>.from(result['scores'] ?? {}),
          profileSummary: result['profile_summary'] ?? '',
        );
      case 'disc':
        return DISCResultData(
          primaryStyle: result['primary_style'] ?? '',
          secondaryStyle: result['secondary_style'] ?? '',
          scores: Map<String, double>.from(result['scores'] ?? {}),
          profileSummary: result['profile_summary'] ?? '',
        );
      case 'dark_triad':
        return DarkTriadResultData(
          scores: Map<String, double>.from(result['scores'] ?? {}),
          darkCore: (result['dark_core'] ?? 0).toDouble(),
          riskLevel: result['risk_level'] ?? 'minimal',
          profileSummary: result['profile_summary'] ?? '',
        );
      case 'human_design':
        return HumanDesignResultData(
          type: result['type'] ?? '',
          typeName: result['type_info']?['es']?['name'] ?? result['type'] ?? '',
          strategy: result['strategy'] ?? '',
          authority: result['authority'] ?? '',
          profile: result['profile'] ?? '',
          centers: Map<String, bool>.from(result['centers'] ?? {}),
          personalityGates: List<int>.from(result['personality_gates'] ?? []),
          designGates: List<int>.from(result['design_gates'] ?? []),
          summary: result['summary'] ?? '',
        );
      default:
        return null;
    }
  }
}

// Simple data classes for history restoration
class BigFiveResultData {
  final Map<String, double> scores;
  final Map<String, double> facets;
  final String profileSummary;
  final Map<String, double> percentiles;
  BigFiveResultData({required this.scores, required this.facets, required this.profileSummary, required this.percentiles});
}

class MBTIResultData {
  final String typeCode;
  final Map<String, double> scores;
  final String profileSummary;
  final Map<String, double> percentages;
  MBTIResultData({required this.typeCode, required this.scores, required this.profileSummary, required this.percentages});
}

class EnneagramResultData {
  final int dominantType;
  final int wing;
  final Map<String, double> scores;
  final String profileSummary;
  EnneagramResultData({required this.dominantType, required this.wing, required this.scores, required this.profileSummary});
}

class DISCResultData {
  final String primaryStyle;
  final String secondaryStyle;
  final Map<String, double> scores;
  final String profileSummary;
  DISCResultData({required this.primaryStyle, required this.secondaryStyle, required this.scores, required this.profileSummary});
}

class DarkTriadResultData {
  final Map<String, double> scores;
  final double darkCore;
  final String riskLevel;
  final String profileSummary;
  DarkTriadResultData({required this.scores, required this.darkCore, required this.riskLevel, required this.profileSummary});
}

class HumanDesignResultData {
  final String type;
  final String typeName;
  final String strategy;
  final String authority;
  final String profile;
  final Map<String, bool> centers;
  final List<int> personalityGates;
  final List<int> designGates;
  final String summary;
  HumanDesignResultData({required this.type, required this.typeName, required this.strategy, required this.authority, required this.profile, required this.centers, required this.personalityGates, required this.designGates, required this.summary});
}
