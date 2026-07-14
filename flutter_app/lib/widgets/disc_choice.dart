import 'package:flutter/material.dart';

class DiscChoice extends StatelessWidget {
  final dynamic value;
  final ValueChanged<String> onChanged;

  const DiscChoice({super.key, required this.value, required this.onChanged});

  static const Map<String, String> _labels = {
    'a': 'D',
    'b': 'I',
    'c': 'S',
    'd': 'C',
  };

  static const Map<String, IconData> _icons = {
    'a': Icons.gps_fixed,
    'b': Icons.chat_bubble,
    'c': Icons.handshake,
    'd': Icons.analytics,
  };

  static const Map<String, Color> _colors = {
    'a': Color(0xFFEF4444),
    'b': Color(0xFFF59E0B),
    'c': Color(0xFF22C55E),
    'd': Color(0xFF3B82F6),
  };

  @override
  Widget build(BuildContext context) {
    return GridView.count(
      crossAxisCount: 2,
      shrinkWrap: true,
      mainAxisSpacing: 12,
      crossAxisSpacing: 12,
      childAspectRatio: 1.5,
      physics: const NeverScrollableScrollPhysics(),
      children: _labels.entries.map((entry) {
        final isSelected = value == entry.key;
        final color = _colors[entry.key]!;

        return GestureDetector(
          onTap: () => onChanged(entry.key),
          child: AnimatedContainer(
            duration: const Duration(milliseconds: 200),
            decoration: BoxDecoration(
              color: isSelected ? color.withOpacity(0.2) : Colors.white.withOpacity(0.05),
              borderRadius: BorderRadius.circular(12),
              border: Border.all(
                color: isSelected ? color : Colors.white.withOpacity(0.1),
                width: isSelected ? 2 : 1,
              ),
            ),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Icon(_icons[entry.key], color: isSelected ? color : Colors.white24, size: 28),
                const SizedBox(height: 8),
                Text(
                  entry.value,
                  style: TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.bold,
                    color: isSelected ? color : Colors.white38,
                  ),
                ),
              ],
            ),
          ),
        );
      }).toList(),
    );
  }
}
