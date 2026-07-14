import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import '../config/theme.dart';
import '../services/history_service.dart';
import 'results_screen.dart';

class HistoryScreen extends StatefulWidget {
  const HistoryScreen({super.key});

  @override
  State<HistoryScreen> createState() => _HistoryScreenState();
}

class _HistoryScreenState extends State<HistoryScreen> {
  List<Map<String, dynamic>> _history = [];
  bool _loading = true;

  @override
  void initState() {
    super.initState();
    _loadHistory();
  }

  Future<void> _loadHistory() async {
    final history = await TestHistory.getHistory();
    setState(() {
      _history = history;
      _loading = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Historial'),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => Navigator.pop(context),
        ),
        actions: [
          if (_history.isNotEmpty)
            IconButton(
              icon: const Icon(Icons.delete_outline),
              onPressed: _showClearDialog,
            ),
        ],
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator(color: AppTheme.primary))
          : _history.isEmpty
              ? _buildEmptyState()
              : _buildHistoryList(),
    );
  }

  Widget _buildEmptyState() {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Icon(Icons.history, size: 64, color: Colors.white.withOpacity(0.2)),
          const SizedBox(height: 16),
          Text(
            'Sin resultados aún',
            style: TextStyle(fontSize: 18, color: Colors.white.withOpacity(0.5)),
          ),
          const SizedBox(height: 8),
          Text(
            'Completá un test para ver tu historial',
            style: TextStyle(fontSize: 14, color: Colors.white.withOpacity(0.3)),
          ),
        ],
      ),
    );
  }

  Widget _buildHistoryList() {
    return ListView.builder(
      padding: const EdgeInsets.all(16),
      itemCount: _history.length,
      itemBuilder: (context, index) {
        final entry = _history[index];
        final testType = entry['testType'] ?? '';
        final timestamp = DateTime.tryParse(entry['timestamp'] ?? '') ?? DateTime.now();
        final result = entry['result'];

        return Card(
          margin: const EdgeInsets.only(bottom: 12),
          child: InkWell(
            onTap: () => _openResult(entry),
            borderRadius: BorderRadius.circular(16),
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Row(
                children: [
                  // Icon
                  Container(
                    width: 48,
                    height: 48,
                    decoration: BoxDecoration(
                      color: AppTheme.primary.withOpacity(0.15),
                      borderRadius: BorderRadius.circular(12),
                    ),
                    child: Center(
                      child: Text(
                        TestHistory.getTestIcon(testType),
                        style: const TextStyle(fontSize: 24),
                      ),
                    ),
                  ),
                  const SizedBox(width: 16),

                  // Info
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          TestHistory.getTestDisplayName(testType),
                          style: const TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
                        ),
                        const SizedBox(height: 4),
                        Text(
                          _getResultSummary(testType, result),
                          style: TextStyle(fontSize: 13, color: Colors.white.withOpacity(0.5)),
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                        ),
                      ],
                    ),
                  ),

                  // Date + Arrow
                  Column(
                    crossAxisAlignment: CrossAxisAlignment.end,
                    children: [
                      Text(
                        DateFormat('dd/MM/yyyy').format(timestamp),
                        style: TextStyle(fontSize: 12, color: Colors.white.withOpacity(0.4)),
                      ),
                      const SizedBox(height: 4),
                      Text(
                        DateFormat('HH:mm').format(timestamp),
                        style: TextStyle(fontSize: 11, color: Colors.white.withOpacity(0.3)),
                      ),
                    ],
                  ),
                  const SizedBox(width: 8),
                  Icon(Icons.chevron_right, color: Colors.white.withOpacity(0.3)),
                ],
              ),
            ),
          ),
        );
      },
    );
  }

  String _getResultSummary(String testType, dynamic result) {
    if (result == null) return '';
    try {
      switch (testType) {
        case 'big_five':
          return 'O: ${result['scores']?['O']?.toStringAsFixed(0) ?? '?'}% | C: ${result['scores']?['C']?.toStringAsFixed(0) ?? '?'}%';
        case 'mbti':
          return result['type_code'] ?? '';
        case 'enneagram':
          return 'Tipo ${result['dominant_type'] ?? '?'}w${result['wing'] ?? '?'}';
        case 'disc':
          return '${result['primary_style'] ?? ''}${result['secondary_style'] ?? ''}';
        case 'dark_triad':
          return 'Dark Core: ${result['dark_core']?.toStringAsFixed(0) ?? '?'}%';
        case 'human_design':
          return result['type'] ?? '';
        default:
          return '';
      }
    } catch (e) {
      return '';
    }
  }

  void _openResult(Map<String, dynamic> entry) {
    final testType = entry['testType'] ?? '';
    final resultData = TestHistory.restoreResult(entry);
    if (resultData != null) {
      Navigator.push(context, MaterialPageRoute(
        builder: (_) => ResultsScreen(testType: testType, result: resultData),
      ));
    }
  }

  void _showClearDialog() {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Limpiar historial'),
        content: const Text('¿Eliminar todos los resultados?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Cancelar'),
          ),
          FilledButton(
            onPressed: () async {
              await TestHistory.clearHistory();
              setState(() => _history.clear());
              if (mounted) Navigator.pop(context);
            },
            style: FilledButton.styleFrom(backgroundColor: Colors.red),
            child: const Text('Eliminar'),
          ),
        ],
      ),
    );
  }
}
