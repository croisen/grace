import 'package:flutter_code_editor/flutter_code_editor.dart';
import 'package:flutter_highlight/themes/atom-one-dark.dart';
import 'package:flutter_highlight/themes/atom-one-light.dart';
import 'package:flutter_highlight/themes/dracula.dart';
import 'package:flutter_highlight/themes/gruvbox-dark.dart';
import 'package:flutter_highlight/themes/gruvbox-light.dart';
import 'package:flutter_highlight/themes/nord.dart';
import 'package:flutter_highlight/themes/solarized-dark.dart';
import 'package:flutter_highlight/themes/solarized-light.dart';
import 'package:flutter_highlight/themes/tomorrow-night.dart';
import 'package:flutter_highlight/themes/tomorrow.dart';
import 'package:grace/rust/api.dart';
import 'package:grace/rust/api/java.dart';
import 'package:grace/rust/api/rust.dart';
import 'package:grace/rust/api/sql.dart';
import 'package:grace/utils/colors/mod.dart';
import 'package:grace/utils/settings.dart';
import 'package:highlight/languages/java.dart';
import 'package:highlight/languages/rust.dart';
import 'package:highlight/languages/sql.dart';
import 'package:material_ui/material_ui.dart';
import 'package:provider/provider.dart';

import 'package:flutter/material.dart' as sdk;
import 'package:flutter_localizations/flutter_localizations.dart' as sdk_l10n;

class DrillScreen extends StatefulWidget {
  const DrillScreen({super.key, required this.lang, required this.col});
  final DrillLanguage lang;
  final DrillCollection col;

  @override
  State<StatefulWidget> createState() => _DrillScreen();
}

class _DrillScreen extends State<DrillScreen> {
  late CodeController controller;
  late List<String> conditions;
  late String concept;
  late String answer;
  bool finished = false;
  bool showConditions = true;

  @override
  void initState() {
    super.initState();

    final due = widget.col.dueConcept();
    if (due != null) {
      concept = due;
      switch (widget.lang) {
        case .rust:
          final drill = RustConcept.fromString(s: due).makeDrill();
          conditions = drill.conditions.map((b) => b.$2).toList();
          break;
        case .java:
          final drill = JavaConcept.fromString(s: due).makeDrill();
          conditions = drill.conditions.map((b) => b.$2).toList();
          break;
        case .sql:
          final drill = SqlConcept.fromString(s: due).makeDrill();
          conditions = drill.conditions.map((b) => b.$2).toList();
          break;
      }
    } else {
      finished = true;
    }

    controller = CodeController(
      language: switch (widget.lang) {
        .rust => rust,
        .java => java,
        .sql => sql,
      },
      text: switch (widget.lang) {
        .rust => 'fn main() {\n    // Answer goes here\n}',
        .java => 'class Main {\n  public static void main(String[] args) {\n    // Answer goes here\n  }\n}',
        .sql => '-- Answer goes here',
      },
    );
  }

  @override
  void dispose() {
    controller.dispose();
    super.dispose();
  }

  Map<String, TextStyle> _theme(GraceAppTheme t) {
    final b = t.mode_();
    return switch (t.theme) {
      // aww no catpuccin
      .catpuccin => b == .dark ? atomOneDarkTheme : atomOneLightTheme,

      .dracula => draculaTheme,
      .gruvbox => b == .dark ? gruvboxDarkTheme : gruvboxLightTheme,
      .nord => nordTheme,
      .onedark => atomOneDarkTheme,
      .solarized => b == .dark ? solarizedDarkTheme : solarizedLightTheme,

      // or tokyonight
      .tokyonight => b == .dark ? tomorrowNightTheme : tomorrowTheme,
    };
  }

  void _dismissConditions() {
    setState(() => showConditions = false);
  }

  void _toggleConditions() {
    setState(() => showConditions = !showConditions);
  }

  Widget editor(BuildContext context) {
    final p = Provider.of<Settings>(context, listen: true);
    final b = MediaQuery.platformBrightnessOf(context);
    final t = GraceAppTheme(s: p, platformMode: b);
    final theme = Theme.of(context);

    return sdk.Material(
      child: sdk.Localizations.override(
        context: context,
        delegates: const [
          sdk_l10n.GlobalMaterialLocalizations.delegate,
          sdk_l10n.GlobalWidgetsLocalizations.delegate,
        ],
        child: Stack(
          clipBehavior: .none,
          children: [
            CodeTheme(
              data: CodeThemeData(styles: _theme(t)),
              child: Positioned.fill(
                child: CodeField(
                  controller: controller,
                  minLines: 20,
                  maxLines: 50,
                  expands: false,
                  textStyle: theme.textTheme.bodyLarge?.copyWith(
                    fontFamily: 'monospace',
                  ),
                ),
              ),
            ),

            if (showConditions)
              Positioned(
                top: 0,
                left: 0,
                right: 0,
                child: Material(
                  elevation: 8,
                  color: Theme.of(context).cardColor,
                  child: SafeArea(
                    bottom: false,
                    child: Padding(
                      padding: const EdgeInsets.fromLTRB(16, 12, 8, 16),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          Row(
                            children: [
                              const Icon(Icons.info_outline, size: 20),
                              const SizedBox(width: 8),
                              const Text('Conditions'),
                              const Spacer(),
                              IconButton(
                                icon: const Icon(Icons.close),
                                onPressed: _dismissConditions,
                                visualDensity: VisualDensity.standard,
                              ),
                            ],
                          ),
                          const SizedBox(height: 6),
                          ...conditions.map(
                            (c) => Text(
                              c,
                              style: theme.textTheme.bodyMedium?.copyWith(
                                fontFamily: 'monospace',
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              ),
          ],
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Drill - ...')),
      floatingActionButton: FloatingActionButton(
        onPressed: _toggleConditions,
        child: Icon(Icons.priority_high),
      ),
      body: finished ? Text('Finished') : editor(context),
    );
  }
}
