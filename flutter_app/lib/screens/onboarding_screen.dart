import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import '../config/theme.dart';
import 'home_screen.dart';

class OnboardingScreen extends StatefulWidget {
  const OnboardingScreen({super.key});

  @override
  State<OnboardingScreen> createState() => _OnboardingScreenState();
}

class _OnboardingScreenState extends State<OnboardingScreen> {
  final PageController _pageController = PageController();
  int _currentPage = 0;

  final List<_OnboardingPage> _pages = [
    _OnboardingPage(
      icon: Icons.psychology,
      title: 'Descubrí tu Personalidad',
      description: 'Accedé a 6 tests científicamente validados para conocer tu perfil de personalidad.',
      color: AppTheme.primary,
    ),
    _OnboardingPage(
      icon: Icons.science,
      title: 'Estándares Científicos',
      description: 'Big Five, MBTI, Enneagrama, DISC, Triada Oscura y Diseño Humano — todos con preguntas oficiales.',
      color: AppTheme.secondary,
    ),
    _OnboardingPage(
      icon: Icons.show_chart,
      title: 'Resultados Detallados',
      description: 'Visualizá tus resultados con gráficos interactivos, perfiles completos y explicaciones.',
      color: AppTheme.tertiary,
    ),
    _OnboardingPage(
      icon: Icons.history,
      title: 'Historial y Progreso',
      description: 'Guardá tus resultados y seguí tu evolución a lo largo del tiempo.',
      color: const Color(0xFF22C55E),
    ),
    _OnboardingPage(
      icon: Icons.tips_and_updates,
      title: 'Big Five / OCEAN — Lo que aprenderás',
      description: '',
      color: const Color(0xFF8B5CF6),
      factors: const [
        _FactorExplanation('O', 'Apertura', 'Qué tan abierto sos a nuevas experiencias e ideas'),
        _FactorExplanation('C', 'Responsabilidad', 'Tu nivel de organización, disciplina y compromiso'),
        _FactorExplanation('E', 'Extraversión', 'Cómo te relacionás con los demás y el mundo exterior'),
        _FactorExplanation('A', 'Amabilidad', 'Tu tendencia a la cooperación y la empatía'),
        _FactorExplanation('N', 'Neuroticismo', 'Tu sensibilidad a emociones negativas y el estrés'),
      ],
    ),
  ];

  @override
  void dispose() {
    _pageController.dispose();
    super.dispose();
  }

