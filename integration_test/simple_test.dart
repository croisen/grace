import 'package:flutter_test/flutter_test.dart';
import 'package:grace/rust/frb_generated.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  setUpAll(() async {
    await RustLib.init();
  });

  testWidgets('Grace UI and Rust drill generators work', (tester) async {});
}
