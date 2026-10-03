import 'package:flutter/material.dart';
import 'package:grace/src/rust/api/simple.dart';
import 'package:grace/src/rust/frb_generated.dart';

import 'package:flutter_highlight/themes/gruvbox-dark.dart';
import 'package:flutter_code_editor/flutter_code_editor.dart';
import 'package:highlight/languages/rust.dart';

final controller = CodeController(language: rust);

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      home: Scaffold(
        appBar: AppBar(title: const Text('Grace')),
        body: CodeTheme(
          data: CodeThemeData(styles: gruvboxDarkTheme),
          child: SingleChildScrollView(
            child: CodeField(controller: controller),
          ),
        ),
      ),
    );
  }
}
