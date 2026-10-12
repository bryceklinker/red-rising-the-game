
# Plan: M0 Finale — "Hell Diver" Prologue Vertical Slice

> Supersedes/extends M0's exit criteria (`2026-09-11-milestone-breakdown.md`
> §M0). Grounded in `explore-m0-finale-scope` (2026-10-01), which
> ground-truthed the repo at `abfd3f6` and researched Bevy 0.19.1-compatible
> options. Companion "how": `2026-09-11-m0-implementation-plan.md` (pure/glue
> split, headless `App`+`MinimalPlugins` pattern, `src/plugins/*_plugin.rs`
> composition roots — all still apply here).

## Scope decision

M0 was "window, capsule, WASD, camera." It's now a short playable prologue:

**CharacterSelect → Drilling (Mars gravity, drill-rig controls) → CallEvent
(Narol warns of a gas pocket, drilling halts) → Decision (keep drilling / get
out and check / wait for the team) → End.** Only Darrow is selectable; other
characters are future milestones per the GDD.

**Mode note:** this scope is delegated to a coding sub-agent (`claude_code`),
not hand-written — an explicit, scoped exception to the project's default
"you write all game code" policy, per direct instruction. Standard discipline
still applies: strict TDD, `craft-code:code-style`, its own worktree, its own
PR (human merges, nobody else does).

## Dependency decisions (from research, cite in commit/PR if useful)

| Concern | Choice | Why |
|---|---|---|
| Physics / custom gravity | **`avian3d` 0.7.0** | `Gravity` is a plain glam-`Vec3` `Resource` — matches this codebase's existing ECS-idiomatic style; `bevy_rapier3d` needs `nalgebra` + a `Component`-based config (more ceremony, second math type) |
| Perf/memory/CPU/GPU observability | **Built-in `bevy::diagnostic`** (`FrameTimeDiagnosticsPlugin`, `EntityCountDiagnosticsPlugin`, `SystemInformationDiagnosticsPlugin`, `bevy::render::diagnostic::RenderDiagnosticsPlugin`) + a hand-rolled `bevy_ui` overlay | No current third-party overlay crate supports Bevy 0.19 (`iyes_perf_ui`, `bevy_screen_diagnostics` both stuck on 0.16/0.17) |
| Audio | **Built-in `bevy_audio`** | Zero new dependency, ECS-native components, sufficient for one looping rumble + one one-shot beep; revisit `bevy_kira_audio` only if real mixing is ever needed |
| Call/decision UI | **Plain `bevy_ui`** (`Node`/`Text`/`Button`/`Interaction`), no dialogue crate | `bevy_yarnspinner` is reserved for M2's real branching dialogue data — pulling it forward here is the exact over-build the roadmap avoids |
| Scene flow | **Bevy `States`** (`#[derive(States)]`, `OnEnter`/`OnExit`, `run_if(in_state(..))`) | Stable, unchanged into 0.19, exactly fits a 5-state linear flow |

## Known risk to verify/fix FIRST

`player.rs::spawn_player` (landed in `0b26d00`) takes `ResMut<Assets<Mesh>>`
+ `ResMut<Assets<StandardMaterial>>` directly — the implementation plan's
§6 explicitly warned against this because `tests/player_movement_tests.rs`
builds its app with `MinimalPlugins`, which never registers `AssetPlugin`.
**First step: run `cargo test`. If `player_movement_tests` panics**, split
the mesh/material spawn into its own Startup system (visual-only, no
`Player`/`Transform` params) so the movement tests stop depending on asset
resources, per the plan's original prescription. Fix this before building
anything else on top of `player.rs`.

## Bevy 0.19 traps (don't trust older tutorials/training data)

- `PbrBundle` is gone — use `Mesh3d`/`MeshMaterial3d` (already correct in
  this repo).
- `SystemInformationDiagnosticsPlugin` **does not work under the
  `dynamic_linking` Bevy feature**. `Cargo.toml` currently bakes
  `dynamic_linking` into the default feature set — switch to the
  per-invocation form (`cargo run --features bevy/dynamic_linking`) so plain
  `cargo build`/`cargo test` (and this diagnostics plugin) work correctly.
- `RenderDiagnosticsPlugin` must be registered **after** `DefaultPlugins`
  (needs `RenderDevice` to exist).
- Bevy 0.19's default audio decoder is `vorbis` (OGG) — pick `.ogg` for the
  rumble/beep assets, or add the `mp3` feature explicitly.
- `bevy_rapier3d`'s `RapierConfiguration` is a `Component`, not a `Resource`
  (as of 0.28+) — not relevant once avian3d is chosen, but a trap if anyone
  free-hands a Rapier example later.

## Game states

```rust
#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
enum GameState {
    #[default]
    CharacterSelect,
    Drilling,
    CallEvent,
    Decision,
    End,
}
```

