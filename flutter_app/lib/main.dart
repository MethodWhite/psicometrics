import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'config/theme.dart';

import 'screens/splash_screen.dart';
import 'screens/onboarding_screen.dart';
import 'screens/home_screen.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(const PsicoMetricsApp());
}

class PsicoMetricsApp extends StatefulWidget {
  const PsicoMetricsApp({super.key});

  static void setLocale(BuildContext context, Locale locale) {
    _PsicoMetricsAppState? state = context.findAncestorStateOfType<_PsicoMetricsAppState>();
    state?.setLocale(locale);
  }

  @override
  State<PsicoMetricsApp> createState() => _PsicoMetricsAppState();
}

class _PsicoMetricsAppState extends State<PsicoMetricsApp> {
  Locale _locale = const Locale('es', '');
  bool _showSplash = true;
  bool _onboardingComplete = false;
  bool _loading = true;

  void setLocale(Locale locale) {
    setState(() => _locale = locale);
  }

  @override
  void initState() {
    super.initState();
    _init();
  }

  Future<void> _init() async {
    final prefs = await SharedPreferences.getInstance();
    _onboardingComplete = prefs.getBool('onboarding_complete') ?? false;
    final lang = prefs.getString('language') ?? 'es';
    _locale = Locale(lang);

    // Show splash for 2 seconds
    await Future.delayed(const Duration(seconds: 2));
    setState(() {
      _showSplash = false;
      _loading = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    if (_loading) {
      return MaterialApp(
        debugShowCheckedModeBanner: false,
        theme: AppTheme.darkTheme,
        home: const SplashScreen(),
      );
    }

    if (_showSplash) {
      return MaterialApp(
        debugShowCheckedModeBanner: false,
        theme: AppTheme.darkTheme,
        home: const SplashScreen(),
      );
    }

    return MaterialApp(
      title: 'PsicoMetrics',
      debugShowCheckedModeBanner: false,
      theme: AppTheme.darkTheme,
      locale: _locale,
      supportedLocales: const [
        Locale('es', ''),
        Locale('en', ''),
      ],
      localizationsDelegates: const [
        AppLocalizations.delegate,
        GlobalMaterialLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
      ],
      home: _onboardingComplete ? const HomeScreen() : const OnboardingScreen(),
    );
  }
}

class AppLocalizations {
  static const LocalizationsDelegate<AppLocalizations> delegate = _AppLocalizationsDelegate();

  final Locale locale;
  AppLocalizations(this.locale);

  static AppLocalizations? of(BuildContext context) {
    return Localizations.of<AppLocalizations>(context, AppLocalizations);
  }

  static const Map<String, Map<String, String>> _strings = {
    'es': {
      'appTitle': 'PsicoMetrics',
      'appSubtitle': 'Tests de Personalidad Oficiales',
      'appDescription': 'Evaluaciones psicológicas basadas en estándares científicos validados',
      'startTest': 'Comenzar Test',
      'back': 'Volver',
      'next': 'Siguiente',
      'previous': 'Anterior',
      'submit': 'Enviar Respuestas',
      'loading': 'Cargando...',
      'error': 'Ha ocurrido un error',
      'questionOf': 'Pregunta {{current}} de {{total}}',
      'exitTest': '¿Salir del test?',
      'exitConfirm': '¿Querés volver al menú principal?',
      'exitLoseProgress': 'Perderás el progreso actual. ¿Querés salir?',
      'cancel': 'Cancelar',
      'exit': 'Salir',
      'retry': 'Reintentar',
      'results': 'Resultados',
      'about': 'Acerca de',
      'yourProfile': 'Tu Perfil',
      'score': 'Puntuación',
      'percentile': 'Percentil',
      'dominant': 'Dominante',
      'wing': 'Ala',
      'primary': 'Primario',
      'secondary': 'Secundario',
      'retake': 'Volver a hacer el test',
      'shareResult': 'Compartir resultado',
      'copyResult': 'Copiar resultado',
      'saved': 'Resultado guardado',
      'birthDate': 'Fecha de nacimiento',
      'birthTime': 'Hora de nacimiento',
      'birthLocation': 'Lugar de nacimiento',
      'timeNote': 'La hora exacta es importante para un cálculo preciso',
      'locationPlaceholder': 'Ciudad, País',
      'big_five': 'Big Five / OCEAN',
      'mbti': 'MBTI — Myers-Briggs',
      'enneagram': 'Enneagrama',
      'disc': 'DISC',
      'dark_triad': 'Triada Oscura — SD3',
      'human_design': 'Diseño Humano',
      'big_five_desc': 'El estándar científico más validado. 120 ítems del IPIP-NEO.',
      'mbti_desc': 'Indicador de tipo con 4 dicotomías. El más utilizado en empresas.',
      'enneagram_desc': '9 tipos y alas. Basado en el Enneagrama de Riso-Hudson.',
      'disc_desc': 'Perfil de comportamiento con 4 dimensiones.',
      'dark_triad_desc': 'SD3 de Jones & Paulhus. Maquiavelismo, Narcisismo, Psicopatía.',
      'human_design_desc': 'Basado en astrología e I Ching. Calcula tu Tipo desde tu nacimiento.',
      'big_five_items': '120 preguntas',
      'big_five_time': '~25 min',
      'mbti_items': '72 preguntas',
      'mbti_time': '~15 min',
      'enneagram_items': '81 preguntas',
      'enneagram_time': '~20 min',
      'disc_items': '28 preguntas',
      'disc_time': '~8 min',
      'dark_triad_items': '27 preguntas',
      'dark_triad_time': '~8 min',
      'human_design_items': 'Datos de nacimiento',
      'human_design_time': '~3 min',
      'strategy': 'Estrategia',
      'authority': 'Autoridad',
      'profile': 'Perfil',
      'bodyGraph': 'Body Graph',
      'personalityGates': 'Gates Personalidad',
      'designGates': 'Gates Diseño',
      'riskMinimal': 'Mínimo',
      'riskLow': 'Bajo',
      'riskModerate': 'Moderado',
      'riskHigh': 'Alto',
      'darkCore': 'Dark Core Score',
      'history': 'Historial',
      'settings': 'Configuración',
    },
    'en': {
      'appTitle': 'PsicoMetrics',
      'appSubtitle': 'Official Personality Tests',
      'appDescription': 'Psychological assessments based on validated scientific standards',
      'startTest': 'Start Test',
      'back': 'Back',
      'next': 'Next',
      'previous': 'Previous',
      'submit': 'Submit Answers',
      'loading': 'Loading...',
      'error': 'An error occurred',
      'questionOf': 'Question {{current}} of {{total}}',
      'exitTest': 'Exit test?',
      'exitConfirm': 'Return to main menu?',
      'exitLoseProgress': 'You will lose your progress. Exit anyway?',
      'cancel': 'Cancel',
      'exit': 'Exit',
      'retry': 'Retry',
      'results': 'Results',
      'about': 'About',
      'yourProfile': 'Your Profile',
      'score': 'Score',
      'percentile': 'Percentile',
      'dominant': 'Dominant',
      'wing': 'Wing',
      'primary': 'Primary',
      'secondary': 'Secondary',
      'retake': 'Retake Test',
      'shareResult': 'Share result',
      'copyResult': 'Copy result',
      'saved': 'Result saved',
      'birthDate': 'Birth date',
      'birthTime': 'Birth time',
      'birthLocation': 'Birth location',
      'timeNote': 'Exact time is important for accurate calculation',
      'locationPlaceholder': 'City, Country',
      'big_five': 'Big Five / OCEAN',
      'mbti': 'MBTI — Myers-Briggs',
      'enneagram': 'Enneagram',
      'disc': 'DISC',
      'dark_triad': 'Dark Triad — SD3',
      'human_design': 'Human Design',
      'big_five_desc': 'The most validated scientific standard. 120 IPIP-NEO items.',
      'mbti_desc': 'Personality type indicator with 4 dichotomies. Most used in companies.',
      'enneagram_desc': '9 types and wings. Based on the Riso-Hudson Enneagram.',
      'disc_desc': 'Behavioral profile with 4 dimensions.',
      'dark_triad_desc': 'SD3 by Jones & Paulhus. Machiavellianism, Narcissism, Psychopathy.',
      'human_design_desc': 'Based on astrology and I Ching. Calculate your Type from birth.',
      'big_five_items': '120 questions',
      'big_five_time': '~25 min',
      'mbti_items': '72 questions',
      'mbti_time': '~15 min',
      'enneagram_items': '81 questions',
      'enneagram_time': '~20 min',
      'disc_items': '28 questions',
      'disc_time': '~8 min',
      'dark_triad_items': '27 questions',
      'dark_triad_time': '~8 min',
      'human_design_items': 'Birth data',
      'human_design_time': '~3 min',
      'strategy': 'Strategy',
      'authority': 'Authority',
      'profile': 'Profile',
      'bodyGraph': 'Body Graph',
      'personalityGates': 'Personality Gates',
      'designGates': 'Design Gates',
      'riskMinimal': 'Minimal',
      'riskLow': 'Low',
      'riskModerate': 'Moderate',
      'riskHigh': 'High',
      'darkCore': 'Dark Core Score',
      'history': 'History',
      'settings': 'Settings',
    },
  };

  String translate(String key) {
    return _strings[locale.languageCode]?[key] ?? key;
  }
}

class _AppLocalizationsDelegate extends LocalizationsDelegate<AppLocalizations> {
  const _AppLocalizationsDelegate();

  @override
  bool isSupported(Locale locale) => ['es', 'en'].contains(locale.languageCode);

  @override
  Future<AppLocalizations> load(Locale locale) async => AppLocalizations(locale);

  @override
  bool shouldReload(_AppLocalizationsDelegate old) => false;
}
