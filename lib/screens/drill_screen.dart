import 'package:flutter/material.dart';
import 'package:grace/rust/api/drills/rust/if_statement.dart';
import 'package:grace/rust/api/drills/rust/else_if.dart';
import 'package:grace/rust/api/drills/rust/match_statement.dart';
import 'package:grace/rust/api/drills/rust/while_statement.dart';
import 'package:grace/rust/api/drills/rust/for_statement.dart';
import 'package:grace/rust/api/drills/rust/loop_statement.dart';
import 'package:grace/rust/api/drills/rust/function.dart';
import 'package:flutter_highlight/themes/gruvbox-dark.dart';
import 'package:flutter_code_editor/flutter_code_editor.dart';
import 'package:highlight/languages/rust.dart';

class DrillScreen extends StatefulWidget {
  const DrillScreen({super.key});

  @override
  State<DrillScreen> createState() => _DrillScreenState();
}

class _DrillScreenState extends State<DrillScreen> {
  final controller = CodeController(language: rust);

  String conditions = 'Pick a concept above to start a drill.';
  String expectedAnswer = '';
  String? feedback;

  void loadDrill(String concept) {
    late final String newConditions;
    late final String newAnswer;

    switch (concept) {
      case 'if':
        final d = generateIfStatement();
        newConditions = d.conditions;
        newAnswer = d.answer;
        break;
      case 'else if':
        final d = generateElseIf();
        newConditions = d.conditions;
        newAnswer = d.answer;
        break;
      case 'match':
        final d = generateMatchStatement();
        newConditions = d.conditions;
        newAnswer = d.answer;
        break;
      case 'while':
        final d = generateWhileStatement();
        newConditions = d.conditions;
        newAnswer = d.answer;
        break;
      case 'for':
        final d = generateForStatement();
        newConditions = d.conditions;
        newAnswer = d.answer;
        break;
      case 'loop':
        final d = generateLoopStatement();
        newConditions = d.conditions;
        newAnswer = d.answer;
        break;
      case 'function':
        final d = generateFunction();
        newConditions = d.conditions;
        newAnswer = d.answer;
        break;
      default:
        return;
    }

    setState(() {
      conditions = newConditions;
      expectedAnswer = newAnswer;
      controller.text = '';
      feedback = null;
    });
  }

  void checkAnswer() {
    // Temporary exact-match. Replace later with AST checker.
    final user = controller.text.trim();
    final correct = user == expectedAnswer.trim();

    setState(() {
      feedback = correct ? 'Correct!' : 'Not quite — try again.';
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Grace – Syntax Drill')),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Wrap(
              spacing: 8,
              runSpacing: 8,
              children: [
                for (final c in [
                  'if',
                  'else if',
                  'match',
                  'while',
                  'for',
                  'loop',
                  'function',
                ])
                  ElevatedButton(onPressed: () => loadDrill(c), child: Text(c)),
              ],
            ),
            const SizedBox(height: 20),
            Text(conditions, style: const TextStyle(fontSize: 15, height: 1.4)),
            const SizedBox(height: 16),
            Expanded(
              child: CodeTheme(
                data: CodeThemeData(styles: gruvboxDarkTheme),
                child: CodeField(
                  controller: controller,
                  textStyle: const TextStyle(fontFamily: 'monospace'),
                ),
              ),
            ),
            const SizedBox(height: 12),
            ElevatedButton(
              onPressed: expectedAnswer.isEmpty ? null : checkAnswer,
              child: const Text('Check Answer'),
            ),
            if (feedback != null) ...[
              const SizedBox(height: 12),
              Text(
                feedback!,
                style: TextStyle(
                  fontSize: 16,
                  fontWeight: FontWeight.bold,
                  color: feedback == 'Correct!' ? Colors.green : Colors.red,
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }
}
