# Oracle Suite app icon

The app icon is the **Oracle mark**: a red disc (#d31c3b to #e52c54) with a white four-pointed star,
from [`docs/brand/oracle-mark.svg`](../../docs/brand/oracle-mark.svg). These files are the project's
brand mark, not covered by the code's MIT OR Apache-2.0 licence (see
[`docs/brand/LICENSE-brand.txt`](../../docs/brand/LICENSE-brand.txt)).

- `oracle-suite.svg`: the mark; also the hicolor scalable icon.
- `oracle-suite-1024.png`, `oracle-suite.ico`, `oracle-suite.icns` and `hicolor/`: rendered from it by
  `packaging/icons.sh` (the `.ico` is packed by `cargo xtask ico`).
- The tray / menu-bar icon is the same mark: `docs/brand/oracle-mark-black-44.png` (macOS template,
  22 pt @2x) and `docs/brand/oracle-mark-64.png` (Windows, Linux, FreeBSD).

Never edit the renders by hand: change the source and run `packaging/icons.sh`.
