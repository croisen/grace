import 'package:grace/rust/api.dart';
import 'package:grace/utils/database/drill_memory.dart';

(Map<String, Object?>, List<Map<String, Object?>>) drillCollectionToMap(
  DrillCollection c,
) {
  final map = {if (c.id != BigInt.zero) 'id': c.id, 'name': c.name};
  final list = c.collection.map((m) => drillMemoryToMap(m)).toList();

  return (map, list);
}

DrillCollection drillCollectionFromMap(
  Map<String, Object?> c,
  List<Map<String, Object?>> m,
) {
  final id = BigInt.from(c['id'] as num);
  final name = c['name'] as String;
  final list = m.map((m) => drillMemoryFromMap(m)).toList();

  return DrillCollection(id: id, name: name, collection: list);
}
