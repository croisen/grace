import 'package:grace/utils/colors/mod.dart';
import 'package:material_ui/material_ui.dart';

// https://draculatheme.com/spec

final redDark = Color(0xffff5555);
final yellowDark = Color(0xfff1fa8c);
final greenDark = Color(0xff50fa7b);
final blueDark = Color(0xff8be9fd);
final violetDark = Color(0xffbd93f9);

final baseDark = Color(0xff282a36);
final surfaceDark = Color(0xff282a36);
final onSurfaceDark = Color(0xfff8f8f2);

final redLight = Color(0xffcb3a2a);
final yellowLight = Color(0xff846e15);
final greenLight = Color(0xff14710a);
final blueLight = Color(0xff036a96);
final violetLight = Color(0xff644ac9);

final baseLight = Color(0xfffffbeb);
final surfaceLight = Color(0xfffffbeb);
final onSurfaceLight = Color(0xff1f1f1f);

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
