import 'package:flutter/material.dart';
import '../app.dart';
import '../config/theme.dart';
import '../models/test_info.dart';
import '../services/api_service.dart';
import '../widgets/test_card.dart';
import 'test_screen.dart';
import 'hd_form_screen.dart';
import 'history_screen.dart';
import 'settings_screen.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> with SingleTickerProviderStateMixin {
  final ApiService _api = ApiService();
  List<TestInfo> _tests = [];
  bool _loading = true;
  late AnimationController _animController;

  @override
  void initState() {
    super.initState();
    _animController = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 800),
    )..forward();
    _loadTests();
  }

  @override
  void dispose() {
    _animController.dispose();
    super.dispose();
  }

  Future<void> _loadTests() async {
    try {
      final tests = await _api.getTests();
      setState(() {
        _tests = tests;
        _loading = false;
      });
    } catch (e) {
      setState(() => _loading = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);

    return Scaffold(
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [AppTheme.surface, AppTheme.surfaceVariant, AppTheme.surface],
          ),
        ),
        child: SafeArea(
          child: Column(
            children: [
              // AppBar
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Text(
                      l10n?.translate('appTitle') ?? 'PsicoMetrics',
                      style: const TextStyle(
                        fontSize: 20,
                        fontWeight: FontWeight.bold,
                        color: Colors.white,
                      ),
                    ),
                    Row(
                      children: [
                        // History button
                        IconButton(
                          icon: const Icon(Icons.history, size: 24),
                          onPressed: () => Navigator.push(context, MaterialPageRoute(
                            builder: (_) => const HistoryScreen(),
                          )),
                        ),
                        // Settings button
                        IconButton(
                          icon: const Icon(Icons.settings, size: 24),
                          onPressed: () => Navigator.push(context, MaterialPageRoute(
                            builder: (_) => const SettingsScreen(),
                          )),
                        ),
                        // Language switcher
                        _LanguageSwitcher(),
                      ],
                    ),
                  ],
                ),
              ),

              // Hero + Grid
              Expanded(
                child: SingleChildScrollView(
                  child: Column(
                    children: [
                      const SizedBox(height: 24),

                      // Animated title
                      FadeTransition(
                        opacity: CurvedAnimation(
                          parent: _animController,
                          curve: const Interval(0, 0.5),
                        ),
                        child: SlideTransition(
                          position: Tween<Offset>(
                            begin: const Offset(0, 0.2),
                            end: Offset.zero,
                          ).animate(CurvedAnimation(
                            parent: _animController,
                            curve: const Interval(0, 0.5, curve: Curves.easeOut),
                          )),
                          child: Column(
                            children: [
                              // Logo
                              Container(
                                width: 80,
                                height: 80,
                                decoration: BoxDecoration(
                                  gradient: const LinearGradient(
                                    colors: [AppTheme.primary, AppTheme.tertiary],
                                  ),
                                  borderRadius: BorderRadius.circular(20),
                                  boxShadow: [
                                    BoxShadow(
                                      color: AppTheme.primary.withOpacity(0.4),
                                      blurRadius: 20,
                                      spreadRadius: 2,
                                    ),
                                  ],
                                ),
                                child: const Icon(Icons.psychology, size: 44, color: Colors.white),
                              ),
                              const SizedBox(height: 16),
                              ShaderMask(
                                shaderCallback: (bounds) => const LinearGradient(
                                  colors: [AppTheme.primary, AppTheme.tertiary],
                                ).createShader(bounds),
                                child: Text(
                                  l10n?.translate('appTitle') ?? 'PsicoMetrics',
                                  style: const TextStyle(
                                    fontSize: 36,
                                    fontWeight: FontWeight.bold,
                                    color: Colors.white,
                                  ),
                                ),
                              ),
                              const SizedBox(height: 4),
                              Text(
                                l10n?.translate('appSubtitle') ?? '',
                                style: const TextStyle(fontSize: 16, color: Colors.white70),
                              ),
                              const SizedBox(height: 8),
                              Padding(
                                padding: const EdgeInsets.symmetric(horizontal: 32),
                                child: Text(
                                  l10n?.translate('appDescription') ?? '',
                                  textAlign: TextAlign.center,
                                  style: const TextStyle(color: Colors.white38, fontSize: 13),
                                ),
                              ),
                            ],
                          ),
                        ),
                      ),

                      const SizedBox(height: 32),

                      // Test Grid
                      if (_loading)
                        const Padding(
                          padding: EdgeInsets.all(32),
                          child: CircularProgressIndicator(color: AppTheme.primary),
                        )
                      else
                        Padding(
                          padding: const EdgeInsets.symmetric(horizontal: 16),
                          child: GridView.builder(
                            shrinkWrap: true,
                            physics: const NeverScrollableScrollPhysics(),
                            gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
                              crossAxisCount: 2,
                              mainAxisSpacing: 12,
                              crossAxisSpacing: 12,
                              childAspectRatio: 0.85,
                            ),
                            itemCount: _tests.length,
                            itemBuilder: (context, index) {
                              final test = _tests[index];
                              return FadeTransition(
                                opacity: CurvedAnimation(
                                  parent: _animController,
                                  curve: Interval(0.2 + index * 0.1, 1.0),
                                ),
                                child: SlideTransition(
                                  position: Tween<Offset>(
                                    begin: const Offset(0, 0.3),
                                    end: Offset.zero,
                                  ).animate(CurvedAnimation(
                                    parent: _animController,
                                    curve: Interval(0.2 + index * 0.1, 1.0, curve: Curves.easeOut),
                                  )),
                                  child: TestCard(
                                    test: test,
                                    onTap: () => _navigateToTest(test),
                                  ),
                                ),
                              );
                            },
                          ),
                        ),

                      const SizedBox(height: 32),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  void _navigateToTest(TestInfo test) {
    if (test.testMode == 'birth_data') {
      Navigator.push(context, MaterialPageRoute(
        builder: (_) => HdFormScreen(testType: test.testType),
      ));
    } else {
      Navigator.push(context, MaterialPageRoute(
        builder: (_) => TestScreen(testType: test.testType),
      ));
    }
  }
}

class _LanguageSwitcher extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    final currentLang = l10n?.locale.languageCode ?? 'es';

    return GestureDetector(
      onTap: () {
        final newLang = currentLang == 'es' ? const Locale('en', '') : const Locale('es', '');
        PsicoMetricsApp.setLocale(context, newLang);
      },
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
        decoration: BoxDecoration(
          color: Colors.white.withOpacity(0.1),
          borderRadius: BorderRadius.circular(8),
          border: Border.all(color: Colors.white.withOpacity(0.2)),
        ),
        child: Text(
          currentLang == 'es' ? 'EN' : 'ES',
          style: const TextStyle(fontSize: 12, fontWeight: FontWeight.w600),
        ),
      ),
    );
  }
}
