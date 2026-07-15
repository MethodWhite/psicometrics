import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import '../config/theme.dart';
import '../services/api_service.dart';

class AccountScreen extends StatefulWidget {
  const AccountScreen({super.key});

  @override
  State<AccountScreen> createState() => _AccountScreenState();
}

class _AccountScreenState extends State<AccountScreen> {
  final _emailController = TextEditingController();
  final _api = ApiService();
  bool _loading = false;
  String? _accountId;
  String? _accountEmail;
  List<Map<String, dynamic>> _results = [];
  String? _error;

  @override
  void initState() {
    super.initState();
    _loadSavedAccount();
  }

  Future<void> _loadSavedAccount() async {
    final prefs = await SharedPreferences.getInstance();
    final id = prefs.getString('account_id');
    if (id != null) {
      setState(() => _loading = true);
      try {
        final acc = await _api.getAccount(id);
        setState(() {
          _accountId = acc['id'] as String?;
          _accountEmail = acc['email'] as String?;
          _loading = false;
        });
        _loadResults();
      } catch (_) {
        setState(() => _loading = false);
      }
    }
  }

  Future<void> _register() async {
    final email = _emailController.text.trim();
    if (email.isEmpty) return;

    setState(() {
      _loading = true;
      _error = null;
    });

    try {
      final acc = await _api.registerAccount(email);
      final prefs = await SharedPreferences.getInstance();
      await prefs.setString('account_id', acc['id'] as String);

      setState(() {
        _accountId = acc['id'] as String?;
        _accountEmail = acc['email'] as String?;
        _loading = false;
      });
      _loadResults();
    } catch (e) {
      setState(() {
        _error = 'Error: $e';
        _loading = false;
      });
    }
  }

  Future<void> _loadResults() async {
    if (_accountId == null) return;
    try {
      final data = await _api.getResults(_accountId!);
      setState(() {
        _results = data.cast<Map<String, dynamic>>();
      });
    } catch (_) {}
  }

  Future<void> _disconnect() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.remove('account_id');
    setState(() {
      _accountId = null;
      _accountEmail = null;
      _results = [];
      _emailController.clear();
    });
  }

  String _formatTimestamp(int ts) {
    final dt = DateTime.fromMillisecondsSinceEpoch(ts * 1000);
    return '${dt.day}/${dt.month}/${dt.year} ${dt.hour.toString().padLeft(2, '0')}:${dt.minute.toString().padLeft(2, '0')}';
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Mi Cuenta'),
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator())
          : SingleChildScrollView(
              padding: const EdgeInsets.all(16),
              child: _accountId != null ? _buildAccountView() : _buildRegisterForm(),
            ),
    );
  }

  Widget _buildRegisterForm() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const SizedBox(height: 32),
        Icon(Icons.account_circle, size: 80, color: AppTheme.primary.withOpacity(0.7)),
        const SizedBox(height: 24),
        const Text(
          'Guarda tus resultados',
          textAlign: TextAlign.center,
          style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
        ),
        const SizedBox(height: 8),
        Text(
          'Registrá tu correo para guardar todos tus resultados y ver tu evolución en el tiempo.',
          textAlign: TextAlign.center,
          style: TextStyle(color: Colors.white70, fontSize: 14),
        ),
        const SizedBox(height: 24),
        TextField(
          controller: _emailController,
          decoration: InputDecoration(
            labelText: 'Correo electrónico',
            hintText: 'tu@email.com',
            prefixIcon: const Icon(Icons.email),
            border: OutlineInputBorder(borderRadius: BorderRadius.circular(12)),
          ),
          keyboardType: TextInputType.emailAddress,
        ),
        if (_error != null) ...[
          const SizedBox(height: 8),
          Text(_error!, style: const TextStyle(color: Colors.red)),
        ],
        const SizedBox(height: 16),
        FilledButton.icon(
          onPressed: _register,
          icon: const Icon(Icons.person_add),
          label: const Text('Registrarse / Iniciar sesión'),
          style: FilledButton.styleFrom(padding: const EdgeInsets.symmetric(vertical: 16)),
        ),
        const SizedBox(height: 32),
        Text(
          'Si ya tenés una cuenta, ingresá el mismo correo para continuar.',
          textAlign: TextAlign.center,
          style: TextStyle(color: Colors.white54, fontSize: 12),
        ),
      ],
    );
  }

  Widget _buildAccountView() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Card(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              children: [
                Icon(Icons.account_circle, size: 64, color: AppTheme.primary),
                const SizedBox(height: 12),
                Text(_accountEmail ?? '', style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
                const SizedBox(height: 8),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
                  decoration: BoxDecoration(
                    color: Colors.white10,
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Text(
                    'ID: ${_accountId!.substring(0, 8)}...',
                    style: const TextStyle(fontSize: 12, color: Colors.white54),
                  ),
                ),
                const SizedBox(height: 8),
                Text('${_results.length} resultados guardados',
                    style: const TextStyle(color: Colors.white70)),
              ],
            ),
          ),
        ),
        const SizedBox(height: 8),
        Text('ID completo (copiar para otro dispositivo):',
            style: const TextStyle(fontSize: 12, color: Colors.white54)),
        const SizedBox(height: 4),
        SelectableText(_accountId!, style: const TextStyle(fontSize: 11, color: Colors.white38)),
        const SizedBox(height: 16),

        if (_results.isNotEmpty) ...[
          Text('Resultados guardados', style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold)),
          const SizedBox(height: 8),
          ..._results.map((r) => Card(
            child: ListTile(
              leading: const Icon(Icons.assessment, color: AppTheme.primary),
              title: Text(r['test_type'] as String? ?? 'Test'),
              subtitle: Text(_formatTimestamp((r['created_at'] as num).toInt())),
              trailing: const Icon(Icons.chevron_right),
            ),
          )),
        ] else ...[
          const Card(
            child: Padding(
              padding: EdgeInsets.all(24),
              child: Center(child: Text('Todavía no hay resultados guardados', style: TextStyle(color: Colors.white54))),
            ),
          ),
        ],

        const SizedBox(height: 24),
        OutlinedButton.icon(
          onPressed: _disconnect,
          icon: const Icon(Icons.logout),
          label: const Text('Desconectar'),
          style: OutlinedButton.styleFrom(foregroundColor: Colors.red),
        ),
      ],
    );
  }

  @override
  void dispose() {
    _emailController.dispose();
    super.dispose();
  }
}
