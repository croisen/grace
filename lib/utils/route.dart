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

void snackBarError(BuildContext context, String message) {
  final theme = Theme.of(context);
  ScaffoldMessenger.of(context).showSnackBar(
    SnackBar(
      content: Text(
        message,
        style: theme.textTheme.bodyLarge?.copyWith(
          color: theme.colorScheme.error,
        ),
      ),
    ),
  );
}
