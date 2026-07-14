import 'package:flutter/material.dart';
import '../config/theme.dart';

class DichotomyChoice extends StatelessWidget {
  final dynamic value;
  final String optionA;
  final String optionB;
  final ValueChanged<String> onChanged;

  const DichotomyChoice({
    super.key,
    required this.value,
    required this.optionA,
    required this.optionB,
    required this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Expanded(child: _buildOption('a', optionA, AppTheme.primary)),
        const Padding(
          padding: EdgeInsets.symmetric(horizontal: 12),
          child: Text('VS', style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold, color: Colors.white24)),
        ),
        Expanded(child: _buildOption('b', optionB, AppTheme.secondary)),
      ],
    );
  }

  Widget _buildOption(String option, String text, Color color) {
    final isSelected = value == option;

    return GestureDetector(
      onTap: () => onChanged(option),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 200),
        padding: const EdgeInsets.all(20),
        decoration: BoxDecoration(
          color: isSelected ? color.withOpacity(0.2) : Colors.white.withOpacity(0.05),
          borderRadius: BorderRadius.circular(16),
          border: Border.all(
            color: isSelected ? color : Colors.white.withOpacity(0.1),
            width: isSelected ? 2 : 1,
          ),
          boxShadow: isSelected
              ? [BoxShadow(color: color.withOpacity(0.3), blurRadius: 12)]
              : null,
        ),
        child: Column(
          children: [
            Icon(
              isSelected ? Icons.check_circle : Icons.circle_outlined,
              color: isSelected ? color : Colors.white24,
              size: 32,
            ),
            const SizedBox(height: 12),
            Text(
              text,
              textAlign: TextAlign.center,
              style: TextStyle(
                fontSize: 13,
                color: isSelected ? Colors.white : Colors.white54,
                fontWeight: isSelected ? FontWeight.w600 : FontWeight.normal,
              ),
              maxLines: 4,
              overflow: TextOverflow.ellipsis,
            ),
          ],
        ),
      ),
    );
  }
}
