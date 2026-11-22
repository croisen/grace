import 'package:grace/utils/colors/mod.dart';
import 'package:grace/utils/settings.dart';
import 'package:material_ui/material_ui.dart';
import 'package:provider/provider.dart';

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final p = Provider.of<Settings>(context, listen: true);
    final b = MediaQuery.platformBrightnessOf(context);
    final t = GraceAppTheme(s: p, platformMode: b);

    return ListView(
      children: [
        ListTile(
          leading: Icon(
            t.mode_() == .dark ? Icons.dark_mode : Icons.light_mode,
          ),
          title: const Text('App Mode'),
        ),
        RadioGroup<ThemeMode>(
          groupValue: p.mode,
          onChanged: (mode) {
            if (mode != null) p.mode = mode;
          },
          child: const Column(
            children: [
              RadioListTile<ThemeMode>(value: .system, title: Text('System')),
              RadioListTile<ThemeMode>(value: .light, title: Text('Light')),
              RadioListTile<ThemeMode>(value: .dark, title: Text('Dark')),
            ],
          ),
        ),

        const Divider(thickness: 5),

        ListTile(
          leading: Container(
            width: 27,
            height: 27,
            decoration: BoxDecoration(
              color: t.color_(p.color),
              shape: BoxShape.circle,
            ),
          ),
          title: Text('Accent Color'),
        ),
        Padding(
          padding: const .symmetric(horizontal: 16, vertical: 8),
          child: Row(
            mainAxisAlignment: .start,
            spacing: 20,

            children: GraceColor.values
                .map(
                  (c) => GestureDetector(
                    onTap: () => p.color = c,
                    child: Container(
                      width: 36,
                      height: 36,
                      decoration: BoxDecoration(
                        color: t.color_(c),
                        shape: BoxShape.circle,
                        border: p.color == c
                            ? Border.all(color: Colors.white, width: 3)
                            : null,
                      ),
                    ),
                  ),
                )
                .toList(),
          ),
        ),

        const Divider(thickness: 5),

        const ListTile(title: Text('App Theme')),
        RadioGroup<GraceTheme>(
          groupValue: p.theme,
          onChanged: (theme) {
            if (theme != null) p.theme = theme;
          },
          child: const Column(
            children: [
              RadioListTile<GraceTheme>(
                value: .gruvbox,
                title: Text('Gruvbox'),
              ),
              RadioListTile<GraceTheme>(
                value: .catpuccin,
                title: Text('Catpuccin'),
              ),
              RadioListTile<GraceTheme>(
                value: .dracula,
                title: Text('Dracula'),
              ),
              RadioListTile<GraceTheme>(value: .nord, title: Text('Nord')),
              RadioListTile<GraceTheme>(
                value: .onedark,
                title: Text('One Dark (always dark)'),
              ),
              RadioListTile<GraceTheme>(
                value: .solarized,
                title: Text('Solarized'),
              ),
              RadioListTile<GraceTheme>(
                value: .tokyonight,
                title: Text('Tokyo Day/Night'),
              ),
            ],
          ),
        ),
      ],
    );
  }
}
