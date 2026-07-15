import 'dart:io';
import 'package:flutter/material.dart';
import 'package:path_provider/path_provider.dart';
import 'package:open_file/open_file.dart';
import 'package:shared_preferences/shared_preferences.dart';
import '../app.dart';
import '../config/api.dart';
import '../config/theme.dart';
import '../models/results.dart';
import '../services/api_service.dart';
import '../services/share_service.dart';
import '../widgets/radar_chart.dart';
import '../widgets/score_bar_chart.dart';
import '../widgets/body_graph.dart';
import 'compare_screen.dart';
import 'evolution_screen.dart';

class ResultsScreen extends StatefulWidget {
  final String testType;
  final dynamic result;
  final Map<String, dynamic>? reportData;
  const ResultsScreen({super.key, required this.testType, required this.result, this.reportData});

  @override
  State<ResultsScreen> createState() => _ResultsScreenState();
}

class _ResultsScreenState extends State<ResultsScreen> {
  final _api = ApiService();
  String? _accountId;
  bool _saving = false;
  bool _saved = false;
  int _resultCount = 0;
  bool _downloadingPdf = false;

  @override
  void initState() {
    super.initState();
    _loadAccount();
  }

  Future<void> _loadAccount() async {
    final prefs = await SharedPreferences.getInstance();
    final id = prefs.getString('account_id');
    if (id != null) {
      setState(() => _accountId = id);
      _checkResultCount(id);
    }
  }

  Future<void> _checkResultCount(String accountId) async {
    try {
      final results = await _api.getResults(accountId, testType: widget.testType);
      if (mounted) {
        setState(() => _resultCount = results.length);
      }
    } catch (_) {}
  }

  Map<String, dynamic> _resultToMap() {
    if (widget.result is Map<String, dynamic>) {
      return widget.result as Map<String, dynamic>;
    }
    return (widget.result).toJson();
  }

  Map<String, dynamic>? get _rawData {
    final r = widget.result;
    if (r is Map<String, dynamic>) return r;
    if (r is BigFiveResult) return r.raw;
    if (r is MBTIResult) return r.raw;
    if (r is EnneagramResult) return r.raw;
    if (r is DISCResult) return r.raw;
    if (r is DarkTriadResult) return r.raw;
    if (r is HumanDesignResult) return r.raw;
    return null;
  }

