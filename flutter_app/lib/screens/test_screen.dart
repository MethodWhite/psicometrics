import 'package:flutter/material.dart';
import '../app.dart';
import '../config/theme.dart';
import '../models/question.dart';
import '../services/api_service.dart';
import '../services/history_service.dart';
import '../widgets/likert_scale.dart';
import '../widgets/dichotomy_choice.dart';
import '../widgets/disc_choice.dart';
import 'results_screen.dart';

class TestScreen extends StatefulWidget {
  final String testType;
  const TestScreen({super.key, required this.testType});

  @override
  State<TestScreen> createState() => _TestScreenState();
}

class _TestScreenState extends State<TestScreen> {
  final ApiService _api = ApiService();
  TestData? _testData;
  int _currentQuestion = 0;
  final Map<int, dynamic> _answers = {};
  bool _loading = true;
  bool _submitting = false;
  String _error = '';

  @override
  void initState() {
    super.initState();
    _loadTest();
  }

  Future<void> _loadTest() async {
    try {
      final lang = AppLocalizations.of(context)?.locale.languageCode ?? 'es';
      final data = await _api.getTestQuestions(widget.testType, lang: lang);
      setState(() {
        _testData = data;
        _loading = false;
      });
    } catch (e) {
      setState(() {
        _loading = false;
        _error = e.toString();
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);

    if (_loading) {
      return Scaffold(
        body: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              const CircularProgressIndicator(color: AppTheme.primary),
              const SizedBox(height: 16),
              Text(l10n?.translate('loading') ?? '', style: const TextStyle(color: Colors.white54)),
            ],
          ),
        ),
      );
    }

