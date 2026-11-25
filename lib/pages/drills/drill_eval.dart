import 'dart:async';

import 'package:grace/pages/drills/code_view.dart';
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
import 'package:material_ui/material_ui.dart';
import 'package:provider/provider.dart';

class DrillEval extends StatefulWidget {
  const DrillEval({
    super.key,
    required this.col,
    required this.comp,
    required this.drill,
    required this.userAnswer,
  });

  final DrillCollection col;
  final Object drill;
  final String userAnswer;
  final Completer<void> comp;

  @override
  State<StatefulWidget> createState() => _DrillEval();
}

class _DrillEval extends State<DrillEval> {
  late Diagnostics diagnostics;
  late Grade grade;
  late String concept;
  late List<(bool, String)> conditions;
  late String correctAnswer;
  bool wasReplaced = false;

  @override
  void initState() {
    super.initState();
    correctAnswer = switch (widget.col.lang) {
      .rust => (widget.drill as RustDrill).answer,
      .java => (widget.drill as JavaDrill).answer,
      .sql => (widget.drill as SqlDrill).answer,
    };

    diagnostics = switch (widget.col.lang) {
      .rust => (widget.drill as RustDrill).check(userCode: widget.userAnswer),
      .java => (widget.drill as JavaDrill).check(userCode: widget.userAnswer),
      .sql => (widget.drill as SqlDrill).check(userCode: widget.userAnswer),
    };

    grade = diagnostics.grade();
    conditions = switch (widget.col.lang) {
      .rust => (widget.drill as RustDrill).conditions,
      .java => (widget.drill as JavaDrill).conditions,
      .sql => (widget.drill as SqlDrill).conditions,
    };

    concept = switch (widget.col.lang) {
      .rust => (widget.drill as RustDrill).concept.toString(),
      .java => (widget.drill as JavaDrill).concept.toString(),
      .sql => (widget.drill as SqlDrill).concept.toString(),
    };

    widget.col.reviewDrill(concept: concept, grade: grade);
    DatabaseHelper.instance.saveCollection(widget.col);
  }

  @override
  Widget build(BuildContext context) {
    final p = Provider.of<Settings>(context);
    final b = MediaQuery.platformBrightnessOf(context);
    final t = GraceAppTheme(s: p, platformMode: b);
    final theme = Theme.of(context);

    final color = switch (grade) {
      .again => t.color_(.red),
      .hard => t.color_(.yellow),
      .good => t.color_(.green),
      .easy => t.color_(.blue),
    };

    return PopScope(
      onPopInvokedWithResult: (didPop, _) {
        if (didPop && !wasReplaced) {
          widget.comp.complete();
        }
      },
      child: Scaffold(
        appBar: AppBar(title: const Text('Drill Results')),
        floatingActionButton: FloatingActionButton(
          onPressed: () {
            wasReplaced = true;
            pageReplace(
              context,
              (context) => DrillScreen(
                lang: widget.col.lang,
                comp: widget.comp,
                col: widget.col,
              ),
            );
          },
          child: Icon(Icons.add),
        ),
        body: Stack(
          fit: .expand,
          children: [
            SingleChildScrollView(
              padding: const .all(10),
              child: Column(
                mainAxisAlignment: .start,
                crossAxisAlignment: .stretch,
                children: [
                  Center(
                    child: Text(
                      'Grade: $grade',
                      style: theme.textTheme.headlineLarge?.copyWith(
                        color: color,
                      ),
                    ),
                  ),

                  const Divider(),

                  Text('Conditions'),
                  ...conditions.map(
                    (a) => a.$1
                        ? Text(
                            '[/] ${a.$2}',
                            style: theme.textTheme.bodyLarge?.copyWith(
                              fontFamily: 'monospace',
                            ),
                          )
                        : Text(
                            '[X] ${a.$2}',
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
            ),

            Positioned(
              bottom: MediaQuery.paddingOf(context).bottom + 20,
              left: 1,
              right: 1,
              child: Row(
                mainAxisAlignment: .spaceEvenly,
                children: [
                  OutlinedButton(
                    onPressed: () => pagePush(
                      context,
                      (context) => CodeViewScreen(
                        title: "Your Answer",
                        lang: widget.col.lang,
                        code: widget.userAnswer,
                        d: diagnostics.d,
                      ),
                    ),
                    style: OutlinedButton.styleFrom(
                      backgroundColor: t.color_(.green),
                      foregroundColor: t.scheme_(t.mode_()).onSurface,
                    ),
                    child: const Text('Your Answer'),
                  ),
                  const SizedBox(width: 12),
                  ElevatedButton(
                    onPressed: () => pagePush(
                      context,
                      (context) => CodeViewScreen(
                        title: "Expected Answer",
                        lang: widget.col.lang,
                        code: correctAnswer,
                        d: [],
                      ),
                    ),
                    child: const Text('Expected Answer'),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
