import 'package:grace/rust/api.dart';
import 'package:grace/utils/database/drill_memory.dart';

(Map<String, Object?>, List<Map<String, Object?>>) drillCollectionToMap(
  DrillCollection c,
) {
  final lang = switch (c.lang) {
    DrillLanguage.java => 'JAVA',
    DrillLanguage.rust => 'RUST',
    DrillLanguage.sql => 'SQL',
  };

  final map = {
    if (c.id != BigInt.zero) 'id': c.id,
    'lang': lang,
    'name': c.name,
  };

  final list = c.collection.map((m) => drillMemoryToMap(m)).toList();

  return (map, list);
}

DrillCollection drillCollectionFromMap(
  Map<String, Object?> c,
  List<Map<String, Object?>> m,
) {
  final id = BigInt.from(c['id'] as num);
  final name = c['name'] as String;
  final lang = switch (c['lang']) {
    'JAVA' => DrillLanguage.java,
    'RUST' => DrillLanguage.rust,
    'SQL' => DrillLanguage.sql,
    final value => throw FormatException('Unknown drill language: $value'),
  };

  final list = m.map((m) => drillMemoryFromMap(m)).toList();

  return DrillCollection(id: id, lang: lang, name: name, collection: list);
}
