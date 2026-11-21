import 'package:grace/utils/colors/mod.dart';
import 'package:material_ui/material_ui.dart';

// https://sethue.com/color-palette/one-dark

final redDark = Color(0xffe06c75);
final yellowDark = Color(0xff93691d);
final greenDark = Color(0xff98c379);
final blueDark = Color(0xff61afef);
final violetDark = Color(0xff8a3fa0);

final baseDark = Color(0xff181a1f);
final surfaceDark = Color(0xff282c34);
final onSurfaceDark = Color(0xffabb2bf);

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
    onPrimary: onSurfaceDark,
    surfaceContainerHighest: baseDark,
    onSurfaceVariant: onSurfaceDark,
    surface: surfaceDark,
    onSurface: onSurfaceDark,
    error: redDark,
    onError: onSurfaceDark,
  );
}
