import 'package:flutter/foundation.dart';
import 'package:grace/pages/root.dart';
import 'package:grace/rust/frb_generated.dart';
import 'package:grace/utils/settings.dart';
import 'package:material_ui/material_ui.dart';
import 'package:path/path.dart';
import 'package:path_provider/path_provider.dart';
import 'package:provider/provider.dart';
import 'package:sqflite_common_ffi_web/sqflite_ffi_web.dart';
import 'package:sqflite_ffi/sqflite_ffi.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();

  if (!kIsWeb) {
    final dir = await getApplicationSupportDirectory();
    await databaseFactory.setDatabasesPath(join(dir.path, 'grace.db'));
  } else {
    databaseFactory = databaseFactoryFfiWeb;
    await databaseFactory.setDatabasesPath('grace.db');
  }

  final settings = Settings();
  await settings.init();

  runApp(
    ChangeNotifierProvider(create: (a) => settings, child: const GraceApp()),
  );
}
