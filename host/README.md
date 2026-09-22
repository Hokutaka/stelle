# Stelle host

This is the temporary native host for Stelle.

It owns only the platform-facing parts:

- window creation
- keyboard input
- framebuffer drawing
- frame timing

`src/game.rs` currently mirrors the tiny Cerune game core because Cerune does not
yet expose a general host-call / FFI boundary suitable for a real-time input loop.

The intended direction is to replace `game.rs` with a Cerune bridge while keeping
`main.rs` as the platform host.

## Run

From the repository root:

```sh
cargo run --manifest-path host/Cargo.toml
```

Controls:

- `WASD` or arrow keys: move
- `Esc`: quit
