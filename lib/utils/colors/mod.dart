import 'package:grace/utils/colors/catupuccin.dart' as catpuccin;
import 'package:grace/utils/colors/dracula.dart' as dracula;
import 'package:grace/utils/colors/gruvbox.dart' as gruvbox;
import 'package:grace/utils/colors/nord.dart' as nord;
import 'package:grace/utils/colors/one_dark.dart' as one_dark;
import 'package:grace/utils/colors/solarized.dart' as solarized;
import 'package:grace/utils/colors/tokyo_night.dart' as tokyo_night;
import 'package:grace/utils/settings.dart';
import 'package:material_ui/material_ui.dart';

enum GraceTheme {
  catpuccin,
  dracula,
  gruvbox,
  nord,
  onedark,
  solarized,
  tokyonight,
}

enum GraceColor { red, yellow, green, blue, violet }

class GraceAppTheme {
  final ThemeMode appMode;
  final Brightness platformMode;
  final GraceTheme theme;
  final GraceColor color;
  final Settings s;

  GraceAppTheme({required this.s, required this.platformMode})
    : appMode = s.mode,
      theme = s.theme,
      color = s.color;

  ThemeData themeLight() {
    final scheme = scheme_(.light);
    return _theme(scheme, .light);
  }

  ThemeData themeDark() {
    final scheme = scheme_(.dark);
    return _theme(scheme, .dark);
  }

  Color color_(GraceColor color) {
    final m = mode_();

    return switch (theme) {
      .catpuccin => catpuccin.color(m, color),
      .dracula => dracula.color(m, color),
      .gruvbox => gruvbox.color(m, color),
      .nord => nord.color(m, color),
      .onedark => one_dark.color(m, color),
      .solarized => solarized.color(m, color),
      .tokyonight => tokyo_night.color(m, color),
    };
  }

  ColorScheme scheme_(Brightness b) {
    return switch (theme) {
      .catpuccin => catpuccin.scheme(b, color),
      .dracula => dracula.scheme(b, color),
      .gruvbox => gruvbox.scheme(b, color),
      .nord => nord.scheme(b, color),
      .onedark => one_dark.scheme(b, color),
      .solarized => solarized.scheme(b, color),
      .tokyonight => tokyo_night.scheme(b, color),
    };
  }

  Brightness mode_() {
    return switch (appMode) {
      .system => platformMode,
      .light => Brightness.light,
      .dark => Brightness.dark,
    };
  }

  ThemeData _theme(ColorScheme s, Brightness b) {
    return ThemeData(
      brightness: b,
      colorScheme: s,
      appBarTheme: AppBarThemeData(
        backgroundColor: s.primary,
        foregroundColor: s.onPrimary,
      ),
      bottomNavigationBarTheme: BottomNavigationBarThemeData(
        backgroundColor: s.surfaceContainerHighest,
        unselectedItemColor: s.onSurfaceVariant,
        selectedItemColor: s.primary,
      ),
      floatingActionButtonTheme: FloatingActionButtonThemeData(
        backgroundColor: s.primary,
        foregroundColor: s.onPrimary,
      ),
      listTileTheme: ListTileThemeData(
        tileColor: s.surfaceContainerHighest,
        textColor: s.onSurfaceVariant,
      ),
      scaffoldBackgroundColor: s.surface,
      snackBarTheme: SnackBarThemeData(
        backgroundColor: s.surfaceContainerHighest,
      ),
    );
  }
}
