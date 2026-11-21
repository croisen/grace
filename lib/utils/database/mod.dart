import 'package:grace/rust/api.dart';
import 'package:grace/utils/database/drill_collection.dart';
import 'package:grace/utils/database/drill_memory.dart';
import 'package:sqflite/sqflite.dart';

class DatabaseHelper {
  static final DatabaseHelper instance = DatabaseHelper._();
  static Database? _db;

  DatabaseHelper._();

  Future<Database> get db async {
    if (_db != null) return _db!;
    _db = await _init();
    return _db!;
  }

  Future<Database> _init() async {
    return await openDatabase(
      await databaseFactory.getDatabasesPath(),
      version: 1,
      onCreate: _create,
      onConfigure: _configure,
    );
  }

  Future _create(Database db, int ver) async {
    await db.execute('''
      CREATE TABLE IF NOT EXISTS collections (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL
      );

      CREATE TABLE IF NOT EXISTS drills (
        col_id INTEGER NOT NULL,
        lang TEXT NOT NULL,
        concept TEXT NOT NULL,
        stability REAL,
        difficulty REAL,
        stability_fast REAL,
        last_review INTEGER,
        due INTEGER NOT NULL,
        FOREIGN KEY (col_id) REFERENCES collections(id)
      );
    ''');
  }

  Future _configure(Database db) async {
    await db.execute('PRAGMA foreign_keys = ON');
  }

  Future<int> newCollection(DrillCollection c) async {
    final database = await db;
    return await database.transaction((txn) async {
      final id = await txn.insert('collections', {'name': c.name});
      c.setId(id: BigInt.from(id));
      final (_, mem) = drillCollectionToMap(c);
      for (final drill in mem) {
        await txn.insert('drills', drill);
      }

      return id;
    });
  }

  Future<void> saveCollection(DrillCollection c) async {
    final database = await db;
    return await database.transaction((txn) async {
      await txn.update(
        'collections',
        {'name': c.name},
        where: 'id = ?',
        whereArgs: [c.id.toInt()],
      );

      final (_, mem) = drillCollectionToMap(c);
      for (final drill in mem) {
        await txn.update(
          'drills',
          drill,
          where: 'col_id = ? AND lang = ? AND concept = ?',
          whereArgs: [c.id.toInt(), drill['lang'], drill['concept']],
        );
      }
    });
  }

  Future<List<DrillCollection>> loadCollections() async {
    final database = await db;

    final collections = await database.query('collections');
    final drills = await database.query('drills');

    final c = <int, List<DrillMemory>>{};
    for (final drill in drills) {
      final col = drill['col_id'] as int;
      final d = drillMemoryFromMap(drill);
      c.putIfAbsent(col, () => []).add(d);
    }

    return collections
        .map(
          (col) => DrillCollection(
            id: BigInt.from(col['id'] as num),
            name: col['name'] as String,
            collection: c[col['id'] as num] ?? [],
          ),
        )
        .toList();
  }
}
