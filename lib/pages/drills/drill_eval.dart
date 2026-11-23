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
import 'package:grace/pages/drills/drill.dart';
import 'package:grace/rust/api.dart';
import 'package:grace/rust/api/diagnostics.dart';
import 'package:grace/rust/api/grade.dart';
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

class DrillEval extends StatefulWidget {
  const DrillEval({
    super.key,
    required this.col,
    required this.concept,
    required this.conditions,
    required this.correctAnswer,
    required this.userAnswer,
  });

  final DrillCollection col;
  final List<(bool, String)> conditions;
  final String concept;
  final String correctAnswer;
  final String userAnswer;

  @override
  State<StatefulWidget> createState() => _DrillEval();
}

class _DrillEval extends State<DrillEval> {
  // late CodeController c1;
  // late CodeController c2;
  late Diagnostics diagnostics;
  late Grade grade;

  @override
  void initState() {
    super.initState();
    // c1 = CodeController(
    //   text: widget.userAnswer,
    //   readOnly: true,
    //   language: switch (widget.col.lang) {
    //     .rust => rust,
    //     .java => java,
    //     .sql => sql,
    //   },
    // );
    //
    // c2 = CodeController(
    //   text: widget.correctAnswer,
    //   readOnly: true,
    //   language: switch (widget.col.lang) {
    //     .rust => rust,
    //     .java => java,
    //     .sql => sql,
    //   },
    // );

    diagnostics = switch (widget.col.lang) {
      .rust => RustDrill(
        concept: widget.concept,
        conditions: widget.conditions,
        answer: widget.correctAnswer,
      ).check(userCode: widget.userAnswer),
      .java => JavaDrill(
        concept: widget.concept,
        conditions: widget.conditions,
        answer: widget.correctAnswer,
      ).check(userCode: widget.userAnswer),
      .sql => SqlDrill(
        concept: widget.concept,
        conditions: widget.conditions,
        answer: widget.correctAnswer,
      ).check(userCode: widget.userAnswer),
    };

    grade = diagnostics.grade();
    widget.col.reviewDrill(concept: widget.concept, grade: grade);
    DatabaseHelper.instance.saveCollection(widget.col);
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

  @override
  Widget build(BuildContext context) {
    final p = Provider.of<Settings>(context);
    final b = MediaQuery.platformBrightnessOf(context);
    final t = GraceAppTheme(s: p, platformMode: b);
    final theme = Theme.of(context);

    return Scaffold(
      appBar: AppBar(title: const Text('Drill Results')),
      floatingActionButton: FloatingActionButton(
        onPressed: () => pageReplace(
          context,
          (context) => DrillScreen(lang: widget.col.lang, col: widget.col),
        ),
        child: Icon(Icons.add),
      ),
      body: Column(
        mainAxisAlignment: .start,
        crossAxisAlignment: .stretch,
        children: [
          Text('Grade: $grade'),
          const Divider(),

          Text('Conditions'),
          ...widget.conditions.map(
            (a) => a.$1
                ? Text(
                    '[X] ${a.$2}',
                    style: theme.textTheme.bodyLarge?.copyWith(
                      fontFamily: 'monospace',
                    ),
                  )
                : Text(
                    '[/] ${a.$2}',
                    style: theme.textTheme.bodyLarge?.copyWith(
                      fontFamily: 'monospace',
                    ),
                  ),
          ),

          const Divider(),
          Text('Diagnostics'),
          ...diagnostics.d.map(
            (d) => Text(
              d.toString(),
              style: theme.textTheme.bodyLarge?.copyWith(
                fontFamily: 'monospace',
              ),
            ),
          ),
        ],
      ),
    );
  }
}
