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
  int page = 0;

  @override
  Widget build(BuildContext context) {
    final home = DrillsScreen();
    return Scaffold(
      appBar: AppBar(title: const Text('Grace')),
      body: (page == 0) ? home : const SettingsScreen(),
      floatingActionButton: (page == 0)
          ? FloatingActionButton(
              onPressed: () => pagePush(
                context,
                (context) => LanguageSelect(),
              ).then((_) async => home.refresh()),
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
