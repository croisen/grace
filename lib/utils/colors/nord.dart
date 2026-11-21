import 'package:grace/utils/colors/mod.dart';
import 'package:material_ui/material_ui.dart';

// https://www.nordtheme.com/docs/colors-and-palettes

final redDark = Color(0xffbf616a);
final yellowDark = Color(0xffebcb8b);
final greenDark = Color(0xffaebe8c);
final blueDark = Color(0xff88c0d0);
final violetDark = Color(0xffb48ead);

final baseDark = Color(0xff2e3440);
final surfaceDark = Color(0xff3b4252);
final onSurfaceDark = Color(0xffeceff4);

final redLight = Color(0xffbf616a);
final yellowLight = Color(0xffebcb8b);
final greenLight = Color(0xffaebe8c);
final blueLight = Color(0xff5e81ac);
final violetLight = Color(0xffb48ead);

final baseLight = Color(0xffd8dee9);
final surfaceLight = Color(0xffeceff4);
final onSurfaceLight = Color(0xff3b4252);

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
