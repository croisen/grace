import 'package:grace/utils/colors/catupuccin.dart' as catpuccin;
import 'package:grace/utils/colors/dracula.dart' as dracula;
import 'package:grace/utils/colors/gruvbox.dart' as gruvbox;
import 'package:grace/utils/colors/nord.dart' as nord;
import 'package:grace/utils/colors/one_dark.dart' as one_dark;
import 'package:grace/utils/colors/solarized.dart' as solarized;
import 'package:grace/utils/colors/tokyo_night.dart' as tokyo_night;
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
  final GraceTheme theme;
  final GraceColor color;

  const GraceAppTheme({required this.theme, required this.color});

  ThemeData themeLight() {
    final scheme = switch (theme) {
      .catpuccin => catpuccin.scheme(.light, color),
      .dracula => dracula.scheme(.light, color),
      .gruvbox => gruvbox.scheme(.light, color),
      .nord => nord.scheme(.light, color),
      .onedark => one_dark.scheme(.light, color),
      .solarized => solarized.scheme(.light, color),
      .tokyonight => tokyo_night.scheme(.light, color),
    };

    return _theme(scheme, .light);
  }

  ThemeData themeDark() {
    final scheme = switch (theme) {
      .catpuccin => catpuccin.scheme(.dark, color),
      .dracula => dracula.scheme(.dark, color),
      .gruvbox => gruvbox.scheme(.dark, color),
      .nord => nord.scheme(.dark, color),
      .onedark => one_dark.scheme(.dark, color),
      .solarized => solarized.scheme(.dark, color),
      .tokyonight => tokyo_night.scheme(.dark, color),
    };

    return _theme(scheme, .dark);
  }

  Color color_(Brightness mode, GraceColor color) {
    return switch (theme) {
      .catpuccin => catpuccin.color(mode, color),
      .dracula => dracula.color(mode, color),
      .gruvbox => gruvbox.color(mode, color),
      .nord => nord.color(mode, color),
      .onedark => one_dark.color(mode, color),
      .solarized => solarized.color(mode, color),
      .tokyonight => tokyo_night.color(mode, color),
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
      floatingActionButtonTheme: FloatingActionButtonThemeData(
        backgroundColor: s.primary,
        foregroundColor: s.onPrimary,
      ),
      bottomNavigationBarTheme: BottomNavigationBarThemeData(
        backgroundColor: s.surfaceContainerHighest,
        unselectedItemColor: s.onSurfaceVariant,
        selectedItemColor: s.primary,
      ),
    );
  }
}
