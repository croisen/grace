import 'package:grace/utils/colors/mod.dart';
import 'package:material_ui/material_ui.dart';

// https://github.com/morhetz/gruvbox

final redDark = Color(0xffcc241d);
final yellowDark = Color(0xffd79921);
final greenDark = Color(0xff98971a);
final blueDark = Color(0xff458588);
final violetDark = Color(0xffb16286);

final baseDark = Color(0xff1d2021);
final surfaceDark = Color(0xff282828);
final onSurfaceDark = Color(0xfffbf1c7);

final redLight = Color(0xff9d0006);
final yellowLight = Color(0xffb57614);
final greenLight = Color(0xff79740e);
final blueLight = Color(0xff076678);
final violetLight = Color(0xff8f3f71);

final baseLight = Color(0xfff9f5d7);
final surfaceLight = Color(0xfffbf1c7);
final onSurfaceLight = Color(0xff282828);

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
    onPrimary: (mode == .dark) ? onSurfaceDark : onSurfaceLight,
    surfaceContainerHighest: (mode == .dark) ? baseDark : baseLight,
    onSurfaceVariant: (mode == .dark) ? onSurfaceDark : onSurfaceLight,
    surface: (mode == .dark) ? surfaceDark : surfaceLight,
    onSurface: (mode == .dark) ? onSurfaceDark : onSurfaceLight,
    error: (mode == .dark) ? redDark : redLight,
    onError: (mode == .dark) ? onSurfaceDark : onSurfaceLight,
  );
}
