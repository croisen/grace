import 'package:flutter/material.dart';

class SettingsScreen extends StatelessWidget {
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
  }) onSettingsChanged;

  const SettingsScreen({
    super.key,
    required this.appThemeMode,
    required this.appAccent,
    required this.terminalDark,
    required this.terminalPalette,
    required this.primaries,
    required this.onSettingsChanged,
  });

  static const List<String> palettes = [
    'Default', 'Gruvbox', 'Tokyo Night', 'Dracula',
    'Nord', 'Catppuccin', 'One Dark', 'Solarized',
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Settings')),
      body: ListView(
        children: [
          _sectionHeader('App UI'),
          RadioListTile<ThemeMode>(
            title: const Text('System'),
            value: ThemeMode.system,
            groupValue: appThemeMode,
            onChanged: (v) => onSettingsChanged(appMode: v),
          ),
          RadioListTile<ThemeMode>(
            title: const Text('Light'),
            value: ThemeMode.light,
            groupValue: appThemeMode,
            onChanged: (v) => onSettingsChanged(appMode: v),
          ),
          RadioListTile<ThemeMode>(
            title: const Text('Dark'),
            value: ThemeMode.dark,
            groupValue: appThemeMode,
            onChanged: (v) => onSettingsChanged(appMode: v),
          ),
          const Padding(
            padding: EdgeInsets.fromLTRB(16, 12, 16, 8),
            child: Text('Accent color (from terminal palette)',
                style: TextStyle(fontSize: 13, color: Colors.grey)),
          ),
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
            child: Wrap(
              spacing: 12,
              runSpacing: 12,
              children: primaries.map((c) {
                final selected = c.value == appAccent.value;
                return GestureDetector(
                  onTap: () => onSettingsChanged(accent: c),
                  child: Container(
                    width: 36,
                    height: 36,
                    decoration: BoxDecoration(
                      color: c,
                      shape: BoxShape.circle,
                      border: selected ? Border.all(color: Colors.white, width: 3) : null,
                      boxShadow: selected
                          ? [BoxShadow(color: c.withOpacity(0.6), blurRadius: 8)]
                          : null,
                    ),
                  ),
                );
              }).toList(),
            ),
          ),
          const Divider(height: 32),
          _sectionHeader('Terminal UI'),
          SwitchListTile(
            title: const Text('Dark terminal'),
            subtitle: Text(terminalDark ? 'Dark' : 'Light'),
            value: terminalDark,
            onChanged: (v) => onSettingsChanged(termDark: v),
          ),
          const Padding(
            padding: EdgeInsets.fromLTRB(16, 12, 16, 8),
            child: Text('Color palette', style: TextStyle(fontSize: 13, color: Colors.grey)),
          ),
          ...palettes.map((name) => RadioListTile<String>(
                title: Text(name),
                value: name,
                groupValue: terminalPalette,
                onChanged: (v) => onSettingsChanged(palette: v),
              )),
          const Divider(height: 32),
          _sectionHeader('About'),
          const ListTile(
            title: Text('Grace'),
            subtitle: Text('Programming Language Syntax Drill Software\nThesis project • BSCS-3C'),
            isThreeLine: true,
          ),
          const SizedBox(height: 24),
        ],
      ),
    );
  }

  Widget _sectionHeader(String title) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 20, 16, 8),
      child: Text(title, style: const TextStyle(fontSize: 13, fontWeight: FontWeight.w600, color: Colors.grey)),
    );
  }
}