    if (_error.isNotEmpty) {
      return Scaffold(
        body: Center(
          child: Padding(
            padding: const EdgeInsets.all(32),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                const Icon(Icons.error_outline, size: 64, color: Colors.red),
                const SizedBox(height: 16),
                Text(l10n?.translate('error') ?? '', style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
                const SizedBox(height: 8),
                Text(_error, textAlign: TextAlign.center, style: const TextStyle(color: Colors.white54)),
                const SizedBox(height: 24),
                Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    OutlinedButton.icon(
                      onPressed: () => Navigator.pop(context),
                      icon: const Icon(Icons.arrow_back),
                      label: Text(l10n?.translate('back') ?? ''),
                    ),
                    const SizedBox(width: 12),
                    FilledButton.icon(
                      onPressed: () {
                        setState(() { _loading = true; _error = ''; });
                        _loadTest();
                      },
                      icon: const Icon(Icons.refresh),
                      label: Text(l10n?.translate('retry') ?? 'Reintentar'),
                    ),
                  ],
                ),
              ],
            ),
          ),
        ),
      );
    }

    if (_testData == null) return const SizedBox();

    final questions = _testData!.questions;
    if (questions.isEmpty) return const SizedBox();

    final question = questions[_currentQuestion];
    final progress = (_currentQuestion + 1) / questions.length;

    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          icon: const Icon(Icons.close),
          onPressed: () => _showExitDialog(),
        ),
        title: Text(_testData!.name, style: const TextStyle(fontSize: 16)),
        actions: [
          Center(
            child: Padding(
              padding: const EdgeInsets.only(right: 16),
              child: Text(
                '${(progress * 100).toInt()}%',
                style: const TextStyle(fontSize: 14, fontWeight: FontWeight.w600),
              ),
            ),
          ),
        ],
      ),
      body: Column(
        children: [
          // Progress bar
          LinearProgressIndicator(
            value: progress,
            backgroundColor: Colors.white10,
            valueColor: const AlwaysStoppedAnimation(AppTheme.primary),
            minHeight: 3,
          ),

          // Question counter
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text(
                  l10n?.translate('questionOf')
                      .replaceAll('{{current}}', '${_currentQuestion + 1}')
                      .replaceAll('{{total}}', '${questions.length}') ?? '',
                  style: const TextStyle(color: Colors.white54, fontSize: 13),
                ),
                // Answered count
                Row(
                  children: [
                    Icon(Icons.check_circle, size: 14, color: Colors.green.withOpacity(0.7)),
                    const SizedBox(width: 4),
                    Text(
                      '${_answers.length}/${questions.length}',
                      style: TextStyle(fontSize: 12, color: Colors.white.withOpacity(0.4)),
                    ),
                  ],
                ),
              ],
            ),
          ),

          // Question card
          Expanded(
            child: SingleChildScrollView(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              child: Card(
                child: Padding(
                  padding: const EdgeInsets.all(24),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      // Question number badge
                      Container(
                        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
                        decoration: BoxDecoration(
                          color: AppTheme.primary.withOpacity(0.15),
                          borderRadius: BorderRadius.circular(12),
                        ),
                        child: Text(
                          '${_currentQuestion + 1}',
                          style: const TextStyle(fontSize: 12, fontWeight: FontWeight.bold, color: AppTheme.primary),
                        ),
                      ),
                      const SizedBox(height: 16),
                      // Question text
                      Text(
                        question.text,
                        style: const TextStyle(fontSize: 18, color: Colors.white, height: 1.5),
                      ),
                      const SizedBox(height: 32),
                      // Answer widget
                      _buildAnswerWidget(question),
                    ],
                  ),
                ),
              ),
            ),
          ),

          // Navigation buttons
          Padding(
            padding: const EdgeInsets.all(16),
            child: Row(
              children: [
                // Previous
                Expanded(
                  child: OutlinedButton.icon(
                    onPressed: _currentQuestion > 0
                        ? () => setState(() => _currentQuestion--)
                        : null,
                    icon: const Icon(Icons.arrow_back, size: 18),
                    label: Text(l10n?.translate('previous') ?? ''),
                  ),
                ),
                const SizedBox(width: 12),
                // Next / Submit
                Expanded(
                  child: FilledButton.icon(
                    onPressed: _answers[question.id] == null
                        ? null
                        : _currentQuestion == questions.length - 1
                            ? (_submitting ? null : _submit)
                            : () => setState(() => _currentQuestion++),
                    icon: _submitting
                        ? const SizedBox(
                            width: 18, height: 18,
                            child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white),
                          )
                        : Icon(
                            _currentQuestion == questions.length - 1
                                ? Icons.check
                                : Icons.arrow_forward,
                            size: 18,
                          ),
                    label: Text(
                      _submitting
                          ? (l10n?.translate('loading') ?? '')
                          : _currentQuestion == questions.length - 1
                              ? (l10n?.translate('submit') ?? '')
                              : (l10n?.translate('next') ?? ''),
                    ),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildAnswerWidget(Question question) {
    if (widget.testType == 'big_five' || widget.testType == 'enneagram' || widget.testType == 'dark_triad') {
      final labels = _testData?.scale?['labels'];
      final labelList = labels is List ? List<String>.from(labels) : ['1', '2', '3', '4', '5'];
      return LikertScale(
        value: _answers[question.id],
        labels: labelList,
        onChanged: (v) => setState(() => _answers[question.id] = v),
      );
    }

    if (widget.testType == 'mbti') {
      // Find paired question (same dichotomy, different pole)
      final paired = _testData!.questions.firstWhere(
        (q) => q.dichotomy == question.dichotomy && q.id != question.id && q.pole != question.pole,
        orElse: () => question,
      );
      return DichotomyChoice(
        value: _answers[question.id],
        optionA: question.text,
        optionB: paired.text,
        onChanged: (v) => setState(() => _answers[question.id] = v),
      );
    }

    if (widget.testType == 'disc') {
      return DiscChoice(
        value: _answers[question.id],
        onChanged: (v) => setState(() => _answers[question.id] = v),
      );
    }

    return const SizedBox();
  }

  void _showExitDialog() {
    final l10n = AppLocalizations.of(context);
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(l10n?.translate('exitTest') ?? '¿Salir del test?'),
        content: Text(
          _answers.isEmpty
              ? (l10n?.translate('exitConfirm') ?? '¿Querés volver al menú principal?')
              : (l10n?.translate('exitLoseProgress') ?? 'Perderás el progreso actual. ¿Querés salir?'),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: Text(l10n?.translate('cancel') ?? 'Cancelar'),
          ),
          FilledButton(
            onPressed: () {
              Navigator.pop(context);
              Navigator.pop(context);
            },
            style: FilledButton.styleFrom(backgroundColor: Colors.red),
            child: Text(l10n?.translate('exit') ?? 'Salir'),
          ),
        ],
      ),
    );
  }

  Future<void> _submit() async {
    setState(() => _submitting = true);
    try {
      final lang = AppLocalizations.of(context)?.locale.languageCode ?? 'es';
      final answersList = _answers.entries.map((e) => {
        'question_id': e.key,
        'value': e.value,
      }).toList();

      dynamic result;
      switch (widget.testType) {
        case 'big_five':
          result = await _api.submitBigFive(answersList, lang);
          break;
        case 'mbti':
          result = await _api.submitMBTI(answersList, lang);
          break;
        case 'enneagram':
          result = await _api.submitEnneagram(answersList, lang);
          break;
        case 'disc':
          result = await _api.submitDISC(answersList, lang);
          break;
        case 'dark_triad':
          result = await _api.submitDarkTriad(answersList, lang);
          break;
      }

      // Save to history
      await TestHistory.saveResult(
        testType: widget.testType,
        testName: _testData!.name,
        result: result,
        lang: lang,
      );

      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(AppLocalizations.of(context)?.translate('saved') ?? 'Resultado guardado'),
            backgroundColor: Colors.green,
          ),
        );
        Navigator.pushReplacement(context, MaterialPageRoute(
          builder: (_) => ResultsScreen(testType: widget.testType, result: result),
        ));
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Error: $e'), backgroundColor: Colors.red),
        );
      }
    } finally {
      setState(() => _submitting = false);
    }
  }
}
