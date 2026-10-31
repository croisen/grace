import 'package:flutter/material.dart';
import 'package:grace/rust/frb_generated.dart';
import 'package:grace/ui/grace_app.dart'; // ← new UI entry point

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized(); // good practice (boi does this trigger some memories)
  await RustLib.init();
  runApp(const GraceApp()); // ← launches the modular UI
}
