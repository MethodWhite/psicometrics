
import 'package:flutter/services.dart';
import 'package:share_plus/share_plus.dart';

class ShareService {
  static Future<void> shareResult({
    required String testType,
    required dynamic result,
    required String lang,
  }) async {
    final text = _buildShareText(testType, result, lang);
    await Share.share(text, subject: 'Mi resultado en PsicoMetrics');
  }

  static String _buildShareText(String testType, dynamic result, String lang) {
    final buffer = StringBuffer();
    buffer.writeln('🧠 PsicoMetrics — Mi Perfil de Personalidad\n');

    switch (testType) {
      case 'big_five':
        if (result != null) {
          buffer.writeln('📊 Big Five / OCEAN');
          buffer.writeln('');
          final scores = result.scores;
          buffer.writeln('Apertura: ${scores['O']?.toStringAsFixed(0)}%');
          buffer.writeln('Responsabilidad: ${scores['C']?.toStringAsFixed(0)}%');
          buffer.writeln('Extraversión: ${scores['E']?.toStringAsFixed(0)}%');
          buffer.writeln('Amabilidad: ${scores['A']?.toStringAsFixed(0)}%');
          buffer.writeln('Neuroticismo: ${scores['N']?.toStringAsFixed(0)}%');
          buffer.writeln('');
          buffer.writeln(result.profileSummary);
        }
        break;

      case 'mbti':
        if (result != null) {
          buffer.writeln('🎭 MBTI');
          buffer.writeln('');
          buffer.writeln('Mi tipo: ${result.typeCode}');
          buffer.writeln('');
          buffer.writeln(result.profileSummary);
        }
        break;

      case 'enneagram':
        if (result != null) {
          buffer.writeln('🔵 Enneagrama');
          buffer.writeln('');
          buffer.writeln('Tipo ${result.dominantType}w${result.wing}');
          buffer.writeln('');
          buffer.writeln(result.profileSummary);
        }
        break;

      case 'disc':
        if (result != null) {
          buffer.writeln('📊 DISC');
          buffer.writeln('');
          buffer.writeln('Estilo: ${result.primaryStyle}${result.secondaryStyle}');
          buffer.writeln('');
          buffer.writeln(result.profileSummary);
        }
        break;

      case 'dark_triad':
        if (result != null) {
          buffer.writeln('🌑 Triada Oscura');
          buffer.writeln('');
          buffer.writeln('Dark Core: ${result.darkCore.toStringAsFixed(0)}%');
          buffer.writeln('Nivel de riesgo: ${result.riskLevel}');
          buffer.writeln('');
          buffer.writeln(result.profileSummary);
        }
        break;

      case 'human_design':
        if (result != null) {
          buffer.writeln('⭐ Diseño Humano');
          buffer.writeln('');
          buffer.writeln('Tipo: ${result.typeName}');
          buffer.writeln('Estrategia: ${result.strategy}');
          buffer.writeln('Autoridad: ${result.authority}');
          buffer.writeln('Perfil: ${result.profile}');
          buffer.writeln('');
          buffer.writeln(result.summary);
        }
        break;
    }

    buffer.writeln('');
    buffer.writeln('Descargá PsicoMetrics para hacer tu test: https://psicometrics.app');

    return buffer.toString();
  }

  static Future<void> copyResultToClipboard({
    required String testType,
    required dynamic result,
  }) async {
    final text = _buildShareText(testType, result, 'es');
    await Clipboard.setData(ClipboardData(text: text));
  }
}
