import 'dart:async';

import 'package:grace/rust/api.dart';
import 'package:grace/rust/api/java.dart';
import 'package:grace/rust/api/rust.dart';
import 'package:grace/rust/api/sql.dart';
import 'package:grace/utils/database/mod.dart';
import 'package:material_ui/material_ui.dart';

class DrillCreate extends StatefulWidget {
  const DrillCreate({super.key, required this.lang, required this.comp});
  final DrillLanguage lang;
  final Completer<void> comp;

  @override
  State<DrillCreate> createState() => _DrillCreate();
}

class _DrillCreate extends State<DrillCreate> {
  late Map<String, bool> concepts;
  late TextEditingController cname;
  bool complete = false;

  @override
  void initState() {
    super.initState();
    final c = switch (widget.lang) {
      .rust =>
        RustConcept.values.map((c) => MapEntry(c.toString(), false)).toList(),
      .java => JavaConcept.values.map((c) => MapEntry(c.toString(), false)),
      .sql => SqlConcept.values.map((c) => MapEntry(c.toString(), false)),
    };

    concepts = Map.fromEntries(c);
    cname = switch (widget.lang) {
      .rust => TextEditingController(text: 'Rust Drills'),
      .java => TextEditingController(text: 'Java Drills'),
      .sql => TextEditingController(text: 'SQL Drills'),
    };
  }

  @override
  void dispose() {
    cname.dispose();
    super.dispose();
  }

  void _saveDrill() async {
    complete = true;
    final name = cname.text;
    final c = concepts.entries
        .map((c) => (c.value) ? c.key : null)
        .whereType<String>()
        .toList();

    final drill = DrillCollection.newWithDefault(
      name: name,
      lang: widget.lang,
      list: c,
    );

    await DatabaseHelper.instance.newCollection(drill);
    widget.comp.complete();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return PopScope(
      onPopInvokedWithResult: (didPop, _) {
        if (didPop && !widget.comp.isCompleted && !complete) {
          widget.comp.complete();
        }
      },
      child: Scaffold(
        appBar: AppBar(title: const Text('New Drill Collection')),
        body: ListView(
          padding: const .all(16),
          children: [
            TextField(
              controller: cname,
              decoration: const InputDecoration(
                labelText: 'Collection Name',
                border: OutlineInputBorder(),
              ),
            ),
            const Divider(),
            Text('Concepts to include', style: theme.textTheme.headlineLarge),
            ...concepts.entries.map(
              (k) => CheckboxListTile(
                title: Text(k.key),
                value: k.value,
                onChanged: (v) => setState(() => concepts[k.key] = v ?? false),
                controlAffinity: .leading,
              ),
            ),

            SizedBox(
              width: double.infinity,
              height: 48,
              child: ElevatedButton.icon(
                onPressed: () {
                  _saveDrill();
                  Navigator.of(context).pop();
                },
                icon: const Icon(Icons.play_arrow),
                label: const Text('Create'),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