  Future<void> _downloadPdf() async {
    if (widget.reportData == null || _downloadingPdf) return;
    setState(() => _downloadingPdf = true);
    try {
      final bytes = await _api.downloadReport(widget.testType, widget.reportData!);
      final dir = await getTemporaryDirectory();
      final file = File('${dir.path}/psicometrics_${widget.testType}_report.pdf');
      await file.writeAsBytes(bytes);
      if (mounted) {
        final result = await OpenFile.open(file.path);
        if (mounted && result.type != ResultType.done) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text('PDF guardado en: ${file.path}'),
              backgroundColor: Colors.green,
            ),
          );
        }
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Error al descargar PDF: $e'), backgroundColor: Colors.red),
        );
      }
    } finally {
      if (mounted) setState(() => _downloadingPdf = false);
    }
  }

  Future<void> _shareResult() async {
    if (_accountId == null) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Primero crea una cuenta para compartir'), backgroundColor: Colors.orange),
        );
      }
      return;
    }
    try {
      // First save the result
      final resultMap = _resultToMap();
      final saveResp = await _api.saveResult(_accountId!, widget.testType, resultMap);
      final resultId = saveResp['result_id'] as String;

      // Create share link
      final shareData = await _api.shareResult(_accountId!, resultId);
      final shareUrl = '${ApiConfig.baseUrl}${shareData['share_url']}';

      if (mounted) {
        await ShareService.shareText(shareUrl, 'Mira mi perfil de PsicoMetrics');
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Error al compartir: $e'), backgroundColor: Colors.red),
        );
      }
    }
  }

  Future<void> _exportCSV() async {
    if (_accountId == null) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Primero crea una cuenta para exportar'), backgroundColor: Colors.orange),
        );
      }
      return;
    }
    try {
      final bytes = await _api.exportCSV(_accountId!);
      final dir = await getTemporaryDirectory();
      final file = File('${dir.path}/psicometrics_results.csv');
      await file.writeAsBytes(bytes);
      if (mounted) {
        final result = await OpenFile.open(file.path);
        if (mounted && result.type != ResultType.done) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text('CSV guardado en: ${file.path}'), backgroundColor: Colors.green),
          );
        }
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Error al exportar CSV: $e'), backgroundColor: Colors.red),
        );
      }
    }
  }

  Future<void> _showPremiumDialog() async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (ctx) => AlertDialog(
        title: const Row(
          children: [
            Icon(Icons.star, color: Colors.amber),
            SizedBox(width: 8),
            Text('Informe Premium'),
          ],
        ),
        content: const Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Análisis exhaustivo de 5 páginas:'),
            SizedBox(height: 12),
            Text('• Resumen ejecutivo'),
            Text('• Análisis detallado por factor'),
            Text('• Tablas comparativas'),
            Text('• Grid de recomendaciones'),
            Text('• Plan de acción 90 días'),
            SizedBox(height: 16),
            Text('\$9.99 USD - Un solo pago'),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx, false),
            child: const Text('Cancelar'),
          ),
          FilledButton(
            onPressed: () => Navigator.pop(ctx, true),
            child: const Text('Obtener Informe Premium'),
          ),
        ],
      ),
    );

    if (confirmed == true) {
      try {
        final session = await _api.createCheckoutSession();
        final url = session['url'] as String;
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text('Redirigiendo a: $url'), backgroundColor: Colors.green),
          );
          // In a real app: launchUrl(Uri.parse(url));
        }
      } catch (e) {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text('Error: $e'), backgroundColor: Colors.red),
          );
        }
      }
    }
  }

  List<Widget> _buildCommonSections() {
    final raw = _rawData;
    if (raw == null) return [];
    final interpretation = raw['interpretation'];
    final recommendations = raw['recommendations'];
    final careerRecs = raw['career_recommendations'];
    final growthAreas = raw['growth_areas'];
    final validity = raw['validity'];
    final list = <Widget>[];

    if (interpretation is Map) {
      list.add(const SizedBox(height: 12));
      list.add(_buildInterpretation(interpretation));
    }

    if (recommendations is Map || careerRecs is List || growthAreas is List) {
      list.add(const SizedBox(height: 12));
      list.add(_buildRecommendations(
        recommendations: recommendations as Map<String, dynamic>?,
        careerRecs: careerRecs as List<dynamic>?,
        growthAreas: growthAreas as List<dynamic>?,
      ));
    }

    if (validity is Map) {
      list.add(const SizedBox(height: 12));
      list.add(_buildValidity(validity));
    }

    return list;
  }

  Widget _buildLevelBadge(String? level) {
    final color = switch (level?.toLowerCase()) {
      'low' => Colors.blueAccent,
      'moderate' => Colors.orangeAccent,
      'high' => Colors.greenAccent,
      _ => Colors.grey,
    };
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
      decoration: BoxDecoration(
        color: color.withOpacity(0.15),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: color),
      ),
      child: Text(
        (level ?? '').toUpperCase(),
        style: TextStyle(color: color, fontWeight: FontWeight.bold, fontSize: 12),
      ),
    );
  }

  Widget _buildInterpretation(Map interpretation) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(Icons.psychology, color: AppTheme.primary, size: 20),
                const SizedBox(width: 8),
                Text('Interpretación', style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
              ],
            ),
            const SizedBox(height: 16),
            ...interpretation.entries.map((entry) {
              final factorName = entry.key.toString();
              final data = entry.value as Map<String, dynamic>?;
              if (data == null) return const SizedBox.shrink();
              final level = data['level'] as String?;
              final description = data['description'] as String?;
              final dailyLife = data['daily_life'] as String?;
              final work = data['work'] as String?;
              final relationships = data['relationships'] as String?;

              return Padding(
                padding: const EdgeInsets.only(bottom: 16),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      children: [
                        Expanded(child: Text(factorName, style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 15))),
                        const SizedBox(width: 8),
                        _buildLevelBadge(level),
                      ],
                    ),
                    if (description != null && description.isNotEmpty) ...[
                      const SizedBox(height: 8),
                      Text(description, style: const TextStyle(color: Colors.white70, height: 1.5)),
                    ],
                    if (dailyLife != null && dailyLife.isNotEmpty) ...[
                      const SizedBox(height: 8),
                      Text('Vida diaria:', style: TextStyle(color: AppTheme.tertiary, fontWeight: FontWeight.w600)),
                      Text(dailyLife, style: const TextStyle(color: Colors.white70, height: 1.5)),
                    ],
                    if (work != null && work.isNotEmpty) ...[
                      const SizedBox(height: 8),
                      Text('Trabajo:', style: TextStyle(color: AppTheme.tertiary, fontWeight: FontWeight.w600)),
                      Text(work, style: const TextStyle(color: Colors.white70, height: 1.5)),
                    ],
                    if (relationships != null && relationships.isNotEmpty) ...[
                      const SizedBox(height: 8),
                      Text('Relaciones:', style: TextStyle(color: AppTheme.tertiary, fontWeight: FontWeight.w600)),
                      Text(relationships, style: const TextStyle(color: Colors.white70, height: 1.5)),
                    ],
                    const Divider(height: 24),
                  ],
                ),
              );
            }),
          ],
        ),
      ),
    );
  }

  Widget _buildRecommendations({
    Map<String, dynamic>? recommendations,
    List<dynamic>? careerRecs,
    List<dynamic>? growthAreas,
  }) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(Icons.lightbulb_outline, color: AppTheme.secondary, size: 20),
                const SizedBox(width: 8),
                Text('Recomendaciones', style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
              ],
            ),

            // Career recommendations
            if (careerRecs != null && careerRecs.isNotEmpty) ...[
              const SizedBox(height: 12),
              Text('Carreras recomendadas:', style: TextStyle(fontWeight: FontWeight.w600, color: AppTheme.tertiary)),
              const SizedBox(height: 8),
              Wrap(
                spacing: 8,
                runSpacing: 8,
                children: careerRecs.map((c) => Container(
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                  decoration: BoxDecoration(
                    color: AppTheme.primary.withOpacity(0.15),
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: AppTheme.primary.withOpacity(0.3)),
                  ),
                  child: Text('$c', style: TextStyle(fontWeight: FontWeight.w600, color: AppTheme.primary)),
                )).toList(),
              ),
            ],

            // Growth areas
            if (growthAreas != null && growthAreas.isNotEmpty) ...[
              const SizedBox(height: 12),
              Text('Áreas de mejora:', style: TextStyle(fontWeight: FontWeight.w600, color: AppTheme.tertiary)),
              const SizedBox(height: 8),
              Wrap(
                spacing: 8,
                runSpacing: 8,
                children: growthAreas.map((a) => Container(
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                  decoration: BoxDecoration(
                    color: Colors.orange.withOpacity(0.15),
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: Colors.orange.withOpacity(0.3)),
                  ),
                  child: Text('$a', style: TextStyle(fontWeight: FontWeight.w600, color: Colors.orangeAccent)),
                )).toList(),
              ),
            ],

            // Factor-level recommendations
            if (recommendations != null && recommendations.isNotEmpty) ...[
              const SizedBox(height: 12),
              ...recommendations.entries.map((entry) {
                final tips = entry.value as List<dynamic>?;
                if (tips == null || tips.isEmpty) return const SizedBox.shrink();
                return Padding(
                  padding: const EdgeInsets.only(bottom: 12),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(entry.key, style: const TextStyle(fontWeight: FontWeight.w600)),
                      const SizedBox(height: 4),
                      ...tips.map((tip) => Padding(
                        padding: const EdgeInsets.only(left: 8, top: 4),
                        child: Row(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Icon(Icons.check_circle_outline, size: 16, color: AppTheme.primary),
                            const SizedBox(width: 8),
                            Expanded(child: Text('$tip', style: const TextStyle(color: Colors.white70))),
                          ],
                        ),
                      )),
                    ],
                  ),
                );
              }),
            ],
          ],
        ),
      ),
    );
  }

  Widget _buildValidity(Map validity) {
    final warnings = validity['warnings'] as List<dynamic>?;
    final isInvalid = validity['is_valid'] == false;
    final attentionFailed = validity['attention_check_failed'] == true;
    final responsePattern = validity['response_pattern'] as String?;

    return Card(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(
                  isInvalid ? Icons.warning_amber : Icons.check_circle,
                  color: isInvalid ? Colors.orange : Colors.green,
                  size: 20,
                ),
                const SizedBox(width: 8),
                Text(
                  isInvalid ? 'Advertencias de validación' : 'Validación',
                  style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
                ),
              ],
            ),

            // Warnings
            if (warnings != null && warnings.isNotEmpty) ...[
              const SizedBox(height: 12),
              ...warnings.map((w) => Padding(
                padding: const EdgeInsets.only(bottom: 8),
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Icon(Icons.warning, size: 18, color: Colors.orange.shade300),
                    const SizedBox(width: 8),
                    Expanded(child: Text('$w', style: TextStyle(color: Colors.orange.shade200))),
                  ],
                ),
              )),
            ],

            // Response pattern
            if (responsePattern != null && responsePattern.isNotEmpty) ...[
              const SizedBox(height: 8),
              Row(
                children: [
                  Icon(Icons.analytics_outlined, size: 18, color: Colors.white54),
                  const SizedBox(width: 8),
                  Text('Patrón de respuesta: $responsePattern', style: const TextStyle(color: Colors.white54)),
                ],
              ),
            ],

            // Attention check failure
            if (attentionFailed) ...[
              const SizedBox(height: 8),
              Container(
                padding: const EdgeInsets.all(12),
                decoration: BoxDecoration(
                  color: Colors.red.withOpacity(0.15),
                  borderRadius: BorderRadius.circular(8),
                  border: Border.all(color: Colors.red.withOpacity(0.3)),
                ),
                child: Row(
                  children: [
                    Icon(Icons.error_outline, color: Colors.red.shade300, size: 20),
                    const SizedBox(width: 8),
                    Expanded(
                      child: Text(
                        'Algunas verificaciones de atención fallaron. Los resultados pueden no ser fiables.',
                        style: TextStyle(color: Colors.red.shade200),
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }

  void _navigateToCompare() {
    final resultMap = _resultToMap();
    Navigator.push(context, MaterialPageRoute(
      builder: (_) => CompareScreen(
        testType: widget.testType,
        currentResult: resultMap,
      ),
    ));
  }

  Future<void> _saveResult() async {
    if (_accountId == null || _saved) return;
    setState(() => _saving = true);

    try {
      final resultMap = _resultToMap();
      await _api.saveResult(_accountId!, widget.testType, resultMap);
      if (mounted) {
        setState(() {
          _saved = true;
          _saving = false;
          _resultCount++;
        });
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Resultado guardado en tu cuenta'), backgroundColor: Colors.green),
        );
      }
    } catch (e) {
      if (mounted) {
        setState(() => _saving = false);
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Error al guardar: $e'), backgroundColor: Colors.red),
        );
      }
    }
  }

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
            onPressed: () => ShareService.shareResult(testType: widget.testType, result: widget.result, lang: l10n?.locale.languageCode ?? 'es'),
          ),
          // Copy button
          IconButton(
            icon: const Icon(Icons.copy),
            onPressed: () async {
              await ShareService.copyResultToClipboard(testType: widget.testType, result: widget.result);
              if (context.mounted) {
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(content: Text('Copiado al portapapeles'), backgroundColor: Colors.green),
                );
              }
            },
          ),
          // Compare button
          IconButton(
            icon: const Icon(Icons.compare_arrows),
            tooltip: 'Comparar',
            onPressed: _navigateToCompare,
          ),
        ],
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            if (widget.result is BigFiveResult) _buildBigFive(context, widget.result as BigFiveResult, l10n),
            if (widget.result is MBTIResult) _buildMBTI(context, widget.result as MBTIResult, l10n),
            if (widget.result is EnneagramResult) _buildEnneagram(context, widget.result as EnneagramResult, l10n),
            if (widget.result is DISCResult) _buildDISC(context, widget.result as DISCResult, l10n),
            if (widget.result is DarkTriadResult) _buildDarkTriad(context, widget.result as DarkTriadResult, l10n),
            if (widget.result is HumanDesignResult) _buildHumanDesign(context, widget.result as HumanDesignResult, l10n),
            ..._buildCommonSections(),
            const SizedBox(height: 24),

            // Save to account button
            if (_accountId != null && !_saved)
              Padding(
                padding: const EdgeInsets.only(bottom: 12),
                child: OutlinedButton.icon(
                  onPressed: _saving ? null : _saveResult,
                  icon: _saving
                      ? const SizedBox(width: 16, height: 16, child: CircularProgressIndicator(strokeWidth: 2))
                      : const Icon(Icons.save),
                  label: Text(_saving ? 'Guardando...' : 'Guardar resultado en mi cuenta'),
                ),
              ),
            if (_saved)
              Padding(
                padding: const EdgeInsets.only(bottom: 12),
                child: Row(
                  children: [
                    const Icon(Icons.check_circle, color: Colors.green, size: 20),
                    const SizedBox(width: 8),
                    const Text('Resultado guardado', style: TextStyle(color: Colors.green)),
                    const Spacer(),
                    if (_resultCount > 1)
                      TextButton.icon(
                        onPressed: () => Navigator.push(context, MaterialPageRoute(
                          builder: (_) => EvolutionScreen(
                            accountId: _accountId!,
                            testType: widget.testType,
                          ),
                        )),
                        icon: const Icon(Icons.trending_up),
                        label: const Text('Ver evolución'),
                      ),
                  ],
                ),
              ),

            // Evolution button (if user already has saved results from before)
            if (_accountId != null && _resultCount > 0 && !_saved)
              Padding(
                padding: const EdgeInsets.only(bottom: 12),
                child: OutlinedButton.icon(
                  onPressed: () => Navigator.push(context, MaterialPageRoute(
                    builder: (_) => EvolutionScreen(
                      accountId: _accountId!,
                      testType: widget.testType,
                    ),
                  )),
                  icon: const Icon(Icons.trending_up),
                  label: Text('Ver evolución ($_resultCount resultados)'),
                ),
              ),

            // Download PDF button
            if (widget.reportData != null)
              Padding(
                padding: const EdgeInsets.only(bottom: 12),
                child: OutlinedButton.icon(
                  onPressed: _downloadingPdf ? null : _downloadPdf,
                  icon: _downloadingPdf
                      ? const SizedBox(width: 16, height: 16, child: CircularProgressIndicator(strokeWidth: 2))
                      : const Icon(Icons.picture_as_pdf),
                  label: Text(_downloadingPdf ? 'Descargando...' : 'Descargar PDF'),
                ),
              ),

            // Share button
            Padding(
              padding: const EdgeInsets.only(bottom: 12),
              child: OutlinedButton.icon(
                onPressed: () => _shareResult(),
                icon: const Icon(Icons.share),
                label: Text('Compartir perfil'),
              ),
            ),

            // Export CSV button
            Padding(
              padding: const EdgeInsets.only(bottom: 12),
              child: OutlinedButton.icon(
                onPressed: () => _exportCSV(),
                icon: const Icon(Icons.table_chart),
                label: Text('Exportar CSV'),
              ),
            ),

            // Premium Report button
            Padding(
              padding: const EdgeInsets.only(bottom: 12),
              child: FilledButton.icon(
                onPressed: () => _showPremiumDialog(),
                icon: const Icon(Icons.star),
                label: Text('Informe Premium'),
              ),
            ),

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
