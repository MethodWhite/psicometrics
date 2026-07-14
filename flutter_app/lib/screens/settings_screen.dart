import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import '../app.dart';
import '../config/theme.dart';

class SettingsScreen extends StatefulWidget {
  const SettingsScreen({super.key});

  @override
  State<SettingsScreen> createState() => _SettingsScreenState();
}

class _SettingsScreenState extends State<SettingsScreen> {
  bool _notifications = true;
  String _language = 'es';

  @override
  void initState() {
    super.initState();
    _loadSettings();
  }

  Future<void> _loadSettings() async {
    final prefs = await SharedPreferences.getInstance();
    setState(() {
      _notifications = prefs.getBool('notifications') ?? true;
      _language = prefs.getString('language') ?? 'es';
    });
  }

  Future<void> _saveSetting(String key, dynamic value) async {
    final prefs = await SharedPreferences.getInstance();
    if (value is bool) {
      await prefs.setBool(key, value);
    } else if (value is String) {
      await prefs.setString(key, value);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Configuración'),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => Navigator.pop(context),
        ),
      ),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          // Language
          _buildSection(
            title: 'Idioma',
            child: SegmentedButton<String>(
              segments: const [
                ButtonSegment(value: 'es', label: Text('Español')),
                ButtonSegment(value: 'en', label: Text('English')),
              ],
              selected: {_language},
              onSelectionChanged: (values) {
                setState(() => _language = values.first);
                _saveSetting('language', _language);
                final newLang = _language == 'es' ? const Locale('es') : const Locale('en');
                PsicoMetricsApp.setLocale(context, newLang);
              },
            ),
          ),
          const SizedBox(height: 24),

          // Notifications
          _buildSection(
            title: 'Notificaciones',
            child: SwitchListTile(
              title: const Text('Recordatorios de test'),
              subtitle: const Text('Recibí recordatorios para completar tests'),
              value: _notifications,
              onChanged: (value) {
                setState(() => _notifications = value);
                _saveSetting('notifications', value);
              },
              activeColor: AppTheme.primary,
            ),
          ),
          const SizedBox(height: 24),

          // Data
          _buildSection(
            title: 'Datos',
            child: Column(
              children: [
                ListTile(
                  leading: const Icon(Icons.delete_outline, color: Colors.red),
                  title: const Text('Limpiar historial'),
                  subtitle: const Text('Eliminar todos los resultados guardados'),
                  onTap: () => _showClearHistoryDialog(),
                ),
                const Divider(),
                ListTile(
                  leading: const Icon(Icons.info_outline),
                  title: const Text('Versión'),
                  subtitle: const Text('1.0.0'),
                ),
              ],
            ),
          ),
          const SizedBox(height: 24),

          // About
          _buildSection(
            title: 'Acerca de',
            child: Column(
              children: [
                ListTile(
                  leading: const Icon(Icons.science),
                  title: const Text('Base científica'),
                  subtitle: const Text('Conocé más sobre los tests utilizados'),
                  trailing: const Icon(Icons.chevron_right),
                  onTap: () => Navigator.push(context, MaterialPageRoute(
                    builder: (_) => const AboutTestsScreen(),
                  )),
                ),
                const Divider(),
                ListTile(
                  leading: const Icon(Icons.security),
                  title: const Text('Seguridad y privacidad'),
                  subtitle: const Text('Tus datos están seguros'),
                  trailing: const Icon(Icons.chevron_right),
                  onTap: () => _showPrivacyDialog(),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSection({required String title, required Widget child}) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          title,
          style: TextStyle(
            fontSize: 14,
            fontWeight: FontWeight.w600,
            color: Colors.white.withOpacity(0.6),
          ),
        ),
        const SizedBox(height: 8),
        Card(
          child: Padding(
            padding: const EdgeInsets.all(4),
            child: child,
          ),
        ),
      ],
    );
  }

  void _showClearHistoryDialog() {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Limpiar historial'),
        content: const Text('¿Estás seguro de que querés eliminar todo el historial de resultados?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Cancelar'),
          ),
          FilledButton(
            onPressed: () async {
              final prefs = await SharedPreferences.getInstance();
              await prefs.remove('test_history');
              if (mounted) Navigator.pop(context);
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(content: Text('Historial eliminado')),
              );
            },
            style: FilledButton.styleFrom(backgroundColor: Colors.red),
            child: const Text('Eliminar'),
          ),
        ],
      ),
    );
  }

  void _showPrivacyDialog() {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Seguridad y privacidad'),
        content: const SingleChildScrollView(
          child: Text(
            'PsicoMetrics respeta tu privacidad:\n\n'
            '• No recopilamos datos personales identificables\n'
            '• Los resultados se guardan solo en tu dispositivo\n'
            '• No enviamos datos a servidores third-party\n'
            '• Las conexiones al backend son cifradas\n'
            '• Podés borrar tu historial en cualquier momento\n\n'
            'Los tests son solo para fines educativos y no sustituyen un diagnóstico profesional.',
          ),
        ),
        actions: [
          FilledButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Entendido'),
          ),
        ],
      ),
    );
  }
}

