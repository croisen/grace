import 'dart:async';

import 'package:grace/pages/drills/drill.dart';
import 'package:grace/rust/api.dart';
import 'package:grace/utils/colors/mod.dart';
import 'package:grace/utils/database/mod.dart';
import 'package:grace/utils/route.dart';
import 'package:grace/utils/settings.dart';
import 'package:material_ui/material_ui.dart';
import 'package:provider/provider.dart';
import 'package:sprintf/sprintf.dart';

class DrillCard extends StatelessWidget {
  const DrillCard({super.key, required this.col, required this.refreshHome});
  final DrillCollection col;
  final void Function() refreshHome;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final p = Provider.of<Settings>(context, listen: true);
    final b = MediaQuery.platformBrightnessOf(context);
    final t = GraceAppTheme(s: p, platformMode: b);
    final now = localNow();

    col.sortByRetrieve();
    final mostDue = col.collection.reversed.first;
    double retrieve = mostDue.retrievability(now: now) * 100;
    Color color;
    if (retrieve > 90) {
      color = t.color_(.blue);
    } else if (retrieve > 75) {
      color = t.color_(.green);
    } else if (retrieve > 50) {
      color = t.color_(.yellow);
    } else {
      color = t.color_(.red);
    }

    return ExpansionTile(
      tilePadding: .all(10),
      title: Column(
        crossAxisAlignment: .start,
        children: [
          Text(col.name, style: theme.textTheme.headlineSmall),
          Text(
            'Most due is `${mostDue.concept}` (${mostDue.dueDateStr()})',
            style: theme.textTheme.bodyLarge?.copyWith(color: color),
          ),
          Row(
            mainAxisAlignment: .spaceAround,
            children: [
              IconButton(
                onPressed: () async {
                  await DatabaseHelper.instance.deleteCollection(col);
                  refreshHome();
                },
                icon: Icon(Icons.delete, color: t.color_(.red)),
              ),
              IconButton(
                onPressed: () async {
                  final comp = Completer();

                  pagePush(
                    context,
                    (context) =>
                        DrillScreen(lang: col.lang, comp: comp, col: col),
                  );

                  await comp.future;
                  refreshHome();
                },
                icon: Icon(Icons.play_arrow, color: t.color_(.green)),
              ),
            ],
          ),
        ],
      ),
      visualDensity: .standard,
      children: col.collection.map((c) {
        double retrieve = c.retrievability(now: now) * 100.0;
        final retain = c.lastReview != null
            ? sprintf("%3.2f%%", [retrieve])
            : 'Unreviewed';
        retrieve = c.lastReview != null ? retrieve / 100 : 1;

        Color color;
        if (c.lastReview == null) {
          color = t.color_(.red);
        } else if (retrieve > 0.9) {
          color = t.color_(.blue);
        } else if (retrieve > 0.75) {
          color = t.color_(.green);
        } else if (retrieve > 0.5) {
          color = t.color_(.yellow);
        } else {
          color = t.color_(.red);
        }

        return Card(
          child: ListTile(
            contentPadding: .symmetric(vertical: 2, horizontal: 10),
            title: Row(
              mainAxisAlignment: .spaceBetween,
              children: [Text(c.concept), Text(retain)],
            ),
            subtitle: Column(
              crossAxisAlignment: .start,
              children: [
                LinearProgressIndicator(value: retrieve, color: color),
                Text(c.dueDateStr()),
              ],
            ),
          ),
        );
      }).toList(),
    );
  }
}
