import 'package:grace/utils/colors/mod.dart';
import 'package:material_ui/material_ui.dart';

// https://catppuccin.com/palette/

final redDark = Color(0xfff38ba8);
final yellowDark = Color(0xfff9e2af);
final greenDark = Color(0xffa6e3a1);
final blueDark = Color(0xff89b4fa);
final violetDark = Color(0xffcba6f7);

final baseDark = Color(0xff1e1e2e);
final surfaceDark = Color(0xff313244);
final onSurfaceDark = Color(0xffcdd6f4);

final redLight = Color(0xffd20f39);
final yellowLight = Color(0xffdf8e1d);
final greenLight = Color(0xff40a02b);
final blueLight = Color(0xff1e66f5);
final violetLight = Color(0xff8839ef);

final baseLight = Color(0xffeff1f5);
final surfaceLight = Color(0xffccd0da);
final onSurfaceLight = Color(0xff4f4f8d);

Color color(Brightness mode, GraceColor color) {
  return switch (color) {
    .red => (mode == .dark) ? redDark : redLight,
    .yellow => (mode == .dark) ? yellowDark : yellowLight,
    .green => (mode == .dark) ? greenDark : greenLight,
    .blue => (mode == .dark) ? blueDark : blueLight,
    .violet => (mode == .dark) ? violetDark : violetLight,
  };
}

ColorScheme scheme(Brightness mode, GraceColor color2) {
  final c = color(mode, color2);
  return ColorScheme.fromSeed(
    brightness: mode,
    seedColor: c,
    primary: c,
    onPrimary: (mode == .light) ? onSurfaceDark : onSurfaceLight,
    surfaceContainerHighest: (mode == .dark) ? baseDark : baseLight,
    onSurfaceVariant: (mode == .dark) ? onSurfaceDark : onSurfaceLight,
    surface: (mode == .dark) ? surfaceDark : surfaceLight,
    onSurface: (mode == .dark) ? onSurfaceDark : onSurfaceLight,
    error: (mode == .dark) ? redDark : redLight,
    onError: (mode == .dark) ? onSurfaceDark : onSurfaceLight,
  );
}
