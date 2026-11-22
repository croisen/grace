import 'package:grace/widgets/drill_card.dart';
import 'package:material_ui/material_ui.dart';
import 'package:grace/rust/api.dart';
import 'package:grace/utils/database/mod.dart';

class DrillsScreen extends StatefulWidget {
  const DrillsScreen({super.key});

  @override
  State<DrillsScreen> createState() => _DrillsScreen();
}

class _DrillsScreen extends State<DrillsScreen> {
  late Future<List<DrillCollection>> _collections;

  @override
  void initState() {
    super.initState();
    _loadCollections();
  }

  @override
  void setState(void Function() f) {
    _loadCollections();
    super.setState(f);
  }

  void _loadCollections() {
    _collections = DatabaseHelper.instance.loadCollections();
  }

  Future<void> refresh() async {
    setState(() {});
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
          child: ListView.separated(
            padding: .symmetric(vertical: 5, horizontal: 10),
            itemCount: collections.length,
            itemBuilder: (context, idx) {
              final col = collections[idx];
              return DrillCard(col: col, refreshHome: refresh);
            },
            separatorBuilder: (context, idx) => const SizedBox(height: 10),
          ),
        );
      },
    );
  }
}
