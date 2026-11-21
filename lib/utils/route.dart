import 'package:material_ui/material_ui.dart';

Future<T?> pagePush<T extends Object?>(
  BuildContext context,
  Widget Function(BuildContext context) builder,
) {
  return Navigator.push(context, MaterialPageRoute(builder: builder));
}

Future<T?> pageReplace<T extends Object?>(
  BuildContext context,
  Widget Function(BuildContext context) builder,
) {
  return Navigator.pushReplacement(
    context,
    MaterialPageRoute(builder: builder),
  );
}
