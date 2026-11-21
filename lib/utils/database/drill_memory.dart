import 'package:grace/rust/api.dart';

Map<String, Object?> drillMemoryToMap(DrillMemory m) {
  final lang = switch (m.lang) {
    DrillLanguage.java => 'JAVA',
    DrillLanguage.rust => 'RUST',
    DrillLanguage.sql => 'SQL',
  };

  return {
    if (m.colId != BigInt.zero) 'col_id': m.colId.toInt(),
    'lang': lang,
    'concept': m.concept,
    'stability': m.memory?.stability,
    'difficulty': m.memory?.difficulty,
    'stability_fast': m.memory?.stabilityFast,
    'last_review': m.lastReview,
    'due': m.dueDate,
  };
}

DrillMemory drillMemoryFromMap(Map<String, Object?> map) {
  final lang = switch (map['lang']) {
    'JAVA' => DrillLanguage.java,
    'RUST' => DrillLanguage.rust,
    'SQL' => DrillLanguage.sql,
    final value => throw FormatException('Unknown drill language: $value'),
  };

  final stability = map['stability'];
  final difficulty = map['difficulty'];
  final stabilityFast = map['stability_fast'];

  return DrillMemory(
    id: BigInt.from(map['col_id'] as num),
    lang: lang,
    concept: map['concept'] as String,
    memory: stability == null || difficulty == null || stabilityFast == null
        ? null
        : MemoryState(
            stability: stability as double,
            difficulty: difficulty as double,
            stabilityFast: stabilityFast as double,
          ),
    lastReview: map['last_review'] as int?,
    dueDate: map['due'] as int,
  );
}
