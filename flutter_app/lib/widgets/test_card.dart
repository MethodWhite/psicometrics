import 'package:flutter/material.dart';
import '../config/theme.dart';
import '../models/test_info.dart';
import '../app.dart';

class TestCard extends StatelessWidget {
  final TestInfo test;
  final VoidCallback onTap;

  const TestCard({super.key, required this.test, required this.onTap});

  static const Map<String, String> _icons = {
    'big_five': '🧠',
    'mbti': '🎭',
    'enneagram': '🔵',
    'disc': '📊',
    'dark_triad': '🌑',
    'human_design': '⭐',
  };

  static const Map<String, List<Color>> _colors = {
    'big_five': [Color(0xFF6366F1), Color(0xFF8B5CF6)],
    'mbti': [Color(0xFFEC4899), Color(0xFFE11D48)],
    'enneagram': [Color(0xFF06B6D4), Color(0xFF2563EB)],
    'disc': [Color(0xFFF59E0B), Color(0xFFF97316)],
    'dark_triad': [Color(0xFFEF4444), Color(0xFFF97316)],
    'human_design': [Color(0xFF8B5CF6), Color(0xFF6366F1)],
  };

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    final colors = _colors[test.testType] ?? [Colors.grey, Colors.grey];

    return GestureDetector(
      onTap: onTap,
      child: Container(
        decoration: BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Colors.white.withOpacity(0.05), Colors.white.withOpacity(0.02)],
          ),
          borderRadius: BorderRadius.circular(16),
          border: Border.all(color: Colors.white.withOpacity(0.1)),
        ),
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Container(
              width: 48,
              height: 48,
              decoration: BoxDecoration(
                gradient: LinearGradient(colors: colors),
                borderRadius: BorderRadius.circular(12),
              ),
              child: Center(
                child: Text(_icons[test.testType] ?? '📋', style: const TextStyle(fontSize: 24)),
              ),
            ),
            const SizedBox(height: 12),
            Text(
              l10n?.translate(test.testType) ?? test.name,
              style: const TextStyle(
                fontSize: 15,
                fontWeight: FontWeight.bold,
                color: Colors.white,
              ),
              maxLines: 2,
              overflow: TextOverflow.ellipsis,
            ),
            const SizedBox(height: 6),
            Text(
              l10n?.translate('${test.testType}_desc') ?? test.description,
              style: const TextStyle(fontSize: 11, color: Colors.white38),
              maxLines: 2,
              overflow: TextOverflow.ellipsis,
            ),
            const Spacer(),
            Row(
              children: [
                Text(
                  l10n?.translate('${test.testType}_items') ?? '${test.itemCount} items',
                  style: const TextStyle(fontSize: 10, color: Colors.white24),
                ),
                const Text(' • ', style: TextStyle(fontSize: 10, color: Colors.white24)),
                Text(
                  l10n?.translate('${test.testType}_time') ?? '',
                  style: const TextStyle(fontSize: 10, color: Colors.white24),
                ),
              ],
            ),
            const SizedBox(height: 4),
            Row(
              children: [
                Text(
                  l10n?.translate('startTest') ?? '',
                  style: const TextStyle(fontSize: 12, color: AppTheme.primary, fontWeight: FontWeight.w500),
                ),
                const Icon(Icons.arrow_forward_ios, size: 10, color: AppTheme.primary),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
