# Art credits

The world images are **in-game captures** from the Scoria/Horizon test worlds,
taken from `~/Pictures/horizon-visuals-*.png` and cropped between the tutorial
overlay and the HUD. They are not stock photos, AI generations, or Mojang
assets: they are renders produced by this project's own engine during visual
testing.

| File | Source capture | Crop |
|---|---|---|
| `hero-new-world.png` | `horizon-visuals-final-compare.png` | day beach, 5:6 portrait |
| `thumb-new-world.png` | `horizon-visuals-final-compare.png` | day beach, 16:9 |
| `thumb-build.png` | `horizon-visuals-day-compare.png` | forest + stone build, 16:9 |
| `thumb-hardcore.png` | `horizon-visuals-ingame-compare.png` | night beach, 16:9, ember grade |
| `hero-soft.png` | `horizon-visuals-final-compare.png` | day beach, 1:1 |

Processing is in `mc_art.py` (crop, grade, vignette, grain).

These captures are for the local design study. Do not publish them as game
assets; before a public release, replace them with screenshots from the
player's own worlds.
