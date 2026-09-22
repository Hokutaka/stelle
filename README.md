# Stelle

**Stelle** is a tiny scripted CLI RPG written entirely in
[Cerune](https://github.com/Hokutaka/Cerune).

One game, one source of truth. No Rust-side copy of the game logic.

## Run

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

The demo runs a tiny battle:

```text
Stelle
A slime appears!
...
> attack
...
> heal
...
```

## Structure

```text
src/main.ceru      CLI scenario / command source
src/battle.ceru    battle state and game rules
```

`battle.ceru` owns the actual RPG state transitions. `main.ceru` only chooses
which actions happen and prints the resulting state.

## Why the GUI host is gone

The first prototype had a Cerune game core plus a Rust window/input host. Cerune
currently has no general real-time stdin/host-call boundary, so the Rust host
ended up mirroring the map and movement rules. That created two implementations
of the same game.

For this experiment, that is more machinery than the game needs. Stelle now
backs up to the smallest useful shape: a Cerune-only CLI RPG with no duplicated
Rust game core.

## Current limitation

Cerune can print values but does not yet expose an interactive stdin primitive,
so the commands in `src/main.ceru` are scripted for now.

When Cerune gains input, Stelle should only need to replace the scripted action
source with something like `read_action()`. The battle rules can stay exactly
where they are in Cerune.