## Increments (ordered; each independently TDD-able per `craft-code:planning`
— the implementer should further slice each into single-test steps the same
way M0 increments #1–7 were)

1. **[do first] Fix/confirm the `spawn_player` asset-resource risk above.**
2. **Close M0's original #7 — camera follows player.** Pure
   `camera_position_for(player_pos, offset) -> Vec3`; `MainCamera` marker;
   `Update` system replacing the static camera Transform. (`src/camera.rs`)
3. **`GameState` enum + transitions.** Headless `App` test: default state,
   trigger `NextState`, `app.update()`, assert new state.
   (`src/game_state.rs`)
4. **CharacterSelect → Drilling.** `OnEnter`: spawn one "Darrow" `Button`;
   `Interaction::Pressed` → `NextState(Drilling)`; `OnExit`: despawn the UI.
   (`src/plugins/character_select_plugin.rs`)
5. **Cargo.toml: `dynamic_linking` → per-invocation; add `avian3d`,
   `bevy_audio` feature choice.** Config-only; verify gates still green
   before/after.
6. **Mars gravity + drill-rig physics + basic drill/world visuals.**
   `avian3d::PhysicsPlugins`, `insert_resource(Gravity(Vec3::NEG_Y * 3.71))`;
   drill-rig entity (`RigidBody`, simple primitive mesh e.g. `Cylinder` +
   `StandardMaterial`) spawned `OnEnter(Drilling)`; a simple ground/tunnel
   world mesh spawned alongside it. Pure `drill_input_to_velocity(&ButtonInput<KeyCode>) -> Vec3`
   (same shape as the existing `get_direction_from_keys`); thin `Update`
   system writes `LinearVelocity`, gated `run_if(in_state(Drilling))`.
   Decide explicitly whether this repurposes the existing `Player`/`move_player`
   entity or introduces a distinct drill-rig entity — document the choice in
   the PR description either way. (`src/drilling/input.rs`, `src/drilling/rig.rs`,
   `src/plugins/drilling_plugin.rs`)
7. **Depth-trigger → CallEvent.** Pure `call_trigger_state(depth, threshold) -> bool`;
   `Update` system (Drilling-scoped) checks drill depth/progress, transitions
   to `CallEvent` when true. `OnEnter(CallEvent)`: spawn an incoming-call
   banner UI; Drilling's input systems already stop running once the state
   changes (no separate pause flag needed). (`src/drilling/depth.rs`,
   `src/plugins/call_event_plugin.rs`)
8. **CallEvent → Decision → End.** An acknowledge action on the banner moves
   to `Decision`; `OnEnter(Decision)` spawns 3 buttons (keep drilling / get
   out and check / wait for the team). Pure
   `end_state_for_choice(DecisionOption) -> EndState`; click handler calls it,
   stores the result in a `Resource`, transitions to `End`. `OnEnter(End)`
   spawns UI reflecting the stored `EndState` — demo stops here, no further
   transitions. (`src/decision/outcome.rs`, `src/plugins/decision_plugin.rs`,
   `src/plugins/end_plugin.rs`)
9. **Diagnostics overlay (memory/CPU/GPU/frame-time).** Register
   `FrameTimeDiagnosticsPlugin` + `EntityCountDiagnosticsPlugin` +
   `SystemInformationDiagnosticsPlugin` + `RenderDiagnosticsPlugin` (after
   `DefaultPlugins`). Pure `format_diagnostic_line(label, Option<f64>) -> String`;
   thin `Update` system renders formatted lines into `bevy_ui` `Text` nodes,
   toggled by a `Resource` flipped on a keybind (e.g. F3). Test what's
   feasible headless (pure formatter fully; `FrameTime`/`EntityCount` under
   `MinimalPlugins` + `DiagnosticsPlugin`); flag `SystemInformation`/`Render`
   diagnostics as a manual-verification seam like M0's rendering, since they
   need a real window/process. (`src/diagnostics/format.rs`,
   `src/plugins/diagnostics_plugin.rs`)
10. **Audio: drill rumble (loop) + call beep (one-shot).** `bevy_audio`
    components spawned `OnEnter(Drilling)` / `OnEnter(CallEvent)`
    respectively. Headless-testable at "the expected `AudioPlayer`/
    `PlaybackSettings` exists on the right entity for the right state" —
    split any asset-loading glue out of the tested system the same way the
    `spawn_player` fix in step 1 does. (`src/plugins/audio_plugin.rs`)

## Exit criteria

`cargo run` opens a window: pick Darrow at a character-select screen, drive
a drill rig under Mars gravity, trigger Narol's call at a depth threshold
(drilling visibly halts), choose one of 3 options, see an end screen
reflecting the choice. A toggleable overlay shows frame time/entity
count/CPU/memory (GPU where the backend supports it). Drill rumble loops
during Drilling; a beep plays on the call. Every pure function and every
state transition has a green headless test; rendering/audio glue is a
documented manual-verification seam, same convention as M0's original #6/#7.
