import 'package:flutter/material.dart';
import 'package:grace/rust/frb_generated.dart';
import 'package:grace/screens/drill_screen.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return const MaterialApp(
      debugShowCheckedModeBanner: false,
      home: DrillScreen(),
    );
  }
}
