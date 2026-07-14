import 'package:flutter/material.dart';
import '../config/theme.dart';

class BodyGraphWidget extends StatelessWidget {
  final Map<String, bool> centers;

  const BodyGraphWidget({super.key, required this.centers});

  static const Map<String, Offset> _positions = {
    'head': Offset(150, 30),
    'ajna': Offset(150, 90),
    'throat': Offset(150, 150),
    'g': Offset(150, 220),
    'heart': Offset(80, 200),
    'sacral': Offset(150, 310),
    'solar_plexus': Offset(220, 280),
    'splenic': Offset(80, 300),
    'root': Offset(150, 380),
  };

  static const Map<String, String> _labelsEs = {
    'head': 'Corona',
    'ajna': 'Ajna',
    'throat': 'Garganta',
    'g': 'G/Identidad',
    'heart': 'Corazón',
    'sacral': 'Sacral',
    'solar_plexus': 'Plexo Solar',
    'splenic': 'Splénico',
    'root': 'Raíz',
  };

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 300,
      height: 420,
      child: CustomPaint(
        painter: _BodyGraphPainter(centers: centers, labels: _labelsEs),
      ),
    );
  }
}

class _BodyGraphPainter extends CustomPainter {
  final Map<String, bool> centers;
  final Map<String, String> labels;

  _BodyGraphPainter({required this.centers, required this.labels});

  @override
  void paint(Canvas canvas, Size size) {
    final scaleX = size.width / 300;
    final scaleY = size.height / 420;

    final linePaint = Paint()
      ..color = Colors.white.withOpacity(0.1)
      ..strokeWidth = 2
      ..style = PaintingStyle.stroke;

    // Draw connections
    final connections = [
      ['head', 'ajna'],
      ['ajna', 'throat'],
      ['throat', 'g'],
      ['g', 'sacral'],
      ['sacral', 'root'],
      ['g', 'heart'],
      ['g', 'solar_plexus'],
      ['splenic', 'sacral'],
    ];

    for (final conn in connections) {
      final from = BodyGraphWidget._positions[conn[0]]!;
      final to = BodyGraphWidget._positions[conn[1]]!;
      canvas.drawLine(
        Offset(from.dx * scaleX, from.dy * scaleY),
        Offset(to.dx * scaleX, to.dy * scaleY),
        linePaint,
      );
    }

    // Draw centers
    final textPainter = TextPainter(textDirection: TextDirection.ltr);

    for (final entry in BodyGraphWidget._positions.entries) {
      final pos = entry.value;
      final defined = centers[entry.key] ?? false;
      final cx = pos.dx * scaleX;
      final cy = pos.dy * scaleY;

      final shapePaint = Paint()
        ..color = defined ? AppTheme.primary : Colors.white.withOpacity(0.15)
        ..style = defined ? PaintingStyle.fill : PaintingStyle.stroke
        ..strokeWidth = 2;

      // Draw shape
      if (entry.key == 'head' || entry.key == 'root') {
        // Triangle
        final path = Path()
          ..moveTo(cx, cy - 15 * scaleY)
          ..lineTo(cx + 15 * scaleX, cy + 5 * scaleY)
          ..lineTo(cx - 15 * scaleX, cy + 5 * scaleY)
          ..close();
        canvas.drawPath(path, shapePaint);
      } else if (entry.key == 'heart') {
        // Diamond
        final path = Path()
          ..moveTo(cx, cy - 12 * scaleY)
          ..lineTo(cx + 12 * scaleX, cy)
          ..lineTo(cx, cy + 12 * scaleY)
          ..lineTo(cx - 12 * scaleX, cy)
          ..close();
        canvas.drawPath(path, shapePaint);
      } else {
        // Square
        canvas.drawRect(
          Rect.fromCenter(center: Offset(cx, cy), width: 30 * scaleX, height: 30 * scaleY),
          shapePaint,
        );
      }

      // Draw label
      textPainter.text = TextSpan(
        text: labels[entry.key] ?? entry.key,
        style: TextStyle(
          color: defined ? Colors.white70 : Colors.white24,
          fontSize: 9,
        ),
      );
      textPainter.layout();
      textPainter.paint(canvas, Offset(cx - textPainter.width / 2, cy + 22 * scaleY));
    }
  }

  @override
  bool shouldRepaint(covariant _BodyGraphPainter oldDelegate) => true;
}
