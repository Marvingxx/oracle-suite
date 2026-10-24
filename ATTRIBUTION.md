# Attribution

Every non-code asset in this repository, with its author, source and license. Oracle
Suite's own code and original assets are MIT OR Apache-2.0 (see [`LICENSE-MIT`](LICENSE-MIT),
[`LICENSE-APACHE`](LICENSE-APACHE) and [`NOTICE`](NOTICE)). When you add an asset, add a row here
in the same change; third-party assets must be permissively licensed and keep their license file
next to them.

## Bundled in the app

| Path | Title | Author | Source | License |
|---|---|---|---|---|
| `assets/app-icon/` (all files) | Oracle Suite app icon: the Oracle mark (red disc, four-pointed star) | Oracle | Rendered from `docs/brand/oracle-mark.svg` by `packaging/icons.sh` | Oracle brand mark, [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt) |
| `packaging/macos/dmg/` (all files) | macOS DMG window background and layout | Oracle Suite contributors | Original work, see [`packaging/macos/dmg/README.md`](packaging/macos/dmg/README.md) | MIT OR Apache-2.0 |
| `assets/social/github.svg`, `assets/social/globe.svg` | GitHub logo and a globe, linking to the Oracle apps' repositories | [Simple Icons](https://simpleicons.org/) contributors; [Lucide](https://lucide.dev) contributors | Simple Icons `github.svg` and Lucide `globe.svg` (lucide-static 0.468.0), white for tinting; see [`assets/social/README.md`](assets/social/README.md) | CC0 1.0 (GitHub logo; a trademark of GitHub, Inc.); ISC (globe) |
| `docs/screenshots/oracle-suite-grid.png` | Screenshot of Oracle Suite (Windows, grid view), used in the README | Oracle Suite contributors | Captured from the app | MIT OR Apache-2.0 for the screenshot itself; the ArtCraft logo in it stays an ArtCraft trademark ([`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt)) and the app icons in it keep their own licenses (above) |
| (not bundled) the pictures on each app's details page | Screenshots of the Oracle apps, fetched when a page is opened and kept in the launcher's cache folder | The apps' contributors | [Marvingxx/oracle-suite-assets](https://github.com/Marvingxx/oracle-suite-assets), pinned to a tag (`SCREENSHOTS_REV` in `crates/core/src/details.rs`); each picture's source and license are listed in that repository's `ATTRIBUTION.md` | Each app's own license for the interface (MIT OR Apache-2.0); the work shown in them is public domain or made in the app, as that file says |
| `assets/fonts/Inter-Variable.ttf` | Inter 4.001, variable (font metrics for the CJK fallback) | The Inter Project Authors (Rasmus Andersson) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/inter> (`Inter[opsz,wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-Inter.txt`](assets/fonts/OFL-Inter.txt) |
| `assets/fonts/Archivo-Variable.ttf` | Archivo 2.001, variable (the Oracle wordmark's and app icons' lettering, as outlines) | The Archivo Project Authors (Omnibus-Type) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/archivo> (`Archivo[wdth,wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-Archivo.txt`](assets/fonts/OFL-Archivo.txt) |
| `assets/fonts/GeistMono-Variable.ttf` | Geist Mono 1.701, variable (monospace text) | The Geist Project Authors (Vercel) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/geistmono> (`GeistMono[wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-GeistMono.txt`](assets/fonts/OFL-GeistMono.txt) |
| `assets/fonts/Lexend-Variable.ttf` | Lexend, variable (body text, headings, labels and buttons) | The Lexend Project Authors | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/lexend> (`Lexend[wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-Lexend.txt`](assets/fonts/OFL-Lexend.txt) |

### Oracle Suite icons

The launcher shows each Oracle app with that app's own icon. They are copied unmodified from each
app's repository: the Oracle app tiles drawn by Oracle-suite/tools/make_app_icon.py, licensed like
those apps. Refresh them from the source path, never redraw them.

| Path | Title | Author | Source | License |
|---|---|---|---|---|
| `assets/app-icons/oracle-edit.png` | oracle-edit app icon, 128 px | Oracle | `Marvingxx/oracle-edit`, copied unmodified | MIT OR Apache-2.0 |
| `assets/app-icons/oracle-vector.png` | oracle-vector app icon, 128 px | Oracle | `Marvingxx/oracle-vector`, copied unmodified | MIT OR Apache-2.0 |
| `assets/app-icons/oracle-brand-kit.png` | oracle-brand-kit app icon, 128 px | Oracle | `Marvingxx/oracle-brand`, copied unmodified | MIT OR Apache-2.0 |
| `assets/app-icons/oracle-mockup-studio.png` | oracle-mockup-studio app icon, 128 px | Oracle | `Marvingxx/oracle-brand`, copied unmodified | MIT OR Apache-2.0 |
| `assets/app-icons/oracle-guide.png` | oracle-guide app icon, 128 px | Oracle | `Marvingxx/oracle-guide`, copied unmodified | MIT OR Apache-2.0 |
| `assets/app-icons/oracle-motion.png` | oracle-motion app icon, 128 px | Oracle | `Marvingxx/oracle-motion`, copied unmodified | MIT OR Apache-2.0 |
| `assets/app-icons/oracle-video.png` | oracle-video app icon, 128 px | Oracle | `Marvingxx/oracle-video`, copied unmodified | MIT OR Apache-2.0 |

## Documentation and README

| Path | Title | Author | Source | License |
|---|---|---|---|---|
| `docs/brand/` (all files) | Oracle name, logo and mark | Oracle | original work | Not open source; Oracle's brand marks, [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt) |
