import 'package:flutter/material.dart';

class DeckTile extends StatelessWidget {
  final String name;
  final int newCount;
  final int learnCount;
  final int dueCount;
  final VoidCallback onTap;

  const DeckTile({
    super.key,
    required this.name,
    required this.newCount,
    required this.learnCount,
    required this.dueCount,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return ListTile(
      contentPadding: const EdgeInsets.symmetric(horizontal: 20, vertical: 4),
      title: Text(name, style: const TextStyle(fontSize: 16, fontWeight: FontWeight.w500)),
      subtitle: Padding(
        padding: const EdgeInsets.only(top: 4),
        child: Row(
          children: [
            _CountLabel(label: 'New', count: newCount, color: Colors.blue),
            const SizedBox(width: 12),
            _CountLabel(label: 'Learn', count: learnCount, color: Colors.red),
            const SizedBox(width: 12),
            _CountLabel(label: 'Due', count: dueCount, color: Colors.green),
          ],
        ),
      ),
      trailing: const Icon(Icons.chevron_right, color: Colors.grey),
      onTap: onTap,
    );
  }
}

class _CountLabel extends StatelessWidget {
  final String label;
  final int count;
  final Color color;

  const _CountLabel({required this.label, required this.count, required this.color});

  @override
  Widget build(BuildContext context) {
    return Text(
      '$count $label',
      style: TextStyle(fontSize: 12, color: color, fontWeight: FontWeight.w500),
    );
  }
}
