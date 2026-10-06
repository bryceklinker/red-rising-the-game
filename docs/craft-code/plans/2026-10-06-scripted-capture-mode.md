# Scripted capture mode — play a bit of the game headlessly, grab screenshots/video

Companion to `docs/craft-ops/pipelines/2026-10-06-pr-screenshot-capture.md`
(the CI wiring). This doc is the game-side capability that CI piece runs:
a way to drive a short, scripted bit of the real game and capture
screenshots to disk, locally or headlessly. Research grounding:
`explore-pr-screenshot-capture` (Bevy 0.19.1 APIs, verified against
official examples/docs, not memory).

## Why a separate capability, not a test

`cargo test`'s headless `App` + `MinimalPlugins` pattern (used throughout
M0) proves *logic*, not *pixels* — it never renders a frame. This is the
opposite: real rendering, real camera, real `GameState` transitions,
exercised by a script instead of a human's keyboard, so a PR reviewer (or
you, locally) can see what the game actually looks like without opening a
window and playing it by hand.

## Architecture

New files, following the existing plain-function-plugin convention
(`src/plugins/*.rs`, `fn plugin(app: &mut App)` — not `impl Plugin`):

- `src/capture/script.rs` — pure data: `ScriptStep` enum
  (`AdvanceTicks(u32)`, `WaitForState(GameState)`, `Capture(&'static str)`,
  `Exit`), `ScriptedSession(VecDeque<ScriptStep>)` resource. Fully
  unit-testable, no Bevy `App` needed for the data shape itself.
- `src/capture/runner.rs` — the `Update` system that pops/advances
  `ScriptedSession` each tick: counts down `AdvanceTicks`, compares current
  `State<GameState>` for `WaitForState`, and triggers capture/exit.
  Testable headless (`MinimalPlugins` + a fake `State<GameState>`) for the
  step-advancement logic; the `Capture`/`Exit` steps' *side effects*
  (screenshot, process exit) are not — flag those as manual-verification,
  same honesty the M0 plan used for the capsule-renders increment.
- `src/plugins/capture_plugin.rs` — composition root: disables `WinitPlugin`,
  adds `ScheduleRunnerPlugin::run_loop(...)`, sets up the off-screen render
  target, registers the runner system. Only added when capture mode is
  requested (see gating below) — never part of the normal `cargo run`.

## Known Bevy 0.19 traps (from the research, cite before you hit them)

1. **`Screenshot::image()` only captures a texture that's an actual
   camera `RenderTarget`** ([bevyengine/bevy#25215](https://github.com/bevyengine/bevy/issues/25215),
   open as of 0.19.0) — a side off-screen `Image` not bound to the game's
   `Camera` silently screenshots blank. The capture texture *must* be the
   render target the main camera is already rendering into.
2. **`EventWriter`/`.send()` is gone** — current API is
   `MessageWriter<AppExit>` / `.write(AppExit::Success)` (renamed in the
   0.16→0.17 migration). Exiting the scripted session after the last step
   uses this, not the older pattern tutorials still show.
3. **`WinitPlugin` must be explicitly disabled**, not just left unconfigured
   — it panics without a display server. Combine with
   `WindowPlugin { primary_window: None, exit_condition: ExitCondition::DontExit, .. }`
   per Bevy's own `examples/app/headless_renderer.rs`.

## Gating — never affects normal play

Capture mode is **feature-flagged** (`--features capture`) rather than an
env-var branch inside the always-compiled `main.rs`, so `cargo run`/
`cargo test` without the feature carry zero capture-mode code paths or
dependencies. The default script (the prologue playthrough: character
select → drilling → call event → decision → end, one capture per state) is
a small data literal in `src/capture/mod.rs`, swappable later if more
scripts are needed — not over-engineered into a config-file format now
(YAGNI; nothing today demands more than one script).

## Video — not in Rust

Bevy writes a numbered PNG sequence via `save_to_disk` per the official
`Screenshot` pattern; stitching into an mp4/gif is a **separate ffmpeg
shell step** (CI or local), not Rust code. Confirmed no maintained
Bevy-0.19 video-capture crate exists — this keeps the dependency count at
zero for this feature.

## Increments (TDD where the step is logic; manual-verification flagged where it's pixels/process-exit)

1. **`ScriptStep`/`ScriptedSession` data shapes**
   - Test: construct a session from a `Vec<ScriptStep>`, pop steps in order.
   - Files: `src/capture/script.rs`.
2. **`AdvanceTicks` counts down across `Update` ticks**
   - Test: headless `App` + `MinimalPlugins`, seed `AdvanceTicks(3)`, run 3
     `app.update()`s, assert the step completes and the next step starts.
   - Files: `src/capture/runner.rs`.
3. **`WaitForState` blocks until the real `GameState` matches**
   - Test: headless `App`, seed a different current state, assert the step
     does NOT advance; transition state, assert it does.
   - Files: `src/capture/runner.rs`.
4. **`Capture(name)` step triggers a screenshot request** — *manual-verify
   the actual PNG write* (needs real rendering); the *step-advancement*
   (does the runner move past a `Capture` step once issued) is still
   testable the same way as #2/#3.
   - Files: `src/capture/runner.rs`.
5. **`Exit` step ends the app** — *manual-verify* process actually exits;
   testable only that the step is reached in sequence.
   - Files: `src/capture/runner.rs`.
6. **Headless render wiring (`capture_plugin`)** — manual-verification only
   (run `cargo run --features capture` locally, inspect the PNGs). Follows
   trap #1/#3 above exactly.
   - Files: `src/plugins/capture_plugin.rs`, `src/main.rs` (feature-gated
     `add_plugins`).
7. **Default prologue script content** — manual-verification (eyeball the
   5 captured frames: CharacterSelect, Drilling, CallEvent, Decision, End).
   - Files: `src/capture/mod.rs`.

## Local usage (once built)

```bash
cargo run --features capture            # writes PNGs to target/capture/
ffmpeg -framerate 1 -i target/capture/%03d.png out.mp4   # optional clip
```

## Sequencing with the CI piece

This capability ships and is useful standalone (you can already "see
screenshots as the game is played" locally) before any CI change lands.
The CI wiring (separate PR, per the pipeline design note) depends on this
one merging first, since it needs the real `--features capture` CLI
contract to exist.
