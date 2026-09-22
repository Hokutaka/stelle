# Stelle

**Stelle** is a tiny 2D RPG experiment written around
[Cerune](https://github.com/Hokutaka/Cerune).

The project now has two deliberately separate layers:

```text
src/*.ceru       Cerune game core
host/            native window / input / rendering host
```

## 1. Run the Cerune core

Install Cerune:

```sh
git clone https://github.com/Hokutaka/Cerune.git
cd Cerune
cargo install --path .
```

Then in the Stelle repository:

```sh
cerune check src/main.ceru
cerune run src/main.ceru
```

The Cerune example exercises:

- player state
- four-direction movement
- a fixed tile map
- wall collision
- modules, product types, enums, and match

## 2. Run the graphical host

```sh
cargo run --manifest-path host/Cargo.toml
```

Controls:

- `WASD` or arrow keys: move one tile
- `Esc`: quit

The host opens a small window, draws the 7x5 map, and renders the player as a
square.

## Current boundary

Cerune currently has no general real-time host-call / FFI boundary for the game
loop, so `host/src/game.rs` temporarily mirrors the tiny movement/map rules from
the Cerune source.

That duplication is intentional and isolated. The target architecture is:

```text
Cerune
  Player / World / Battle / Events
            |
         Host ABI
            |
Rust host
  Window / Input / Rendering / Audio
```

When Cerune grows the required host boundary, `host/src/game.rs` can be replaced
without rewriting the window/rendering layer.

## Next small milestone

A useful next step is to add a narrow Cerune host ABI with calls shaped roughly
like:

```text
key_down(key)
draw_rect(x, y, width, height)
present()
```

or, preferably, expose game-state stepping from Cerune while keeping platform
effects in the Rust host.
