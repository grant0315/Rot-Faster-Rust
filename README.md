# Rot Faster

A terminal-style roguelike toolkit using raylib and CP437 glyphs.

## Structure

```
src/
  lib.rs            connects the modules together
  char_set.rs       loads and serves glyph textures from cp437_16x16.png
  glyph_buffer.rs   containers, buttons, and text rendering
  main.rs           the actual game (placeholder)
examples/
  demo.rs           test harness for containers, buttons, and glyphs
```

## How to run

| Command | Runs |
|---|---|
| `cargo run --example demo` | Test demo — try the button, see containers wrap |
| `cargo run` | The game (just a blank window for now) |

## Assets

REXPaint `.xp` files are synced from a Windows REXPaint images directory:

```
./sync_assets.sh
```

By default it reads from `/mnt/c/Users/grant/Downloads/REXPaint-v1.70/REXPaint-v1.70/images`.
Override the path with the `REXPAINT_IMAGES_DIR` environment variable:

The library (`lib.rs`) compiles `char_set` and `glyph_buffer` once. Both the game and the demo share the same code — fix a bug in the library and both benefit.
