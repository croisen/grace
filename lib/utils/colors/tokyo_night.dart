import 'package:grace/utils/colors/mod.dart';
import 'package:material_ui/material_ui.dart';

// https://colorvio.com/tokyo-night-colors
final redDark = Color(0xfff7768e);
final yellowDark = Color(0xffe0af68);
final greenDark = Color(0xff9ece6a);
final blueDark = Color(0xff7aa2f7);
final violetDark = Color(0xffbb9af7);

final baseDark = Color(0xff16161e);
final surfaceDark = Color(0xff1a1b26);
final onSurfaceDark = Color(0xffc0caf5);

// https://paletty.dev/p/7yC6wp1wKyJ/tokyo-night-day
final redLight = Color(0xfff52a65);
final yellowLight = Color(0xff8c6c3e);
final greenLight = Color(0xff587539);
final blueLight = Color(0xff2e7de9);
final violetLight = Color(0xff9854f1);

final baseLight = Color(0xffb7c1e3);
final surfaceLight = Color(0xffe1e2e7);
final onSurfaceLight = Color(0xff3760bf);

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
