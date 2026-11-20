import 'package:flutter_test/flutter_test.dart';
import 'package:grace/rust/api/rust.dart';
import 'package:grace/rust/frb_generated.dart';
import 'package:grace/ui/grace_app.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  setUpAll(() async {
    await RustLib.init();
  });

  testWidgets('Grace UI and Rust drill generators work', (tester) async {
    await tester.pumpWidget(const GraceApp());

    expect(find.byType(GraceApp), findsOneWidget);

    final variable = await generateVariable(
      choice: BigInt.from(1),
      ifMut: BigInt.from(0),
    );
    expect(variable.conditions.trim(), isNotEmpty);
    expect(variable.code.trim(), isNotEmpty);

    final ifDrill = generateIfStatement();
    expect(ifDrill.conditions.trim(), isNotEmpty);
    expect(ifDrill.answer.trim(), isNotEmpty);

    final elseIfDrill = generateElseIf();
    expect(elseIfDrill.conditions.trim(), isNotEmpty);
    expect(elseIfDrill.answer.trim(), isNotEmpty);

    final matchDrill = generateMatchStatement();
    expect(matchDrill.conditions.trim(), isNotEmpty);
    expect(matchDrill.answer.trim(), isNotEmpty);

    final whileDrill = generateWhileStatement();
    expect(whileDrill.conditions.trim(), isNotEmpty);
    expect(whileDrill.answer.trim(), isNotEmpty);

    final forDrill = generateForStatement();
    expect(forDrill.conditions.trim(), isNotEmpty);
    expect(forDrill.answer.trim(), isNotEmpty);

    final loopDrill = generateLoopStatement();
    expect(loopDrill.conditions.trim(), isNotEmpty);
    expect(loopDrill.answer.trim(), isNotEmpty);

    final functionDrill = generateFunction();
    expect(functionDrill.conditions.trim(), isNotEmpty);
    expect(functionDrill.answer.trim(), isNotEmpty);
  });
}
