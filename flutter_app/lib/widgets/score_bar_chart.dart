import 'package:flutter/material.dart';
import '../config/theme.dart';

class ScoreBarChart extends StatelessWidget {
  final Map<String, double> scores;
  final String lang;
  final String prefix;

  const ScoreBarChart({
    super.key,
    required this.scores,
    required this.lang,
    this.prefix = '',
  });

  static const Map<String, String> _labelsEs = {
    'O': 'Apertura', 'C': 'Responsabilidad', 'E': 'Extraversión',
    'A': 'Amabilidad', 'N': 'Neuroticismo',
    'EI': 'E/I', 'SN': 'S/N', 'TF': 'T/F', 'JP': 'J/P',
    'E/I': 'E/I', 'S/N': 'S/N', 'T/F': 'T/F', 'J/P': 'J/P',
    'M': 'Maquiavelismo', 'Narc': 'Narcisismo', 'P': 'Psicopatía',
    'D': 'Dominancia', 'I': 'Influencia', 'S': 'Estabilidad',
  };

  static const Map<String, String> _labelsEn = {
    'O': 'Openness', 'C': 'Conscientiousness', 'E': 'Extraversion',
    'A': 'Agreeableness', 'N': 'Neuroticism',
    'EI': 'E/I', 'SN': 'S/N', 'TF': 'T/F', 'JP': 'J/P',
    'E/I': 'E/I', 'S/N': 'S/N', 'T/F': 'T/F', 'J/P': 'J/P',
    'M': 'Machiavellianism', 'Narc': 'Narcissism', 'P': 'Psychopathy',
    'D': 'Dominance', 'I': 'Influence', 'S': 'Steadiness',
  };

  @override
  Widget build(BuildContext context) {
    final labels = lang == 'es' ? _labelsEs : _labelsEn;

    return Column(
      children: scores.entries.map((entry) {
        final label = '$prefix${labels[entry.key] ?? entry.key}';
        final value = entry.value;

        return Padding(
          padding: const EdgeInsets.only(bottom: 12),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(label, style: const TextStyle(fontSize: 13, color: Colors.white70)),
                  Text('${value.toStringAsFixed(1)}%', style: const TextStyle(fontSize: 13, color: Colors.white54)),
                ],
              ),
              const SizedBox(height: 6),
              ClipRRect(
                borderRadius: BorderRadius.circular(4),
                child: LinearProgressIndicator(
                  value: value / 100,
                  minHeight: 8,
                  backgroundColor: Colors.white10,
                  valueColor: const AlwaysStoppedAnimation(AppTheme.primary),
                ),
              ),
            ],
          ),
        );
      }).toList(),
    );
  }
}
