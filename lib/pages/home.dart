import 'dart:async';

import 'package:grace/pages/drill.dart';
import 'package:grace/pages/drills/language_select.dart';
import 'package:grace/pages/settings.dart';
import 'package:material_ui/material_ui.dart';
import 'package:grace/utils/route.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreen();
}

class _HomeScreen extends State<HomeScreen> {
  Key _key = UniqueKey();
  int page = 0;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Grace')),
      body: (page == 0) ? DrillsScreen(key: _key) : const SettingsScreen(),
      floatingActionButton: (page == 0)
          ? FloatingActionButton(
              onPressed: () async {
                final comp = Completer<void>();
                pagePush(context, (context) => LanguageSelect(comp: comp));
                await comp.future;
                _key = UniqueKey();
                setState(() => {});
              },
              child: const Icon(Icons.add),
            )
          : null,
      bottomNavigationBar: BottomNavigationBar(
        currentIndex: page,
        items: const [
          BottomNavigationBarItem(icon: Icon(Icons.home), label: "Home"),
          BottomNavigationBarItem(
            icon: Icon(Icons.settings),
            label: "Settings",
          ),
        ],
        onTap: (v) => setState(() => page = v),
      ),
    );
  }
}
