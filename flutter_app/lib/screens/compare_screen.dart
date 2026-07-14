import 'dart:convert';
import 'package:flutter/material.dart';
import '../app.dart';
import '../config/theme.dart';
import '../models/results.dart';
import '../services/api_service.dart';

class CompareScreen extends StatefulWidget {
  final String testType;
  final Map<String, dynamic> currentResult;

  const CompareScreen({
    super.key,
    required this.testType,
    required this.currentResult,
  });

  @override
  State<CompareScreen> createState() => _CompareScreenState();
}

class _CompareScreenState extends State<CompareScreen> {
  final ApiService _api = ApiService();
  final TextEditingController _controller = TextEditingController();
  bool _loading = false;
  bool _showResult = false;
  String _error = '';
  ComparisonResult? _comparison;

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  Future<void> _compare() async {
    final raw = _controller.text.trim();
    if (raw.isEmpty) {
      setState(() => _error = 'Paste the other result JSON first');
      return;
    }

    Map<String, dynamic> otherResult;
    try {
      otherResult = jsonDecode(raw) as Map<String, dynamic>;
    } catch (_) {
      setState(() => _error = 'Invalid JSON format. Copy the full result as JSON.');
      return;
    }

    setState(() {
      _loading = true;
      _error = '';
    });

    try {
      final comparison = await _api.submitCompare(
        widget.testType,
        widget.currentResult,
        otherResult,
      );
      setState(() {
        _comparison = comparison;
        _showResult = true;
        _loading = false;
      });
    } catch (e) {
      setState(() {
        _error = 'Comparison failed: $e';
        _loading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);

    return Scaffold(
      appBar: AppBar(
        title: Text('${l10n?.translate('compare') ?? 'Compare'} ${l10n?.translate(widget.testType) ?? widget.testType}'),
      ),
      body: _showResult && _comparison != null
          ? _buildComparison(context, l10n)
          : _buildInput(context, l10n),
    );
  }

  Widget _buildInput(BuildContext context, AppLocalizations? l10n) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Row(
                children: [
                  const Icon(Icons.check_circle, color: Colors.green, size: 20),
                  const SizedBox(width: 12),
                  Expanded(
                    child: Text(
                      l10n?.translate('yourResultLoaded') ?? 'Your result is loaded and ready.',
                      style: const TextStyle(color: Colors.white70),
                    ),
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: 16),
          Text(
            l10n?.translate('pasteOtherResult') ?? 'Paste the other person\'s result JSON below:',
            style: const TextStyle(fontSize: 14, color: Colors.white70),
          ),
          const SizedBox(height: 8),
          TextField(
            controller: _controller,
            maxLines: 8,
            style: const TextStyle(fontFamily: 'monospace', fontSize: 12),
            decoration: InputDecoration(
              hintText: l10n?.translate('pasteJsonHere') ?? '{"scores": {...}, ...}',
              filled: true,
              fillColor: Colors.white.withOpacity(0.05),
              border: OutlineInputBorder(
                borderRadius: BorderRadius.circular(12),
                borderSide: BorderSide(color: Colors.white.withOpacity(0.2)),
              ),
            ),
          ),
          if (_error.isNotEmpty) ...[
            const SizedBox(height: 8),
            Text(_error, style: const TextStyle(color: Colors.red, fontSize: 13)),
          ],
          const SizedBox(height: 16),
          FilledButton.icon(
            onPressed: _loading ? null : _compare,
            icon: _loading
                ? const SizedBox(width: 18, height: 18, child: CircularProgressIndicator(strokeWidth: 2))
                : const Icon(Icons.compare_arrows),
            label: Text(l10n?.translate('compare') ?? 'Compare'),
          ),
        ],
      ),
    );
  }

  Widget _buildComparison(BuildContext context, AppLocalizations? l10n) {
    final c = _comparison!;
    final scoreColor = c.compatibilityScore >= 80
        ? Colors.green
        : c.compatibilityScore >= 60
            ? Colors.orange
            : Colors.red;

    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Compatibility score
          Card(
            child: Padding(
              padding: const EdgeInsets.all(24),
              child: Column(
                children: [
                  Text(
                    l10n?.translate('compatibility') ?? 'Compatibility',
                    style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
                  ),
                  const SizedBox(height: 16),
                  Text(
                    '${c.compatibilityScore.toStringAsFixed(0)}%',
                    style: TextStyle(
                      fontSize: 64,
                      fontWeight: FontWeight.w900,
                      color: scoreColor,
                    ),
                  ),
                  const SizedBox(height: 8),
                  if (c.typeCode1 != null && c.typeCode2 != null)
                    Text(
                      '${c.typeCode1} vs ${c.typeCode2}',
                      style: const TextStyle(fontSize: 18, color: Colors.white70),
                    ),
                  const SizedBox(height: 12),
                  Text(
                    c.description,
                    textAlign: TextAlign.center,
                    style: const TextStyle(color: Colors.white70, height: 1.5),
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: 12),
          // Factor details
          ...c.factors.entries.map((entry) {
            final factor = entry.value as Map<String, dynamic>;
            final desc = factor['description']?.toString() ?? '';
            final diff = factor['difference'];
            final match = factor['match'];

            return Card(
              child: Padding(
                padding: const EdgeInsets.all(16),
                child: Row(
                  children: [
                    if (match == true)
                      const Icon(Icons.check_circle, color: Colors.green, size: 20)
                    else if (match == false)
                      const Icon(Icons.swap_horiz, color: Colors.orange, size: 20)
                    else
                      Icon(
                        diff != null && (diff as num) > 15
                            ? Icons.swap_horiz
                            : Icons.check_circle,
                        color: diff != null && (diff as num) > 15 ? Colors.orange : Colors.green,
                        size: 20,
                      ),
                    const SizedBox(width: 12),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            entry.key,
                            style: const TextStyle(
                              fontWeight: FontWeight.bold,
                              fontSize: 14,
                              color: Colors.white,
                            ),
                          ),
                          const SizedBox(height: 4),
                          Text(desc, style: const TextStyle(color: Colors.white54, fontSize: 13)),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            );
          }),
          const SizedBox(height: 24),
          FilledButton.icon(
            onPressed: () {
              setState(() {
                _showResult = false;
                _comparison = null;
              });
            },
            icon: const Icon(Icons.refresh),
            label: Text(l10n?.translate('compareAgain') ?? 'Compare another'),
          ),
        ],
      ),
    );
  }
}
