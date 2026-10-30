import 'package:flutter/material.dart';
import 'create_rust_deck_screen.dart';

class LanguageSelectScreen extends StatelessWidget {
  final bool terminalDark;
  final String terminalPalette;

  const LanguageSelectScreen({
    super.key,
    required this.terminalDark,
    required this.terminalPalette,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Choose Language')),
      body: ListView(
        children: [
          const SizedBox(height: 8),
          const Padding(
            padding: EdgeInsets.fromLTRB(16, 12, 16, 8),
            child: Text('Select a language to create a new drill deck',
                style: TextStyle(color: Colors.grey)),
          ),
          ListTile(
            leading: const Icon(Icons.code),
            title: const Text('Rust'),
            subtitle: const Text('Available'),
            trailing: const Icon(Icons.chevron_right),
            onTap: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (_) => CreateRustDeckScreen(
                    terminalDark: terminalDark,
                    terminalPalette: terminalPalette,
                  ),
                ),
              );
            },
          ),
          const Divider(),
          ListTile(
            leading: Icon(Icons.storage, color: Colors.grey.shade400),
            title: Text('SQL', style: TextStyle(color: Colors.grey.shade500)),
            subtitle: Text('Coming later', style: TextStyle(color: Colors.grey.shade400)),
            enabled: false,
          ),
          ListTile(
            leading: Icon(Icons.code, color: Colors.grey.shade400),
            title: Text('Java', style: TextStyle(color: Colors.grey.shade500)),
            subtitle: Text('Coming later', style: TextStyle(color: Colors.grey.shade400)),
            enabled: false,
          ),
        ],
      ),
    );
  }
}
