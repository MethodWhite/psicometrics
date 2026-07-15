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
        Locale('pt', ''),
        Locale('fr', ''),
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
    'pt': {
      'appTitle': 'PsicoMetrics',
      'appSubtitle': 'Testes de Personalidade Oficiais',
      'appDescription': 'Avaliações psicológicas baseadas em padrões científicos validados',
      'startTest': 'Iniciar Teste',
      'back': 'Voltar',
      'next': 'Próximo',
      'previous': 'Anterior',
      'submit': 'Enviar Respostas',
      'loading': 'Carregando...',
      'error': 'Ocorreu um erro',
      'questionOf': 'Pergunta {{current}} de {{total}}',
      'exitTest': 'Sair do teste?',
      'exitConfirm': 'Voltar ao menu principal?',
      'exitLoseProgress': 'Você perderá seu progresso atual. Deseja sair?',
      'cancel': 'Cancelar',
      'exit': 'Sair',
      'retry': 'Tentar novamente',
      'results': 'Resultados',
      'about': 'Sobre',
      'yourProfile': 'Seu Perfil',
      'score': 'Pontuação',
      'percentile': 'Percentil',
      'dominant': 'Dominante',
      'wing': 'Asa',
      'primary': 'Primário',
      'secondary': 'Secundário',
      'retake': 'Refazer o teste',
      'shareResult': 'Compartilhar resultado',
      'copyResult': 'Copiar resultado',
      'saved': 'Resultado salvo',
      'birthDate': 'Data de nascimento',
      'birthTime': 'Hora de nascimento',
      'birthLocation': 'Local de nascimento',
      'timeNote': 'A hora exata é importante para um cálculo preciso',
      'locationPlaceholder': 'Cidade, País',
      'big_five': 'Big Five / OCEAN',
      'mbti': 'MBTI — Myers-Briggs',
      'enneagram': 'Eneagrama',
      'disc': 'DISC',
      'dark_triad': 'Tríade Sombria — SD3',
      'human_design': 'Design Humano',
      'big_five_desc': 'O padrão científico mais validado. 120 itens do IPIP-NEO.',
      'mbti_desc': 'Indicador de tipo com 4 dicotomias. O mais usado em empresas.',
      'enneagram_desc': '9 tipos e asas. Baseado no Eneagrama de Riso-Hudson.',
      'disc_desc': 'Perfil comportamental com 4 dimensões.',
      'dark_triad_desc': 'SD3 de Jones & Paulhus. Maquiavelismo, Narcisismo, Psicopatia.',
      'human_design_desc': 'Baseado em astrologia e I Ching. Calcule seu Tipo a partir do nascimento.',
      'big_five_items': '120 perguntas',
      'big_five_time': '~25 min',
      'mbti_items': '72 perguntas',
      'mbti_time': '~15 min',
      'enneagram_items': '81 perguntas',
      'enneagram_time': '~20 min',
      'disc_items': '28 perguntas',
      'disc_time': '~8 min',
      'dark_triad_items': '27 perguntas',
      'dark_triad_time': '~8 min',
      'human_design_items': 'Dados de nascimento',
      'human_design_time': '~3 min',
      'strategy': 'Estratégia',
      'authority': 'Autoridade',
      'profile': 'Perfil',
      'bodyGraph': 'Body Graph',
      'personalityGates': 'Portas da Personalidade',
      'designGates': 'Portas do Design',
      'riskMinimal': 'Mínimo',
      'riskLow': 'Baixo',
      'riskModerate': 'Moderado',
      'riskHigh': 'Alto',
      'darkCore': 'Dark Core Score',
      'history': 'Histórico',
      'settings': 'Configurações',
    },
    'fr': {
      'appTitle': 'PsicoMetrics',
      'appSubtitle': 'Tests de Personnalité Officiels',
      'appDescription': 'Évaluations psychologiques basées sur des standards scientifiques validés',
      'startTest': 'Commencer le Test',
      'back': 'Retour',
      'next': 'Suivant',
      'previous': 'Précédent',
      'submit': 'Envoyer les Réponses',
      'loading': 'Chargement...',
      'error': 'Une erreur est survenue',
      'questionOf': 'Question {{current}} sur {{total}}',
      'exitTest': 'Quitter le test ?',
      'exitConfirm': 'Retour au menu principal ?',
      'exitLoseProgress': 'Vous perdrez votre progression actuelle. Voulez-vous quitter ?',
      'cancel': 'Annuler',
      'exit': 'Quitter',
      'retry': 'Réessayer',
      'results': 'Résultats',
      'about': 'À propos',
      'yourProfile': 'Votre Profil',
      'score': 'Score',
      'percentile': 'Centile',
      'dominant': 'Dominant',
      'wing': 'Aile',
      'primary': 'Primaire',
      'secondary': 'Secondaire',
      'retake': 'Refaire le test',
      'shareResult': 'Partager le résultat',
      'copyResult': 'Copier le résultat',
      'saved': 'Résultat enregistré',
      'birthDate': 'Date de naissance',
      'birthTime': 'Heure de naissance',
      'birthLocation': 'Lieu de naissance',
      'timeNote': 'L\'heure exacte est importante pour un calcul précis',
      'locationPlaceholder': 'Ville, Pays',
      'big_five': 'Big Five / OCEAN',
      'mbti': 'MBTI — Myers-Briggs',
      'enneagram': 'Ennéagramme',
      'disc': 'DISC',
      'dark_triad': 'Triade Sombre — SD3',
      'human_design': 'Design Humain',
      'big_five_desc': 'La norme scientifique la plus validée. 120 items IPIP-NEO.',
      'mbti_desc': 'Indicateur de type avec 4 dichotomies. Le plus utilisé en entreprise.',
      'enneagram_desc': '9 types et ailes. Basé sur l\'Ennéagramme de Riso-Hudson.',
      'disc_desc': 'Profil comportemental avec 4 dimensions.',
      'dark_triad_desc': 'SD3 de Jones & Paulhus. Machiavélisme, Narcissisme, Psychopathie.',
      'human_design_desc': 'Basé sur l\'astrologie et le I Ching. Calculez votre Type depuis la naissance.',
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
      'human_design_items': 'Données de naissance',
      'human_design_time': '~3 min',
      'strategy': 'Stratégie',
      'authority': 'Autorité',
      'profile': 'Profil',
      'bodyGraph': 'Body Graph',
      'personalityGates': 'Portes de la Personnalité',
      'designGates': 'Portes du Design',
      'riskMinimal': 'Minimal',
      'riskLow': 'Faible',
      'riskModerate': 'Modéré',
      'riskHigh': 'Élevé',
      'darkCore': 'Dark Core Score',
      'history': 'Historique',
      'settings': 'Paramètres',
    },
  };

  String translate(String key) {
    return _strings[locale.languageCode]?[key] ?? key;
  }
}

class _AppLocalizationsDelegate extends LocalizationsDelegate<AppLocalizations> {
  const _AppLocalizationsDelegate();

  @override
  bool isSupported(Locale locale) => ['es', 'en', 'pt', 'fr'].contains(locale.languageCode);

  @override
  Future<AppLocalizations> load(Locale locale) async => AppLocalizations(locale);

  @override
  bool shouldReload(_AppLocalizationsDelegate old) => false;
}
