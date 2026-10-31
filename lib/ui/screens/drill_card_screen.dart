import 'dart:math';

import 'package:flutter/material.dart';

// Real generators
import 'package:grace/rust/api/drills/rust/variable.dart';
import 'package:grace/rust/api/drills/rust/if_statement.dart';
import 'package:grace/rust/api/drills/rust/else_if.dart';
import 'package:grace/rust/api/drills/rust/match_statement.dart';
import 'package:grace/rust/api/drills/rust/while_statement.dart';
import 'package:grace/rust/api/drills/rust/for_statement.dart';
import 'package:grace/rust/api/drills/rust/loop_statement.dart';
import 'package:grace/rust/api/drills/rust/function.dart';

import '../widgets/terminal_editor.dart';

class DrillCardScreen extends StatefulWidget {
  final String deckName;
  final int totalCards;
  final List<String> concepts;
  final bool terminalDark;
  final String terminalPalette;

  const DrillCardScreen({
    super.key,
    required this.deckName,
    required this.totalCards,
    required this.concepts,
    required this.terminalDark,
    required this.terminalPalette,
  });

  @override
  State<DrillCardScreen> createState() => _DrillCardScreenState();
}

class _DrillCardScreenState extends State<DrillCardScreen> {
  final TextEditingController _codeController = TextEditingController();
  final _rng = Random();

  int currentIndex = 1;
  bool showConditions = true;

  String conditions = '';
  String expectedAnswer = '';

  @override
  void initState() {
    super.initState();
    _loadCard();
  }

  @override
  void dispose() {
    _codeController.dispose();
    super.dispose();
  }

  Future<void> _loadCard() async {
    if (widget.concepts.isEmpty) {
      setState(() {
        conditions = 'No concepts selected.';
        expectedAnswer = '';
      });
      return;
    }

    final concept = widget.concepts[_rng.nextInt(widget.concepts.length)];

    try {
      switch (concept) {
        case 'Variables':
          // choice: 1 = i32 (most common), ifMut: 0 or 1
          final r = await generateVariable(
            choice: BigInt.from(1),
            ifMut: BigInt.from(_rng.nextInt(2)),
          );
          conditions = r.conditions;
          expectedAnswer = r.code;
          break;

        case 'If / Else If':
          if (_rng.nextBool()) {
            final r = generateIfStatement();
            conditions = r.conditions;
            expectedAnswer = r.answer;
          } else {
            final r = generateElseIf();
            conditions = r.conditions;
            expectedAnswer = r.answer;
          }
          break;

        case 'Match':
          final r = generateMatchStatement();
          conditions = r.conditions;
          expectedAnswer = r.answer;
          break;

        case 'While':
          final r = generateWhileStatement();
          conditions = r.conditions;
          expectedAnswer = r.answer;
          break;

        case 'For':
          final r = generateForStatement();
          conditions = r.conditions;
          expectedAnswer = r.answer;
          break;

        case 'Loop':
          final r = generateLoopStatement();
          conditions = r.conditions;
          expectedAnswer = r.answer;
          break;

        case 'Functions':
          final r = generateFunction();
          conditions = r.conditions;
          expectedAnswer = r.answer;
          break;

        default:
          conditions = 'Unknown concept: $concept';
          expectedAnswer = '';
      }
    } catch (e) {
      conditions = '• Concept: $concept\n• Generator error: $e';
      expectedAnswer = '';
    }

    if (mounted) setState(() {});
  }

  void _toggleConditions() {
    setState(() => showConditions = !showConditions);
  }

  void _dismissConditions() {
    if (showConditions) setState(() => showConditions = false);
  }

  void _onSkip() => _goToNext();

  void _onSubmit() {
    // TODO: real structural checker (compare _codeController.text vs expectedAnswer)
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Submitted (checker not implemented yet)')),
    );
    Future.delayed(const Duration(milliseconds: 500), _goToNext);
  }

  void _goToNext() {
    if (currentIndex >= widget.totalCards) {
      showDialog(
        context: context,
        builder: (ctx) => AlertDialog(
          title: const Text('Session complete'),
          content: Text('You finished ${widget.totalCards} cards.'),
          actions: [
            TextButton(
              onPressed: () {
                Navigator.of(ctx).pop();
                Navigator.of(context).popUntil((r) => r.isFirst);
              },
              child: const Text('Back to Decks'),
            ),
          ],
        ),
      );
      return;
    }

    setState(() {
      currentIndex++;
      _codeController.clear();
      showConditions = true;
    });
    _loadCard();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(
          '${widget.deckName}  •  $currentIndex / ${widget.totalCards}',
        ),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => Navigator.pop(context),
        ),
      ),
      body: Stack(
        children: [
          // Full-bleed terminal
          Positioned.fill(
            child: GestureDetector(
              onTap: _dismissConditions,
              behavior: HitTestBehavior.opaque,
              child: TerminalEditor(
                controller: _codeController,
                dark: widget.terminalDark,
                palette: widget.terminalPalette,
              ),
            ),
          ),

          // Conditions overlay
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
                            const Text(
                              'Conditions',
                              style: TextStyle(
                                fontWeight: FontWeight.w600,
                                fontSize: 15,
                              ),
                            ),
                            const Spacer(),
                            IconButton(
                              icon: const Icon(Icons.close, size: 20),
                              onPressed: _dismissConditions,
                              visualDensity: VisualDensity.compact,
                            ),
                          ],
                        ),
                        const SizedBox(height: 6),
                        Text(
                          conditions,
                          style: const TextStyle(
                            fontFamily: 'monospace',
                            fontSize: 13,
                            height: 1.45,
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ),

          // Skip + Submit only when conditions are open
          if (showConditions)
            Positioned(
              left: 16,
              right: 16,
              bottom: 24,
              child: Row(
                children: [
                  Expanded(
                    child: OutlinedButton(
                      onPressed: _onSkip,
                      style: OutlinedButton.styleFrom(
                        minimumSize: const Size.fromHeight(48),
                        backgroundColor: Theme.of(context).cardColor
                            .withOpacity(0.9),
                      ),
                      child: const Text('Skip'),
                    ),
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: ElevatedButton(
                      onPressed: _onSubmit,
                      style: ElevatedButton.styleFrom(
                        minimumSize: const Size.fromHeight(48),
                      ),
                      child: const Text('Submit'),
                    ),
                  ),
                ],
              ),
            ),
        ],
      ),
      floatingActionButtonLocation: FloatingActionButtonLocation.endFloat,
      floatingActionButton: Padding(
        padding: EdgeInsets.only(bottom: showConditions ? 70 : 16),
        child: FloatingActionButton(
          onPressed: _toggleConditions,
          child: const Text(
            '!',
            style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
          ),
        ),
      ),
    );
  }
}
