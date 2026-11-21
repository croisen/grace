import 'package:flutter/material.dart';

import '../widgets/deck_tile.dart';
import 'settings_screen.dart';
import 'language_select_screen.dart';

class DeckListScreen extends StatefulWidget {
  final ThemeMode appThemeMode;
  final Color appAccent;
  final bool terminalDark;
  final String terminalPalette;
  final List<Color> primaries;
  final void Function({
    ThemeMode? appMode,
    Color? accent,
    bool? termDark,
    String? palette,
  })
  onSettingsChanged;

  const DeckListScreen({
    super.key,
    required this.appThemeMode,
    required this.appAccent,
    required this.terminalDark,
    required this.terminalPalette,
    required this.primaries,
    required this.onSettingsChanged,
  });

  @override
  State<DeckListScreen> createState() => _DeckListScreenState();
}

class _DeckListScreenState extends State<DeckListScreen> {
  final List<Map<String, dynamic>> _decks = [];

  void _openSettings() {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) => SettingsScreen(
          appThemeMode: widget.appThemeMode,
          appAccent: widget.appAccent,
          terminalDark: widget.terminalDark,
          terminalPalette: widget.terminalPalette,
          primaries: widget.primaries,
          onSettingsChanged: widget.onSettingsChanged,
        ),
      ),
    );
  }

  void _onAddPressed() {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) => LanguageSelectScreen(
          terminalDark: widget.terminalDark,
          terminalPalette: widget.terminalPalette,
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const SizedBox.shrink(),
        leading: IconButton(
          icon: const Icon(Icons.menu),
          onPressed: _openSettings,
        ),
        actions: [IconButton(icon: const Icon(Icons.search), onPressed: () {})],
      ),
      body: _decks.isEmpty
          ? const Center(
              child: Text(
                'No decks yet.\nTap + to create one.',
                textAlign: TextAlign.center,
                style: TextStyle(fontSize: 16, color: Colors.grey),
              ),
            )
          : ListView.separated(
              itemCount: _decks.length,
              separatorBuilder: (_, __) => const Divider(height: 1),
              itemBuilder: (context, index) {
                final deck = _decks[index];
                return DeckTile(
                  name: deck['name'] as String,
                  newCount: deck['new'] as int,
                  learnCount: deck['learn'] as int,
                  dueCount: deck['due'] as int,
                  onTap: () {},
                );
              },
            ),
      floatingActionButton: FloatingActionButton(
        onPressed: _onAddPressed,
        child: const Icon(Icons.add),
      ),
    );
  }
}
