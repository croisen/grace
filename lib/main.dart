import 'package:grace/pages/root.dart';
import 'package:grace/rust/frb_generated.dart';
import 'package:grace/utils/settings.dart';
import 'package:material_ui/material_ui.dart';
import 'package:path/path.dart';
import 'package:path_provider/path_provider.dart';
import 'package:provider/provider.dart';
import 'package:sqflite_ffi/sqflite_ffi.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  var dir = await getApplicationSupportDirectory();
  await databaseFactory.setDatabasesPath(join(dir.path, 'grace.db'));
  var settings = Settings();
  await settings.init();

  runApp(
    ChangeNotifierProvider(create: (a) => settings, child: const GraceApp()),
  );
}
