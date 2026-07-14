import 'dart:math';
import 'package:flutter/material.dart';
import '../config/theme.dart';

class BigFiveRadarChart extends StatelessWidget {
  final Map<String, double> scores;
  final String lang;

  const BigFiveRadarChart({super.key, required this.scores, required this.lang});

  static const Map<String, String> _labelsEs = {
    'O': 'Apertura',
    'C': 'Responsabilidad',
    'E': 'Extraversión',
    'A': 'Amabilidad',
    'N': 'Neuroticismo',
  };

  static const Map<String, String> _labelsEn = {
    'O': 'Openness',
    'C': 'Conscientiousness',
    'E': 'Extraversion',
    'A': 'Agreeableness',
    'N': 'Neuroticism',
  };

  @override
  Widget build(BuildContext context) {
    final labels = lang == 'es' ? _labelsEs : _labelsEn;
    final factors = ['O', 'C', 'E', 'A', 'N'];
    final values = factors.map((f) => (scores[f] ?? 50) / 100).toList();

    return CustomPaint(
      painter: _RadarPainter(values: values, labels: factors.map((f) => labels[f] ?? f).toList()),
      size: Size.infinite,
    );
  }
}

class _RadarPainter extends CustomPainter {
  final List<double> values;
  final List<String> labels;

  _RadarPainter({required this.values, required this.labels});

  @override
  void paint(Canvas canvas, Size size) {
    final center = Offset(size.width / 2, size.height / 2);
    final radius = min(size.width, size.height) * 0.35;
    final sides = values.length;
    final angle = 2 * pi / sides;

    // Draw grid
    final gridPaint = Paint()
      ..color = Colors.white.withOpacity(0.1)
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1;

    for (int level = 1; level <= 5; level++) {
      final r = radius * level / 5;
      final path = Path();
      for (int i = 0; i < sides; i++) {
        final a = angle * i - pi / 2;
        final point = Offset(center.dx + r * cos(a), center.dy + r * sin(a));
        if (i == 0) {
          path.moveTo(point.dx, point.dy);
        } else {
          path.lineTo(point.dx, point.dy);
        }
      }
      path.close();
      canvas.drawPath(path, gridPaint);
    }

    // Draw axes
    for (int i = 0; i < sides; i++) {
      final a = angle * i - pi / 2;
      final point = Offset(center.dx + radius * cos(a), center.dy + radius * sin(a));
      canvas.drawLine(center, point, gridPaint);
    }

    // Draw data polygon
    final dataPath = Path();
    final dataPaint = Paint()
      ..color = AppTheme.primary.withOpacity(0.3)
      ..style = PaintingStyle.fill;

    for (int i = 0; i < sides; i++) {
      final a = angle * i - pi / 2;
      final r = radius * values[i];
      final point = Offset(center.dx + r * cos(a), center.dy + r * sin(a));
      if (i == 0) {
        dataPath.moveTo(point.dx, point.dy);
      } else {
        dataPath.lineTo(point.dx, point.dy);
      }
    }
    dataPath.close();
    canvas.drawPath(dataPath, dataPaint);

    // Draw data polygon border
    final borderPaint = Paint()
      ..color = AppTheme.primary
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2;
    canvas.drawPath(dataPath, borderPaint);

    // Draw data points
    final pointPaint = Paint()..color = AppTheme.primary;
    for (int i = 0; i < sides; i++) {
      final a = angle * i - pi / 2;
      final r = radius * values[i];
      final point = Offset(center.dx + r * cos(a), center.dy + r * sin(a));
      canvas.drawCircle(point, 5, pointPaint);
      canvas.drawCircle(point, 3, Paint()..color = Colors.white);
    }

    // Draw labels
    final textPainter = TextPainter(textDirection: TextDirection.ltr);
    for (int i = 0; i < sides; i++) {
      final a = angle * i - pi / 2;
      final labelRadius = radius + 24;
      final point = Offset(
        center.dx + labelRadius * cos(a),
        center.dy + labelRadius * sin(a),
      );
      textPainter.text = TextSpan(
        text: labels[i],
        style: const TextStyle(color: Colors.white54, fontSize: 11),
      );
      textPainter.layout();
      textPainter.paint(canvas, Offset(point.dx - textPainter.width / 2, point.dy - textPainter.height / 2));
    }
  }

  @override
  bool shouldRepaint(covariant _RadarPainter oldDelegate) => true;
}
