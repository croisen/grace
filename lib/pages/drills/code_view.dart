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
import 'package:grace/rust/api/diagnostics.dart';
import 'package:grace/utils/colors/mod.dart';
import 'package:grace/utils/settings.dart';
import 'package:highlight/languages/java.dart';
import 'package:highlight/languages/rust.dart';
import 'package:highlight/languages/sql.dart';
import 'package:material_ui/material_ui.dart';

import 'package:flutter/material.dart' as sdk;
import 'package:flutter_localizations/flutter_localizations.dart' as sdk_l10n;
import 'package:provider/provider.dart';

class CodeViewScreen extends StatefulWidget {
  const CodeViewScreen({
    super.key,
    required this.title,
    required this.lang,
    required this.code,
    required this.d,
  });

  final String title;
  final String code;
  final DrillLanguage lang;
  final List<Diagnostic> d;

  @override
  State<CodeViewScreen> createState() => _CodeViewScreen();
}

class _CodeViewScreen extends State<CodeViewScreen> {
  late CodeController c1;

  @override
  void initState() {
    super.initState();

    c1 = CodeController(
      text: widget.code,
      readOnly: true,
      analyzer: GraceAnalyzer(d: widget.d),
      language: switch (widget.lang) {
        .rust => rust,
        .java => java,
        .sql => sql,
      },
    );
  }

  @override
  void dispose() {
    c1.dispose();
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

  @override
  Widget build(BuildContext context) {
    final p = Provider.of<Settings>(context);
    final b = MediaQuery.platformBrightnessOf(context);
    final t = GraceAppTheme(s: p, platformMode: b);

    c1.analyzeCode();

    return Scaffold(
      appBar: AppBar(title: Text('Drill - ${widget.title}')),
      body: Expanded(
        child: sdk.Material(
          type: sdk.MaterialType.transparency,
          child: sdk.Localizations.override(
            context: context,
            delegates: const [
              sdk_l10n.GlobalMaterialLocalizations.delegate,
              sdk_l10n.GlobalWidgetsLocalizations.delegate,
            ],
            child: CodeTheme(
              data: CodeThemeData(styles: _theme(t)),
              child: CodeField(controller: c1),
            ),
          ),
        ),
      ),
    );
  }
}

class GraceAnalyzer extends AbstractAnalyzer {
  const GraceAnalyzer({required this.d});
  final List<Diagnostic> d;

  @override
  Future<AnalysisResult> analyze(Code code) async {
    return AnalysisResult(
      issues: d
          .map(
            (d) => Issue(
              line: d.line?.toInt() ?? 0,
              message: d.message,
              type: switch (d.grade) {
                .again => .error,
                .hard => .warning,
                .good => .info,
                .easy => .info,
              },
            ),
          )
          .toList(),
    );
  }
}
