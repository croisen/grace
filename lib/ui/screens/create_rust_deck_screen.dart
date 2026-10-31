import 'package:flutter/material.dart';

import 'drill_card_screen.dart';

class CreateRustDeckScreen extends StatefulWidget {
  final bool terminalDark;
  final String terminalPalette;

  const CreateRustDeckScreen({
    super.key,
    required this.terminalDark,
    required this.terminalPalette,
  });

  @override
  State<CreateRustDeckScreen> createState() => _CreateRustDeckScreenState();
}

class _CreateRustDeckScreenState extends State<CreateRustDeckScreen> {
  final Map<String, bool> _concepts = {
    'Variables': true,
    'If / Else If': true,
    'Match': true,
    'While': true,
    'For': true,
    'Loop': true,
    'Functions': true,
  };

  int _repetitions = 10;
  final _repsController = TextEditingController(text: '10');
  final _nameController = TextEditingController(text: 'Rust Deck');

  @override
  void dispose() {
    _repsController.dispose();
    _nameController.dispose();
    super.dispose();
  }

  void _createAndStart() {
    final selected = _concepts.entries
        .where((e) => e.value)
        .map((e) => e.key)
        .toList();

    if (selected.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Select at least one concept')),
      );
      return;
    }

    Navigator.pushReplacement(
      context,
      MaterialPageRoute(
        builder: (_) => DrillCardScreen(
          deckName: _nameController.text,
          totalCards: _repetitions,
          concepts: selected,
          terminalDark: widget.terminalDark,
          terminalPalette: widget.terminalPalette,
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('New Rust Deck')),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          TextField(
            controller: _nameController,
            decoration: const InputDecoration(
              labelText: 'Deck name',
              border: OutlineInputBorder(),
            ),
          ),
          const SizedBox(height: 24),
          const Text(
            'Concepts to include',
            style: TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
          ),
          const SizedBox(height: 8),
          ..._concepts.keys.map((name) {
            return CheckboxListTile(
              title: Text(name),
              value: _concepts[name],
              onChanged: (v) => setState(() => _concepts[name] = v ?? false),
              controlAffinity: ListTileControlAffinity.leading,
              contentPadding: EdgeInsets.zero,
            );
          }),
          const SizedBox(height: 16),
          const Divider(),
          const SizedBox(height: 8),
          const Text(
            'How many cards / repetitions',
            style: TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
          ),
          const SizedBox(height: 12),
          TextField(
            controller: _repsController,
            keyboardType: TextInputType.number,
            decoration: const InputDecoration(
              labelText: 'Number of cards',
              border: OutlineInputBorder(),
            ),
            onChanged: (v) {
              final p = int.tryParse(v);
              if (p != null && p > 0) setState(() => _repetitions = p);
            },
          ),
          const SizedBox(height: 32),
          SizedBox(
            width: double.infinity,
            height: 48,
            child: ElevatedButton.icon(
              onPressed: _createAndStart,
              icon: const Icon(Icons.play_arrow),
              label: const Text('Create & Start'),
            ),
          ),
        ],
      ),
    );
  }
}
