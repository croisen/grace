import 'package:flutter/material.dart';

/// Full-bleed terminal-style editor (no bezel), with line numbers.
class TerminalEditor extends StatelessWidget {
  final TextEditingController controller;
  final bool dark;
  final String palette;

  const TerminalEditor({
    super.key,
    required this.controller,
    required this.dark,
    required this.palette,
  });

  Color get _bg {
    if (!dark) return const Color(0xFFF8F8F2);
    switch (palette) {
      case 'Gruvbox':     return const Color(0xFF282828);
      case 'Tokyo Night': return const Color(0xFF1A1B26);
      case 'Dracula':     return const Color(0xFF282A36);
      case 'Nord':        return const Color(0xFF2E3440);
      case 'Catppuccin':  return const Color(0xFF1E1E2E);
      case 'One Dark':    return const Color(0xFF282C34);
      case 'Solarized':   return const Color(0xFF002B36);
      default:            return const Color(0xFF1E1E1E);
    }
  }

  Color get _fg {
    if (!dark) return const Color(0xFF24292E);
    switch (palette) {
      case 'Gruvbox':     return const Color(0xFFEBDBB2);
      case 'Tokyo Night': return const Color(0xFFA9B1D6);
      case 'Dracula':     return const Color(0xFFF8F8F2);
      case 'Nord':        return const Color(0xFFD8DEE9);
      case 'Catppuccin':  return const Color(0xFFCDD6F4);
      case 'One Dark':    return const Color(0xFFABB2BF);
      case 'Solarized':   return const Color(0xFF839496);
      default:            return const Color(0xFFD4D4D4);
    }
  }

  Color get _lineNumberColor => _fg.withOpacity(0.35);

  @override
  Widget build(BuildContext context) {
    return Container(
      color: _bg, // full bleed – no border, no radius
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Line numbers
          Container(
            width: 48,
            padding: const EdgeInsets.only(top: 16, right: 10),
            child: ValueListenableBuilder<TextEditingValue>(
              valueListenable: controller,
              builder: (_, value, __) {
                final lineCount = value.text.split('\n').length;
                final displayLines = lineCount < 20 ? 20 : lineCount;
                return Column(
                  crossAxisAlignment: CrossAxisAlignment.end,
                  children: List.generate(displayLines, (i) {
                    return SizedBox(
                      height: 22,
                      child: Text(
                        '${i + 1}',
                        style: TextStyle(
                          fontFamily: 'monospace',
                          fontSize: 13,
                          height: 1.6,
                          color: _lineNumberColor,
                        ),
                      ),
                    );
                  }),
                );
              },
            ),
          ),
          // Code area
          Expanded(
            child: TextField(
              controller: controller,
              maxLines: null,
              expands: true,
              textAlignVertical: TextAlignVertical.top,
              style: TextStyle(
                fontFamily: 'monospace',
                fontSize: 15,
                height: 1.6,
                color: _fg,
              ),
              cursorColor: _fg,
              decoration: InputDecoration(
                border: InputBorder.none,
                contentPadding: const EdgeInsets.fromLTRB(4, 16, 16, 16),
                hintText: '// write your code here',
                hintStyle: TextStyle(color: _fg.withOpacity(0.3)),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
