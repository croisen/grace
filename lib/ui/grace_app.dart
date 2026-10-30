import 'package:flutter/material.dart';
import 'screens/deck_list_screen.dart';

class GraceApp extends StatefulWidget {
  const GraceApp({super.key});

  @override
  State<GraceApp> createState() => _GraceAppState();
}

class _GraceAppState extends State<GraceApp> {
  ThemeMode appThemeMode = ThemeMode.system;
  Color appAccent = const Color(0xFF2196F3);

  bool terminalDark = true;
  String terminalPalette = 'Default';

  void updateSettings({
    ThemeMode? appMode,
    Color? accent,
    bool? termDark,
    String? palette,
  }) {
    setState(() {
      if (appMode != null) appThemeMode = appMode;
      if (accent != null) appAccent = accent;
      if (termDark != null) terminalDark = termDark;
      if (palette != null) {
        terminalPalette = palette;
        appAccent = _primariesFor(palette).first;
      }
    });
  }

  static List<Color> _primariesFor(String palette) {
    switch (palette) {
      case 'Gruvbox':
        return const [
          Color(0xFFFB4934), Color(0xFFB8BB26), Color(0xFFFABD2F),
          Color(0xFF83A598), Color(0xFFD3869B), Color(0xFF8EC07C), Color(0xFFFE8019),
        ];
      case 'Tokyo Night':
        return const [
          Color(0xFFF7768E), Color(0xFF9ECE6A), Color(0xFFE0AF68),
          Color(0xFF7AA2F7), Color(0xFFBB9AF7), Color(0xFF7DCFFF), Color(0xFFFF9E64),
        ];
      case 'Dracula':
        return const [
          Color(0xFFFF5555), Color(0xFF50FA7B), Color(0xFFF1FA8C),
          Color(0xFFBD93F9), Color(0xFF8BE9FD), Color(0xFFFFB86C), Color(0xFFFF79C6),
        ];
      case 'Nord':
        return const [
          Color(0xFFBF616A), Color(0xFFA3BE8C), Color(0xFFEBCB8B),
          Color(0xFF81A1C1), Color(0xFFB48EAD), Color(0xFF88C0D0), Color(0xFFD08770),
        ];
      case 'Catppuccin':
        return const [
          Color(0xFFF38BA8), Color(0xFFA6E3A1), Color(0xFFF9E2AF),
          Color(0xFF89B4FA), Color(0xFFCBA6F7), Color(0xFF94E2D5), Color(0xFFFAB387),
        ];
      case 'One Dark':
        return const [
          Color(0xFFE06C75), Color(0xFF98C379), Color(0xFFE5C07B),
          Color(0xFF61AFEF), Color(0xFFC678DD), Color(0xFF56B6C2), Color(0xFFD19A66),
        ];
      case 'Solarized':
        return const [
          Color(0xFFDC322F), Color(0xFF859900), Color(0xFFB58900),
          Color(0xFF268BD2), Color(0xFF6C71C4), Color(0xFF2AA198), Color(0xFFCB4B16),
        ];
      default:
        return const [
          Color(0xFFE53935), Color(0xFF43A047), Color(0xFFFDD835),
          Color(0xFF1E88E5), Color(0xFF8E24AA), Color(0xFF00ACC1), Color(0xFFFB8C00),
        ];
    }
  }

  List<Color> get currentPrimaries => _primariesFor(terminalPalette);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Grace',
      debugShowCheckedModeBanner: false,
      themeMode: appThemeMode,
      theme: ThemeData(
        useMaterial3: true,
        brightness: Brightness.light,
        colorScheme: ColorScheme.fromSeed(seedColor: appAccent, brightness: Brightness.light),
        appBarTheme: AppBarTheme(
          backgroundColor: appAccent,
          foregroundColor: Colors.white,
          elevation: 0,
        ),
        floatingActionButtonTheme: FloatingActionButtonThemeData(
          backgroundColor: appAccent,
          foregroundColor: Colors.white,
        ),
      ),
      darkTheme: ThemeData(
        useMaterial3: true,
        brightness: Brightness.dark,
        colorScheme: ColorScheme.fromSeed(seedColor: appAccent, brightness: Brightness.dark),
        appBarTheme: AppBarTheme(
          backgroundColor: appAccent, // now follows accent even in dark
          foregroundColor: Colors.white,
          elevation: 0,
        ),
        floatingActionButtonTheme: FloatingActionButtonThemeData(
          backgroundColor: appAccent,
          foregroundColor: Colors.white,
        ),
      ),
      home: DeckListScreen(
        appThemeMode: appThemeMode,
        appAccent: appAccent,
        terminalDark: terminalDark,
        terminalPalette: terminalPalette,
        primaries: currentPrimaries,
        onSettingsChanged: updateSettings,
      ),
    );
  }
}
