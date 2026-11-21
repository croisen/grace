import 'package:grace/utils/colors/mod.dart';
import 'package:material_ui/material_ui.dart';
import 'package:shared_preferences/shared_preferences.dart';

class Settings extends ChangeNotifier {
  late SharedPreferences p;
  final String kmode = 'mode';
  final String kcolor = 'color';
  final String ktheme = 'theme';

  Future<void> init() async {
    p = await SharedPreferences.getInstance();
    _mode = ThemeMode.values[p.getInt(kmode) ?? 0];
    _color = GraceColor.values[p.getInt(kcolor) ?? 2];
    _theme = GraceTheme.values[p.getInt(ktheme) ?? 2];
  }

  ThemeMode _mode = .system;
  ThemeMode get mode => _mode;
  set mode(ThemeMode m) => _saveMode(m);
  void _saveMode(ThemeMode m) async {
    _mode = m;
    await p.setInt(kmode, _mode.index);
    notifyListeners();
  }

  GraceColor _color = .green;
  GraceColor get color => _color;
  set color(GraceColor c) => _saveColor(c);
  void _saveColor(GraceColor c) async {
    _color = c;
    await p.setInt(kcolor, _color.index);
    notifyListeners();
  }

  GraceTheme _theme = .catpuccin;
  GraceTheme get theme => _theme;
  set theme(GraceTheme t) => _saveTheme(t);
  void _saveTheme(GraceTheme t) async {
    _theme = t;
    await p.setInt(ktheme, _theme.index);
    notifyListeners();
  }
}