class AboutTestsScreen extends StatelessWidget {
  const AboutTestsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Base Científica'),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => Navigator.pop(context),
        ),
      ),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          _buildTestInfo(
            icon: '🧠',
            name: 'Big Five / OCEAN',
            standard: 'IPIP-NEO-120',
            source: 'International Personality Item Pool',
            description: 'El estándar científico más validado en psicología de la personalidad. Mide los Cinco Grandes Rasgos con 120 ítems, 12 por factor. Escala Likert 1-5.',
            reliability: 'α > 0.85 para todos los factores',
            reference: 'John, O. P., & Srivastava, S. (1999)',
          ),
          _buildTestInfo(
            icon: '🎭',
            name: 'MBTI — Myers-Briggs',
            standard: 'Myers-Briggs Type Indicator',
            source: 'Myers (1962)',
            description: 'Indicador de tipo de personalidad con 4 dicotomías: E/I, S/N, T/F, J/P. 72 ítems de elección forzada A/B.',
            reliability: 'Consistencia test-retest: 75-90%',
            reference: 'Myers, I. B. (1962)',
          ),
          _buildTestInfo(
            icon: '🔵',
            name: 'Enneagrama',
            standard: 'RHETI',
            source: 'Riso-Hudson',
            description: '9 tipos de personalidad con alas. 81 ítems en escala Likert 4 puntos. Identifica tipo dominante e influencia del ala.',
            reliability: 'Consistencia interna: α = 0.83',
            reference: 'Riso, D. R., & Hudson, R. (1999)',
          ),
          _buildTestInfo(
            icon: '📊',
            name: 'DISC',
            standard: 'Marston DISC Theory',
            source: 'William Moulton Marston',
            description: 'Perfil de comportamiento con 4 dimensiones: Dominancia, Influencia, Estabilidad, Conciencia. 28 ítems.',
            reliability: 'Ampliamente utilizado en desarrollo organizacional',
            reference: 'Marston, W. M. (1928)',
          ),
          _buildTestInfo(
            icon: '🌑',
            name: 'Triada Oscura — SD3',
            standard: 'Short Dark Triad',
            source: 'Jones & Paulhus (2014)',
            description: 'Mide Maquiavelismo, Narcisismo y Psicopatía con 27 ítems. Incluye score compuesto Dark Core y nivel de riesgo.',
            reliability: 'α = 0.82-0.90 por rasgo',
            reference: 'Jones, D. N., & Paulhus, D. L. (2014)',
          ),
          _buildTestInfo(
            icon: '⭐',
            name: 'Diseño Humano',
            standard: 'Human Design System',
            source: 'Ra Uru Hu',
            description: 'Sistema basado en astrología, I Ching, Centro Kabbalístico y Chakras. Calcula Tipo, Estrategia, Autoridad y Perfil.',
            reliability: 'Sistema esotérico — noValidado científicamente',
            reference: 'Ra Uru Hu (1987)',
          ),
        ],
      ),
    );
  }

  Widget _buildTestInfo({
    required String icon,
    required String name,
    required String standard,
    required String source,
    required String description,
    required String reliability,
    required String reference,
  }) {
    return Card(
      margin: const EdgeInsets.only(bottom: 16),
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Text(icon, style: const TextStyle(fontSize: 28)),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(name, style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
                      Text(standard, style: TextStyle(fontSize: 13, color: Colors.white.withOpacity(0.5))),
                    ],
                  ),
                ),
              ],
            ),
            const SizedBox(height: 12),
            Text(description, style: const TextStyle(color: Colors.white70, height: 1.5)),
            const SizedBox(height: 12),
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: Colors.white.withOpacity(0.05),
                borderRadius: BorderRadius.circular(8),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Icon(Icons.verified, size: 16, color: AppTheme.primary),
                      const SizedBox(width: 8),
                      Text(reliability, style: const TextStyle(fontSize: 12, color: Colors.white60)),
                    ],
                  ),
                  const SizedBox(height: 4),
                  Text(reference, style: TextStyle(fontSize: 11, color: Colors.white.withOpacity(0.4))),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
