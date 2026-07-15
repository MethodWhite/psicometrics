import 'package:flutter/material.dart';
import '../app.dart';
import '../config/theme.dart';
import '../services/api_service.dart';
import 'hd_form_screen.dart';
import 'test_screen.dart';

class TestInstructionsScreen extends StatefulWidget {
  final String testType;
  const TestInstructionsScreen({super.key, required this.testType});

  @override
  State<TestInstructionsScreen> createState() => _TestInstructionsScreenState();
}

class _TestInstructionsScreenState extends State<TestInstructionsScreen> {
  final ApiService _api = ApiService();
  Map<String, dynamic>? _metadata;
  bool _loading = true;
  bool _consentAccepted = false;
  String _error = '';

  @override
  void initState() {
    super.initState();
    _loadMetadata();
  }

  Future<void> _loadMetadata() async {
    try {
      final metadata = await _api.getTestMetadata(widget.testType);
      setState(() {
        _metadata = metadata;
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
    final lang = l10n?.locale.languageCode ?? 'es';

    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => Navigator.pop(context),
        ),
        title: Text(_metadata?['name']?['es'] ?? 'Test'),
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator(color: AppTheme.primary))
          : _error.isNotEmpty
              ? Center(child: Text(_error))
              : SingleChildScrollView(
                  padding: const EdgeInsets.all(24),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      // Test info card
                      Card(
                        child: Padding(
                          padding: const EdgeInsets.all(20),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Row(
                                children: [
                                  const Icon(Icons.info_outline, color: AppTheme.primary, size: 20),
                                  const SizedBox(width: 8),
                                  Text(
                                    '${_getTestName(lang)}',
                                    style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
                                  ),
                                ],
                              ),
                              const SizedBox(height: 8),
                              Text('${_metadata?['item_count']} preguntas · ${_metadata?['estimated_minutes']} min',
                                  style: const TextStyle(color: Colors.white54)),
                            ],
                          ),
                        ),
                      ),
                      const SizedBox(height: 16),

                      // Scientific basis
                      if (_metadata?['scientific_basis'] != null) ...[
                        Card(
                          child: Padding(
                            padding: const EdgeInsets.all(20),
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Row(
                                  children: [
                                    const Icon(Icons.science, color: AppTheme.tertiary, size: 20),
                                    const SizedBox(width: 8),
                                    const Text('Base Científica',
                                        style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                                  ],
                                ),
                                const SizedBox(height: 12),
                                Text(
                                  _getField('scientific_basis', lang),
                                  style: const TextStyle(color: Colors.white70, height: 1.5),
                                ),
                              ],
                            ),
                          ),
                        ),
                        const SizedBox(height: 16),
                      ],

                      // Instructions
                      Card(
                        child: Padding(
                          padding: const EdgeInsets.all(20),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Row(
                                children: [
                                  const Icon(Icons.list_alt, color: AppTheme.secondary, size: 20),
                                  const SizedBox(width: 8),
                                  const Text('Instrucciones',
                                      style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                                ],
                              ),
                              const SizedBox(height: 12),
                              Text(
                                _getField('instructions', lang),
                                style: const TextStyle(color: Colors.white70, height: 1.5),
                              ),
                            ],
                          ),
                        ),
                      ),
                      const SizedBox(height: 16),

                      // Consent
                      Card(
                        color: _consentAccepted
                            ? Colors.green.withOpacity(0.1)
                            : Colors.orange.withOpacity(0.1),
                        child: Padding(
                          padding: const EdgeInsets.all(20),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Row(
                                children: [
                                  Icon(
                                    _consentAccepted ? Icons.check_circle : Icons.warning_amber,
                                    color: _consentAccepted ? Colors.green : Colors.orange,
                                    size: 20,
                                  ),
                                  const SizedBox(width: 8),
                                  const Text('Consentimiento',
                                      style: TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
                                ],
                              ),
                              const SizedBox(height: 12),
                              Text(
                                _getField('consent', lang),
                                style: const TextStyle(color: Colors.white70, height: 1.5),
                              ),
                              const SizedBox(height: 16),
                              CheckboxListTile(
                                title: const Text('Acepto los términos y condiciones'),
                                value: _consentAccepted,
                                onChanged: (v) => setState(() => _consentAccepted = v ?? false),
                                activeColor: AppTheme.primary,
                                contentPadding: EdgeInsets.zero,
                              ),
                            ],
                          ),
                        ),
                      ),
                      const SizedBox(height: 24),

                      // Start button
                      FilledButton.icon(
                        onPressed: _consentAccepted
                            ? () {
                                if (widget.testType == 'human_design') {
                                  Navigator.push(context, MaterialPageRoute(
                                    builder: (_) => HdFormScreen(testType: widget.testType),
                                  ));
                                } else {
                                  Navigator.push(context, MaterialPageRoute(
                                    builder: (_) => TestScreen(testType: widget.testType),
                                  ));
                                }
                              }
                            : null,
                        icon: const Icon(Icons.play_arrow),
                        label: Text(l10n?.translate('startTest') ?? 'Comenzar Test'),
                        style: FilledButton.styleFrom(
                          backgroundColor: AppTheme.primary,
                          padding: const EdgeInsets.symmetric(vertical: 16),
                          disabledBackgroundColor: Colors.white12,
                        ),
                      ),
                    ],
                  ),
                ),
    );
  }

  String _getField(String field, String lang) {
    final data = _metadata?[field];
    if (data == null) return '';
    if (data is Map) {
      return data[lang] ?? data['es'] ?? '';
    }
    return data.toString();
  }

  String _getTestName(String lang) {
    final name = _metadata?['name'];
    if (name is Map) {
      return name[lang] ?? name['es'] ?? 'Test';
    }
    return 'Test';
  }
}
