import 'package:flutter/material.dart';
import '../app.dart';
import '../config/theme.dart';
import '../services/api_service.dart';
import 'results_screen.dart';

class HdFormScreen extends StatefulWidget {
  final String testType;
  const HdFormScreen({super.key, required this.testType});

  @override
  State<HdFormScreen> createState() => _HdFormScreenState();
}

class _HdFormScreenState extends State<HdFormScreen> {
  final ApiService _api = ApiService();
  DateTime? _birthDate;
  TimeOfDay? _birthTime;
  final _locationController = TextEditingController();
  bool _submitting = false;

  @override
  void dispose() {
    _locationController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);

    return Scaffold(
      appBar: AppBar(
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => Navigator.pop(context),
        ),
        title: Text(l10n?.translate('human_design') ?? 'Diseño Humano'),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(
              l10n?.translate('human_design') ?? '',
              style: const TextStyle(fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white),
            ),
            const SizedBox(height: 8),
            Text(
              l10n?.translate('human_design_desc') ?? '',
              style: const TextStyle(color: Colors.white54),
            ),
            const SizedBox(height: 32),

            // Birth Date
            _buildField(
              label: l10n?.translate('birthDate') ?? 'Fecha de nacimiento',
              child: InkWell(
                onTap: _pickDate,
                child: InputDecorator(
                  decoration: _inputDecoration(),
                  child: Text(
                    _birthDate != null
                        ? '${_birthDate!.day}/${_birthDate!.month}/${_birthDate!.year}'
                        : 'DD/MM/YYYY',
                    style: TextStyle(
                      color: _birthDate != null ? Colors.white : Colors.white38,
                    ),
                  ),
                ),
              ),
            ),
            const SizedBox(height: 16),

            // Birth Time
            _buildField(
              label: l10n?.translate('birthTime') ?? 'Hora de nacimiento',
              child: InkWell(
                onTap: _pickTime,
                child: InputDecorator(
                  decoration: _inputDecoration(),
                  child: Text(
                    _birthTime != null
                        ? '${_birthTime!.hour.toString().padLeft(2, '0')}:${_birthTime!.minute.toString().padLeft(2, '0')}'
                        : 'HH:MM',
                    style: TextStyle(
                      color: _birthTime != null ? Colors.white : Colors.white38,
                    ),
                  ),
                ),
              ),
            ),
            const SizedBox(height: 4),
            Text(
              l10n?.translate('timeNote') ?? '',
              style: const TextStyle(fontSize: 12, color: Colors.white30),
            ),
            const SizedBox(height: 16),

            // Birth Location
            _buildField(
              label: l10n?.translate('birthLocation') ?? 'Lugar de nacimiento',
              child: TextField(
                controller: _locationController,
                decoration: _inputDecoration().copyWith(
                  hintText: l10n?.translate('locationPlaceholder') ?? 'Ciudad, País',
                  hintStyle: const TextStyle(color: Colors.white30),
                ),
                style: const TextStyle(color: Colors.white),
              ),
            ),
            const SizedBox(height: 32),

            // Submit
            FilledButton(
              onPressed: _canSubmit ? _submit : null,
              style: FilledButton.styleFrom(
                backgroundColor: AppTheme.primary,
                padding: const EdgeInsets.symmetric(vertical: 16),
              ),
              child: _submitting
                  ? const SizedBox(
                      width: 20, height: 20,
                      child: CircularProgressIndicator(strokeWidth: 2, color: Colors.white),
                    )
                  : Text(l10n?.translate('submit') ?? 'Enviar'),
            ),
          ],
        ),
      ),
    );
  }

  bool get _canSubmit =>
      _birthDate != null && _birthTime != null && _locationController.text.isNotEmpty && !_submitting;

  Widget _buildField({required String label, required Widget child}) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(label, style: const TextStyle(fontSize: 14, color: Colors.white70)),
        const SizedBox(height: 8),
        child,
      ],
    );
  }

  InputDecoration _inputDecoration() {
    return InputDecoration(
      filled: true,
      fillColor: Colors.white.withOpacity(0.05),
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(12),
        borderSide: BorderSide(color: Colors.white.withOpacity(0.1)),
      ),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(12),
        borderSide: BorderSide(color: Colors.white.withOpacity(0.1)),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(12),
        borderSide: const BorderSide(color: AppTheme.primary),
      ),
      contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
    );
  }

  Future<void> _pickDate() async {
    final date = await showDatePicker(
      context: context,
      initialDate: DateTime(1990),
      firstDate: DateTime(1900),
      lastDate: DateTime.now(),
    );
    if (date != null) setState(() => _birthDate = date);
  }

  Future<void> _pickTime() async {
    final time = await showTimePicker(
      context: context,
      initialTime: const TimeOfDay(hour: 12, minute: 0),
    );
    if (time != null) setState(() => _birthTime = time);
  }

  Future<void> _submit() async {
    setState(() => _submitting = true);
    try {
      final lang = AppLocalizations.of(context)?.locale.languageCode ?? 'es';
      final dateStr = '${_birthDate!.year}-${_birthDate!.month.toString().padLeft(2, '0')}-${_birthDate!.day.toString().padLeft(2, '0')}';
      final timeStr = '${_birthTime!.hour.toString().padLeft(2, '0')}:${_birthTime!.minute.toString().padLeft(2, '0')}';

      final result = await _api.submitHumanDesign(
        dateStr, timeStr, _locationController.text, lang,
      );

      if (mounted) {
        Navigator.pushReplacement(context, MaterialPageRoute(
          builder: (_) => ResultsScreen(testType: widget.testType, result: result),
        ));
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Error: $e')),
        );
      }
    } finally {
      setState(() => _submitting = false);
    }
  }
}
