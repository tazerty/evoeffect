# EvoEffect app icon

**Creature:** a unicorn, as an engraved head-and-shoulders bust looking out of the tile.

**Style:** an ink engraving portrait in the Building Apps' owl-template framing (the EvoFilm owl):
full-bleed colour field, tight head-and-shoulders crop, the body running off the bottom and right edges.

**Tile:** 512 × 512 viewBox, rounded square `rx=112`, clipped; no border or roundel.

**Palette** (exactly three colours):

| Colour | Hex | Used for |
|---|---|---|
| App colour (EvoEffect pink) | `#e0368f` | the full-bleed field |
| Paper | `#efe9dc` | the figure |
| Ink | `#0b0b0c` | line work and contour |

**Provenance:** the project owner drew it in Upstream (a 2880 px engraving-style image, kept in
evorules at `assets/app-icons/evoeffect/source.png`). It was vectorised with evorules
`assets/logo-options/_tools/vectorize_tile.py`, which keys out the field and traces the ink with potrace.
Licence: see [LICENSE.txt](LICENSE.txt) (MIT OR Apache-2.0, like the repo).

## Files

| File | What |
|---|---|
| `evoeffect.svg` | master vector, traced at 2048 px (canonical) |
| `evoeffect-small.svg` | lighter vector (~350 KB), traced at 1024 px; also the web favicon |
| `evoeffect-1024.png` | 1024 px render |
| `evoeffect-macos-512.png` | runtime Dock icon on macOS, on Apple's 824/1024 grid |
| `evoeffect.icns` | macOS icon (Apple grid) |
| `evoeffect.ico` | Windows icon, 16–256 px; embedded in `evoeffect.exe` by `apps/evoeffect/build.rs` |
| `hicolor/<n>x<n>/apps/ai.storyteller.evoeffect.png` | Linux icon theme, 16–512 px; the 256 px one is the runtime icon on Windows and Linux |
| `hicolor/scalable/apps/ai.storyteller.evoeffect.svg` | Linux scalable icon |

App ID: `ai.storyteller.evoeffect` (Wayland app ID, `packaging/linux/ai.storyteller.evoeffect.desktop`).

## Regenerate

Copy a new `icon-master.svg` / `icon.svg` from evorules `assets/app-icons/evoeffect/` over
`evoeffect.svg` / `evoeffect-small.svg`, then run `packaging/icons.sh` (needs `resvg`; `iconutil`
on macOS for the `.icns`). Copy `evoeffect-small.svg` to `apps/evoeffect-web/web/favicon.svg` too.
