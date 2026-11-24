import 'dart:async';

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
import 'package:grace/pages/drills/drill_eval.dart';
import 'package:grace/rust/api.dart';
import 'package:grace/rust/api/java.dart';
import 'package:grace/rust/api/rust.dart';
import 'package:grace/rust/api/sql.dart';
import 'package:grace/utils/colors/mod.dart';
import 'package:grace/utils/database/mod.dart';
import 'package:grace/utils/route.dart';
import 'package:grace/utils/settings.dart';
import 'package:highlight/languages/java.dart';
import 'package:highlight/languages/rust.dart';
import 'package:highlight/languages/sql.dart';
import 'package:material_ui/material_ui.dart';
import 'package:provider/provider.dart';

import 'package:flutter/material.dart' as sdk;
import 'package:flutter_localizations/flutter_localizations.dart' as sdk_l10n;

class DrillScreen extends StatefulWidget {
  const DrillScreen({
    super.key,
    required this.lang,
    required this.col,
    required this.comp,
  });
  final DrillLanguage lang;
  final DrillCollection col;
  final Completer<void> comp;

  @override
  State<StatefulWidget> createState() => _DrillScreen();
}

class _DrillScreen extends State<DrillScreen> {
  late CodeController controller;
  late List<(bool, String)> conditions;
  late String concept;
  late String answer;
  late Object drill;
  bool wasReplaced = false;
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
          final d = RustConcept.fromString(s: due).makeDrill();
          drill = d;
          conditions = d.conditions;
          answer = d.answer;
          break;
        case .java:
          final d = JavaConcept.fromString(s: due).makeDrill();
          drill = d;
          conditions = d.conditions;
          answer = d.answer;
          break;
        case .sql:
          final d = SqlConcept.fromString(s: due).makeDrill();
          drill = d;
          conditions = d.conditions;
          answer = d.answer;
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
          fit: .expand,
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
                              c.$2,
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

            if (showConditions)
              Positioned(
                bottom: MediaQuery.paddingOf(context).bottom + 20,
                left: 1,
                right: 1,
                child: Row(
                  mainAxisAlignment: .spaceEvenly,
                  children: [
                    OutlinedButton(
                      onPressed: () async {
                        wasReplaced = true;
                        widget.col.reviewDrill(concept: concept, grade: .again);
                        await DatabaseHelper.instance.saveCollection(
                          widget.col,
                        );

                        if (context.mounted) {
                          pageReplace(
                            context,
                            (context) => DrillScreen(
                              col: widget.col,
                              lang: widget.col.lang,
                              comp: widget.comp,
                            ),
                          );
                        }
                      },
                      style: OutlinedButton.styleFrom(
                        backgroundColor: t.color_(.red),
                        foregroundColor: t.scheme_(t.mode_()).onSurface,
                      ),
                      child: const Text('Skip'),
                    ),
                    const SizedBox(width: 12),
                    ElevatedButton(
                      onPressed: () {
                        wasReplaced = true;
                        pageReplace(
                          context,
                          (context) => DrillEval(
                            col: widget.col,
                            comp: widget.comp,
                            drill: drill,
                            userAnswer: controller.text,
                          ),
                        );
                      },
                      child: const Text('Submit'),
                    ),
                  ],
                ),
              ),
          ],
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return PopScope(
      onPopInvokedWithResult: (didPop, _) {
        if (didPop && !wasReplaced) {
          widget.comp.complete();
        }
      },
      child: Scaffold(
        appBar: AppBar(title: const Text('Drill - ...')),
        floatingActionButton: FloatingActionButton(
          onPressed: _toggleConditions,
          child: Icon(Icons.priority_high),
        ),
        body: finished ? Text('Finished') : editor(context),
      ),
    );
  }
}
