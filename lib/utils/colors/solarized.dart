import 'package:grace/utils/colors/mod.dart';
import 'package:material_ui/material_ui.dart';

// https://pensieve.swijaya.me/solarized-selenized-cheatsheet

final redDark = Color(0xffe0332e);
final yellowDark = Color(0xffbb8801);
final greenDark = Color(0xff8d9800);
final blueDark = Color(0xff008dd1);
final violetDark = Color(0xff5c73c4);

final baseDark = Color(0xff002b36);
final surfaceDark = Color(0xff003641);
final onSurfaceDark = Color(0xfffff6e3);

final baseLight = Color(0xfff0e7d5);
final surfaceLight = Color(0xfffff6e3);
final onSurfaceLight = Color(0xff003641);

Color color(Brightness mode, GraceColor color) {
  return switch (color) {
    .red => redDark,
    .yellow => yellowDark,
    .green => greenDark,
    .blue => blueDark,
    .violet => violetDark,
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
    error: redDark,
    onError: (mode == .dark) ? onSurfaceDark : onSurfaceLight,
  );
}