  Future<void> _completeOnboarding() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setBool('onboarding_complete', true);
    if (mounted) {
      Navigator.pushReplacement(context, MaterialPageRoute(
        builder: (_) => const HomeScreen(),
      ));
    }
  }

  Widget _buildFactorTile(_FactorExplanation f) {
    final colors = {
      'O': const Color(0xFF8B5CF6),
      'C': const Color(0xFF3B82F6),
      'E': const Color(0xFFF59E0B),
      'A': const Color(0xFF22C55E),
      'N': const Color(0xFFEF4444),
    };
    final color = colors[f.letter] ?? AppTheme.primary;

    return Container(
      margin: const EdgeInsets.only(bottom: 10),
      padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
      decoration: BoxDecoration(
        color: color.withOpacity(0.1),
        borderRadius: BorderRadius.circular(10),
        border: Border.all(color: color.withOpacity(0.2)),
      ),
      child: Row(
        children: [
          Container(
            width: 32,
            height: 32,
            alignment: Alignment.center,
            decoration: BoxDecoration(
              color: color.withOpacity(0.2),
              borderRadius: BorderRadius.circular(8),
            ),
            child: Text(
              f.letter,
              style: TextStyle(
                color: color,
                fontWeight: FontWeight.bold,
                fontSize: 16,
              ),
            ),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  f.name,
                  style: const TextStyle(
                    color: Colors.white,
                    fontWeight: FontWeight.w600,
                    fontSize: 15,
                  ),
                ),
                const SizedBox(height: 2),
                Text(
                  f.explanation,
                  style: TextStyle(
                    color: Colors.white.withOpacity(0.6),
                    fontSize: 13,
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Color(0xFF0F172A), Color(0xFF1E1B4B)],
          ),
        ),
        child: SafeArea(
          child: Column(
            children: [
              // Skip button
              Align(
                alignment: Alignment.topRight,
                child: TextButton(
                  onPressed: _completeOnboarding,
                  child: Text(
                    'Saltar',
                    style: TextStyle(color: Colors.white.withOpacity(0.6)),
                  ),
                ),
              ),

              // Pages
              Expanded(
                child: PageView.builder(
                  controller: _pageController,
                  itemCount: _pages.length,
                  onPageChanged: (index) => setState(() => _currentPage = index),
                  itemBuilder: (context, index) {
                    final page = _pages[index];
                    return Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 24),
                      child: Column(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: [
                          // Icon
                          Container(
                            width: 100,
                            height: 100,
                            decoration: BoxDecoration(
                              color: page.color.withOpacity(0.15),
                              shape: BoxShape.circle,
                              border: Border.all(color: page.color.withOpacity(0.3)),
                            ),
                            child: Icon(page.icon, size: 48, color: page.color),
                          ),
                          const SizedBox(height: 32),
                          // Title
                          Text(
                            page.title,
                            textAlign: TextAlign.center,
                            style: const TextStyle(
                              fontSize: 24,
                              fontWeight: FontWeight.bold,
                              color: Colors.white,
                            ),
                          ),
                          const SizedBox(height: 12),
                          // Description or factors
                          if (page.factors != null)
                            Expanded(
                              child: SingleChildScrollView(
                                padding: const EdgeInsets.only(top: 16),
                                child: Column(
                                  children: page.factors!.map((f) => _buildFactorTile(f)).toList(),
                                ),
                              ),
                            )
                          else
                            Padding(
                              padding: const EdgeInsets.symmetric(horizontal: 8),
                              child: Text(
                                page.description,
                                textAlign: TextAlign.center,
                                style: TextStyle(
                                  fontSize: 15,
                                  color: Colors.white.withOpacity(0.6),
                                  height: 1.5,
                                ),
                              ),
                            ),
                        ],
                      ),
                    );
                  },
                ),
              ),

              // Dots + Button
              Padding(
                padding: const EdgeInsets.all(32),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    // Dots
                    Row(
                      children: List.generate(
                        _pages.length,
                        (index) => AnimatedContainer(
                          duration: const Duration(milliseconds: 300),
                          margin: const EdgeInsets.only(right: 8),
                          width: _currentPage == index ? 24 : 8,
                          height: 8,
                          decoration: BoxDecoration(
                            color: _currentPage == index
                                ? AppTheme.primary
                                : Colors.white.withOpacity(0.2),
                            borderRadius: BorderRadius.circular(4),
                          ),
                        ),
                      ),
                    ),

                    // Next/Start button
                    FilledButton(
                      onPressed: () {
                        if (_currentPage == _pages.length - 1) {
                          _completeOnboarding();
                        } else {
                          _pageController.nextPage(
                            duration: const Duration(milliseconds: 300),
                            curve: Curves.easeInOut,
                          );
                        }
                      },
                      style: FilledButton.styleFrom(
                        backgroundColor: AppTheme.primary,
                        padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 16),
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                      ),
                      child: Row(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          Text(
                            _currentPage == _pages.length - 1 ? 'Comenzar' : 'Siguiente',
                            style: const TextStyle(fontSize: 16),
                          ),
                          const SizedBox(width: 8),
                          Icon(
                            _currentPage == _pages.length - 1
                                ? Icons.check
                                : Icons.arrow_forward,
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _OnboardingPage {
  final IconData icon;
  final String title;
  final String description;
  final Color color;
  final List<_FactorExplanation>? factors;

  _OnboardingPage({
    required this.icon,
    required this.title,
    required this.description,
    required this.color,
    this.factors,
  });
}

class _FactorExplanation {
  final String letter;
  final String name;
  final String explanation;

  const _FactorExplanation(this.letter, this.name, this.explanation);
}
