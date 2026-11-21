import 'package:material_ui/material_ui.dart';
import 'package:grace/rust/api.dart';
import 'package:grace/utils/database/mod.dart';

class DrillsScreen extends StatefulWidget {
  DrillsScreen({super.key});
  final state = _DrillsScreen();

  void refresh() {
    state.refresh();
  }

  @override
  State<DrillsScreen> createState() => state;
}

class _DrillsScreen extends State<DrillsScreen> {
  late Future<List<DrillCollection>> _collections;
  @override
  void initState() {
    super.initState();
    _loadCollections();
  }

  void _loadCollections() {
    _collections = DatabaseHelper.instance.loadCollections();
  }

  Future<void> refresh() async {
    setState(_loadCollections);
    await _collections;
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: _collections,
      builder: (context, s) {
        if (s.connectionState == ConnectionState.waiting) {
          return const Center(child: CircularProgressIndicator());
        }

        if (s.hasError) {
          return Center(child: Text('Could not load collections: ${s.error}'));
        }

        final collections = s.data ?? [];
        if (collections.isEmpty) {
          return const Center(
            child: Text(
              'No decks yet.\nTap + to create one.',
              textAlign: TextAlign.center,
              style: TextStyle(fontSize: 16, color: Colors.grey),
            ),
          );
        }

        return RefreshIndicator(
          onRefresh: refresh,
          child: ListView.builder(
            itemCount: collections.length,
            itemBuilder: (context, idx) {
              final col = collections[idx];
              return ListTile(
                title: Text(col.name),
                subtitle: Text('${col.collection.length} drills'),
              );
            },
          ),
        );
      },
    );
  }
}
