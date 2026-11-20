import 'package:flutter_test/flutter_test.dart';
import 'package:grace/ui/grace_app.dart';

void main() {
  testWidgets('Grace app mounts successfully', (tester) async {
    await tester.pumpWidget(const GraceApp());

    expect(find.byType(GraceApp), findsOneWidget);
  });
}
