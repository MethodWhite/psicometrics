import 'package:flutter/material.dart';
import '../app.dart';
import '../config/theme.dart';
import '../models/results.dart';
import '../services/share_service.dart';
import '../widgets/radar_chart.dart';
import '../widgets/score_bar_chart.dart';
import '../widgets/body_graph.dart';

class ResultsScreen extends StatelessWidget {
  final String testType;
  final dynamic result;
  const ResultsScreen({super.key, required this.testType, required this.result});

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);

    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => Navigator.pop(context),
        ),
        title: Text(l10n?.translate('results') ?? 'Resultados'),
        actions: [
          // Share button
          IconButton(
            icon: const Icon(Icons.share),
            onPressed: () => ShareService.shareResult(testType: testType, result: result, lang: l10n?.locale.languageCode ?? 'es'),
          ),
          // Copy button
          IconButton(
            icon: const Icon(Icons.copy),
            onPressed: () async {
              await ShareService.copyResultToClipboard(testType: testType, result: result);
              if (context.mounted) {
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(content: Text('Copiado al portapapeles'), backgroundColor: Colors.green),
                );
              }
            },
          ),
        ],
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            if (result is BigFiveResult) _buildBigFive(context, result as BigFiveResult, l10n),
            if (result is MBTIResult) _buildMBTI(context, result as MBTIResult, l10n),
            if (result is EnneagramResult) _buildEnneagram(context, result as EnneagramResult, l10n),
            if (result is DISCResult) _buildDISC(context, result as DISCResult, l10n),
            if (result is DarkTriadResult) _buildDarkTriad(context, result as DarkTriadResult, l10n),
            if (result is HumanDesignResult) _buildHumanDesign(context, result as HumanDesignResult, l10n),
            const SizedBox(height: 24),

            // Action buttons
            Row(
              children: [
                Expanded(
                  child: OutlinedButton.icon(
                    onPressed: () => Navigator.pop(context),
                    icon: const Icon(Icons.arrow_back),
                    label: Text(l10n?.translate('back') ?? 'Volver'),
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: FilledButton.icon(
                    onPressed: () => Navigator.pop(context),
                    icon: const Icon(Icons.refresh),
                    label: Text(l10n?.translate('retake') ?? 'Volver a hacer'),
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildBigFive(BuildContext context, BigFiveResult r, AppLocalizations? l10n) {
    return Column(
      children: [
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              children: [
                Text(l10n?.translate('yourProfile') ?? '', style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold)),
                const SizedBox(height: 16),
                SizedBox(
                  height: 280,
                  child: BigFiveRadarChart(scores: r.scores, lang: l10n?.locale.languageCode ?? 'es'),
                ),
              ],
            ),
          ),
        ),
        const SizedBox(height: 12),
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Icon(Icons.bar_chart, color: AppTheme.primary, size: 20),
                    const SizedBox(width: 8),
                    Text(l10n?.translate('score') ?? '', style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                  ],
                ),
                const SizedBox(height: 16),
                ScoreBarChart(scores: r.scores, lang: l10n?.locale.languageCode ?? 'es'),
              ],
            ),
          ),
        ),
        const SizedBox(height: 12),
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Icon(Icons.percent, color: AppTheme.tertiary, size: 20),
                    const SizedBox(width: 8),
                    Text(l10n?.translate('percentile') ?? '', style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                  ],
                ),
                const SizedBox(height: 16),
                ScoreBarChart(scores: r.percentiles, lang: l10n?.locale.languageCode ?? 'es'),
              ],
            ),
          ),
        ),
        const SizedBox(height: 12),
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Icon(Icons.info_outline, color: AppTheme.secondary, size: 20),
                    const SizedBox(width: 8),
                    Text(l10n?.translate('about') ?? '', style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                  ],
                ),
                const SizedBox(height: 12),
                Text(r.profileSummary, style: const TextStyle(color: Colors.white70, height: 1.5)),
              ],
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildMBTI(BuildContext context, MBTIResult r, AppLocalizations? l10n) {
    return Column(
      children: [
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              children: [
                Text(l10n?.translate('yourProfile') ?? '', style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold)),
                const SizedBox(height: 16),
                ShaderMask(
                  shaderCallback: (bounds) => const LinearGradient(
                    colors: [AppTheme.primary, AppTheme.tertiary],
                  ).createShader(bounds),
                  child: Text(
                    r.typeCode,
                    style: TextStyle(fontSize: 72, fontWeight: FontWeight.w900, color: Colors.white),
                  ),
                ),
                const SizedBox(height: 16),
                Text(r.profileSummary, textAlign: TextAlign.center, style: const TextStyle(color: Colors.white70, height: 1.5)),
              ],
            ),
          ),
        ),
        const SizedBox(height: 12),
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Icon(Icons.bar_chart, color: AppTheme.primary, size: 20),
                    const SizedBox(width: 8),
                    Text(l10n?.translate('score') ?? '', style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                  ],
                ),
                const SizedBox(height: 16),
                ScoreBarChart(scores: r.percentages, lang: l10n?.locale.languageCode ?? 'es'),
              ],
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildEnneagram(BuildContext context, EnneagramResult r, AppLocalizations? l10n) {
    return Column(
      children: [
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              children: [
                Text(l10n?.translate('yourProfile') ?? '', style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold)),
                const SizedBox(height: 16),
                ShaderMask(
                  shaderCallback: (bounds) => const LinearGradient(
                    colors: [AppTheme.primary, AppTheme.tertiary],
                  ).createShader(bounds),
                  child: Text(
                    'Tipo ${r.dominantType}w${r.wing}',
                    style: TextStyle(fontSize: 48, fontWeight: FontWeight.w900, color: Colors.white),
                  ),
                ),
                const SizedBox(height: 8),
                Text('${l10n?.translate('dominant') ?? ''}: Tipo ${r.dominantType} | ${l10n?.translate('wing') ?? ''}: ${r.wing}',
                    style: const TextStyle(color: Colors.white54)),
                const SizedBox(height: 12),
                Text(r.profileSummary, textAlign: TextAlign.center, style: const TextStyle(color: Colors.white70, height: 1.5)),
              ],
            ),
          ),
        ),
        const SizedBox(height: 12),
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Icon(Icons.bar_chart, color: AppTheme.primary, size: 20),
                    const SizedBox(width: 8),
                    Text(l10n?.translate('score') ?? '', style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                  ],
                ),
                const SizedBox(height: 16),
                ScoreBarChart(scores: r.scores, lang: l10n?.locale.languageCode ?? 'es', prefix: 'Tipo '),
              ],
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildDISC(BuildContext context, DISCResult r, AppLocalizations? l10n) {
    return Column(
      children: [
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              children: [
                Text(l10n?.translate('yourProfile') ?? '', style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold)),
                const SizedBox(height: 16),
                ShaderMask(
                  shaderCallback: (bounds) => const LinearGradient(
                    colors: [AppTheme.primary, AppTheme.tertiary],
                  ).createShader(bounds),
                  child: Text(
                    '${r.primaryStyle}${r.secondaryStyle}',
                    style: TextStyle(fontSize: 48, fontWeight: FontWeight.w900, color: Colors.white),
                  ),
                ),
                const SizedBox(height: 12),
                Text(r.profileSummary, textAlign: TextAlign.center, style: const TextStyle(color: Colors.white70, height: 1.5)),
              ],
            ),
          ),
        ),
        const SizedBox(height: 12),
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Icon(Icons.bar_chart, color: AppTheme.primary, size: 20),
                    const SizedBox(width: 8),
                    Text(l10n?.translate('score') ?? '', style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                  ],
                ),
                const SizedBox(height: 16),
                ScoreBarChart(scores: r.scores, lang: l10n?.locale.languageCode ?? 'es'),
              ],
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildDarkTriad(BuildContext context, DarkTriadResult r, AppLocalizations? l10n) {
    final riskColors = {
      'minimal': Colors.green,
      'low': Colors.blue,
      'moderate': Colors.orange,
      'high': Colors.red,
    };
    final riskLabels = {
      'minimal': l10n?.translate('riskMinimal') ?? 'Mínimo',
      'low': l10n?.translate('riskLow') ?? 'Bajo',
      'moderate': l10n?.translate('riskModerate') ?? 'Moderado',
      'high': l10n?.translate('riskHigh') ?? 'Alto',
    };

    return Column(
      children: [
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              children: [
                Text(l10n?.translate('yourProfile') ?? '', style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold)),
                const SizedBox(height: 16),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 10),
                  decoration: BoxDecoration(
                    color: (riskColors[r.riskLevel] ?? Colors.grey).withOpacity(0.15),
                    borderRadius: BorderRadius.circular(20),
                    border: Border.all(color: riskColors[r.riskLevel] ?? Colors.grey),
                  ),
                  child: Text(
                    riskLabels[r.riskLevel] ?? r.riskLevel,
                    style: TextStyle(
                      color: riskColors[r.riskLevel] ?? Colors.grey,
                      fontWeight: FontWeight.bold,
                      fontSize: 18,
                    ),
                  ),
                ),
                const SizedBox(height: 16),
                Text(r.profileSummary, textAlign: TextAlign.center, style: const TextStyle(color: Colors.white70, height: 1.5)),
              ],
            ),
          ),
        ),
        const SizedBox(height: 12),
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(Icons.warning_amber, color: riskColors[r.riskLevel], size: 20),
                    const SizedBox(width: 8),
                    Text(l10n?.translate('darkCore') ?? '', style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                  ],
                ),
                const SizedBox(height: 12),
                ShaderMask(
                  shaderCallback: (bounds) => const LinearGradient(
                    colors: [Colors.red, Colors.orange],
                  ).createShader(bounds),
                  child: Text(
                    '${r.darkCore.toStringAsFixed(1)}%',
                    style: TextStyle(fontSize: 42, fontWeight: FontWeight.w900, color: Colors.white),
                  ),
                ),
              ],
            ),
          ),
        ),
        const SizedBox(height: 12),
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Icon(Icons.bar_chart, color: AppTheme.primary, size: 20),
                    const SizedBox(width: 8),
                    Text(l10n?.translate('score') ?? '', style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                  ],
                ),
                const SizedBox(height: 16),
                ScoreBarChart(scores: r.scores, lang: l10n?.locale.languageCode ?? 'es'),
              ],
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildHumanDesign(BuildContext context, HumanDesignResult r, AppLocalizations? l10n) {
    final lang = l10n?.locale.languageCode ?? 'es';
    final typeName = r.typeInfo[lang]?['name'] ?? r.type;

    return Column(
      children: [
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              children: [
                Text(l10n?.translate('yourProfile') ?? '', style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold)),
                const SizedBox(height: 16),
                ShaderMask(
                  shaderCallback: (bounds) => const LinearGradient(
                    colors: [AppTheme.primary, AppTheme.tertiary],
                  ).createShader(bounds),
                  child: Text(
                    typeName,
                    style: TextStyle(fontSize: 36, fontWeight: FontWeight.w900, color: Colors.white),
                  ),
                ),
                const SizedBox(height: 8),
                Text('${l10n?.translate('profile') ?? ''}: ${r.profile}', style: const TextStyle(color: Colors.white54)),
                const SizedBox(height: 12),
                Text(r.summary, textAlign: TextAlign.center, style: const TextStyle(color: Colors.white70, height: 1.5)),
              ],
            ),
          ),
        ),
        const SizedBox(height: 12),
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Icon(Icons.explore, color: AppTheme.primary, size: 20),
                    const SizedBox(width: 8),
                    Text(l10n?.translate('strategy') ?? '', style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                  ],
                ),
                const SizedBox(height: 8),
                Text(r.strategy, style: const TextStyle(color: Colors.white70, height: 1.5)),
              ],
            ),
          ),
        ),
        const SizedBox(height: 12),
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Icon(Icons.psychology, color: AppTheme.secondary, size: 20),
                    const SizedBox(width: 8),
                    Text(l10n?.translate('authority') ?? '', style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                  ],
                ),
                const SizedBox(height: 8),
                Text(r.authority, style: const TextStyle(color: Colors.white70, height: 1.5)),
              ],
            ),
          ),
        ),
        const SizedBox(height: 12),
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(Icons.accessibility_new, color: AppTheme.tertiary, size: 20),
                    const SizedBox(width: 8),
                    Text(l10n?.translate('bodyGraph') ?? '', style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                  ],
                ),
                const SizedBox(height: 16),
                BodyGraphWidget(centers: r.centers),
              ],
            ),
          ),
        ),
        const SizedBox(height: 12),
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Icon(Icons.fingerprint, color: AppTheme.primary, size: 20),
                    const SizedBox(width: 8),
                    Text(l10n?.translate('personalityGates') ?? '', style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                  ],
                ),
                const SizedBox(height: 12),
                Wrap(
                  spacing: 8,
                  runSpacing: 8,
                  children: r.personalityGates.map((gate) => Container(
                    padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                    decoration: BoxDecoration(
                      color: AppTheme.primary.withOpacity(0.15),
                      borderRadius: BorderRadius.circular(8),
                      border: Border.all(color: AppTheme.primary.withOpacity(0.3)),
                    ),
                    child: Text('$gate', style: const TextStyle(fontSize: 14, fontWeight: FontWeight.w600, color: AppTheme.primary)),
                  )).toList(),
                ),
              ],
            ),
          ),
        ),
        const SizedBox(height: 12),
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Icon(Icons.design_services, color: AppTheme.secondary, size: 20),
                    const SizedBox(width: 8),
                    Text(l10n?.translate('designGates') ?? '', style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                  ],
                ),
                const SizedBox(height: 12),
                Wrap(
                  spacing: 8,
                  runSpacing: 8,
                  children: r.designGates.map((gate) => Container(
                    padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                    decoration: BoxDecoration(
                      color: AppTheme.secondary.withOpacity(0.15),
                      borderRadius: BorderRadius.circular(8),
                      border: Border.all(color: AppTheme.secondary.withOpacity(0.3)),
                    ),
                    child: Text('$gate', style: const TextStyle(fontSize: 14, fontWeight: FontWeight.w600, color: AppTheme.secondary)),
                  )).toList(),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}
