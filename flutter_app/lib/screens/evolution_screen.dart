import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:fl_chart/fl_chart.dart';
import '../config/theme.dart';
import '../services/api_service.dart';

class EvolutionScreen extends StatefulWidget {
  final String accountId;
  final String testType;

  const EvolutionScreen({
    super.key,
    required this.accountId,
    required this.testType,
  });

  @override
  State<EvolutionScreen> createState() => _EvolutionScreenState();
}

class _EvolutionScreenState extends State<EvolutionScreen> {
  final _api = ApiService();
  List<Map<String, dynamic>> _data = [];
  bool _loading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    try {
      final raw = await _api.getEvolution(widget.accountId, widget.testType);
      setState(() {
        _data = raw.cast<Map<String, dynamic>>();
        _loading = false;
      });
    } catch (e) {
      setState(() {
        _error = e.toString();
        _loading = false;
      });
    }
  }

  String _formatDate(int ts) {
    final dt = DateTime.fromMillisecondsSinceEpoch(ts * 1000);
    return '${dt.day}/${dt.month}';
  }

  @override
  Widget build(BuildContext context) {
    final testName = widget.testType.replaceAll('_', ' ').split(' ').map((w) =>
        w.isNotEmpty ? '${w[0].toUpperCase()}${w.substring(1)}' : '').join(' ');

    return Scaffold(
      appBar: AppBar(
        title: Text('Evolución - $testName'),
      ),
      body: _buildBody(),
    );
  }

  Widget _buildBody() {
    if (_loading) return const Center(child: CircularProgressIndicator());
    if (_error != null) return Center(child: Text('Error: $_error', style: const TextStyle(color: Colors.red)));
    if (_data.isEmpty) return const Center(child: Text('Sin datos de evolución'));

    // Collect all score keys from all data points
    final allKeys = <String>{};
    for (final item in _data) {
      final scores = item['scores'];
      if (scores is Map) {
        scores.forEach((k, _) => allKeys.add(k.toString()));
      }
    }

    final sortedKeys = allKeys.toList()..sort();
    final colors = [
      Colors.cyanAccent,
      Colors.pinkAccent,
      Colors.amberAccent,
      Colors.greenAccent,
      Colors.orangeAccent,
      Colors.blueAccent,
      Colors.purpleAccent,
      Colors.redAccent,
    ];

    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('${_data.length} mediciones', style: const TextStyle(color: Colors.white70)),
          const SizedBox(height: 16),

          // Legend
          Wrap(
            spacing: 12,
            runSpacing: 4,
            children: sortedKeys.asMap().entries.map((e) => Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                Container(width: 12, height: 12, color: colors[e.key % colors.length]),
                const SizedBox(width: 4),
                Text(e.value, style: const TextStyle(fontSize: 12)),
              ],
            )).toList(),
          ),

          const SizedBox(height: 16),

          // Line chart
          Expanded(
            child: Card(
              child: Padding(
                padding: const EdgeInsets.fromLTRB(8, 24, 16, 16),
                child: LineChart(
                  LineChartData(
                    gridData: FlGridData(
                      show: true,
                      drawVerticalLine: false,
                      getDrawingHorizontalLine: (value) => FlLine(
                        color: Colors.white10,
                        strokeWidth: 1,
                      ),
                    ),
                    titlesData: FlTitlesData(
                      leftTitles: AxisTitles(
                        sideTitles: SideTitles(
                          showTitles: true,
                          reservedSize: 32,
                          getTitlesWidget: (value, meta) => Text(
                            value.toInt().toString(),
                            style: const TextStyle(fontSize: 10, color: Colors.white54),
                          ),
                        ),
                      ),
                      bottomTitles: AxisTitles(
                        sideTitles: SideTitles(
                          showTitles: true,
                          reservedSize: 24,
                          interval: 1,
                          getTitlesWidget: (value, meta) {
                            final idx = value.toInt();
                            if (idx < 0 || idx >= _data.length) return const SizedBox();
                            return Padding(
                              padding: const EdgeInsets.only(top: 4),
                              child: Text(
                                _formatDate((_data[idx]['timestamp'] as num).toInt()),
                                style: const TextStyle(fontSize: 9, color: Colors.white54),
                              ),
                            );
                          },
                        ),
                      ),
                      topTitles: AxisTitles(sideTitles: SideTitles(showTitles: false)),
                      rightTitles: AxisTitles(sideTitles: SideTitles(showTitles: false)),
                    ),
                    borderData: FlBorderData(show: false),
                    lineBarsData: sortedKeys.asMap().entries.map((entry) {
                      final keyIdx = entry.key;
                      final key = entry.value;
                      final spots = _data.asMap().entries.map((item) {
                        final scores = item.value['scores'] as Map? ?? {};
                        final val = (scores[key] as num?)?.toDouble() ?? 0;
                        return FlSpot(item.key.toDouble(), val);
                      }).toList();

                      return LineChartBarData(
                        spots: spots,
                        isCurved: true,
                        preventCurveOverShooting: true,
                        color: colors[keyIdx % colors.length],
                        barWidth: 2,
                        dotData: FlDotData(
                          show: true,
                          getDotPainter: (spot, percent, barData, index) => FlDotCirclePainter(
                            radius: 3,
                            color: colors[keyIdx % colors.length],
                            strokeWidth: 0,
                          ),
                        ),
                        belowBarData: BarAreaData(show: false),
                      );
                    }).toList(),
                    minY: 0,
                    maxY: 100,
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
