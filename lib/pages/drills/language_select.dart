import 'dart:async';

import 'package:grace/pages/drills/drill_create.dart';
import 'package:material_ui/material_ui.dart';
import 'package:grace/utils/route.dart';

class LanguageSelect extends StatelessWidget {
  final Completer<void> comp;
  const LanguageSelect({super.key, required this.comp});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Select Language')),
      body: ListView(
        children: [
          const SizedBox(height: 8),
          const Padding(
            padding: EdgeInsets.fromLTRB(16, 12, 16, 8),
            child: Text(
              'Select a language to create a new drill deck',
              style: TextStyle(color: Colors.grey),
            ),
          ),
          ListTile(
            leading: const Icon(Icons.code),
            title: const Text('Rust'),
            subtitle: const Text('Available'),
            trailing: const Icon(Icons.chevron_right),
            onTap: () {
              pageReplace(
                context,
                (context) => DrillCreate(lang: .rust, comp: comp),
              );
            },
          ),
          const Divider(),
          ListTile(
            leading: const Icon(Icons.storage),
            title: const Text('SQL'),
            subtitle: const Text('Available'),
            trailing: const Icon(Icons.chevron_right),
            onTap: () {
              pageReplace(
                context,
                (context) => DrillCreate(lang: .sql, comp: comp),
              );
            },
          ),
          const Divider(),
          ListTile(
            leading: const Icon(Icons.code),
            title: const Text('Java'),
            subtitle: const Text('Available'),
            trailing: const Icon(Icons.chevron_right),
            onTap: () {
              pageReplace(
                context,
                (context) => DrillCreate(lang: .java, comp: comp),
              );
            },
          ),
        ],
      ),
    );
  }
}
