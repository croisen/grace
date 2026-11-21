import 'package:grace/pages/home.dart';
import 'package:grace/utils/colors/mod.dart';
import 'package:grace/utils/settings.dart';
import 'package:material_ui/material_ui.dart';
import 'package:provider/provider.dart';

class GraceApp extends StatelessWidget {
  const GraceApp({super.key});

  @override
  Widget build(BuildContext context) {
    final p = Provider.of<Settings>(context, listen: true);
    final theme = GraceAppTheme(theme: p.theme, color: p.color);

    return MaterialApp(
      title: 'Grace',
      themeMode: p.mode,
      theme: theme.themeLight(),
      darkTheme: theme.themeDark(),
      home: const HomeScreen(),
    );
  }
}
