# M0 Implementation Plan — Integration, Commands, DI, UI/Logic Boundaries

> The "how" underneath `2026-09-11-milestone-breakdown.md`'s 7 increments.
> Grounded on **Bevy 0.19.1** (2026-09-11). Bevy is pre-1.0 with real API
> churn — re-verify version-sensitive bits (`Cargo.toml` pin, `Single<D>`
> vs `Query::single()`, feature flags) before starting M1.

## 1. Integration setup

- `Cargo.toml`: `bevy = "0.19"`, `edition = "2024"` (matches Bevy's own
  manifest).
- `dynamic_linking`: prefer per-invocation
  `cargo run --features bevy/dynamic_linking` over baking it into
  `Cargo.toml`, so `--release` can never accidentally ship it. (Windows:
  without the profile split below this can throw a "too many exported
  symbols" link error.) **As-built:** the repo instead bakes it into
  `Cargo.toml` (`bevy = { features = ["dynamic_linking"] }`) for dev
  convenience — a deliberate deviation from this preference. Revisit
  before any `--release` build so it never ships.
- Dev-profile speedup (Bevy's own recommendation) plus the macOS
  direct-launch rpath fix (needed once RustRover or another test runner
  invokes a built binary directly instead of through `cargo test` — only
  `cargo test` itself injects `DYLD_LIBRARY_PATH` for you; see
  bevyengine/bevy#3856):
  ```toml
  [profile.dev]
  opt-level = 1
  rpath = true
  [profile.dev.package."*"]
  opt-level = 3
  ```
- Asset hot-reload (`file_watcher` / the `dev` meta-feature) — not needed
  until M2's dialogue data; same dev-only-never-ship caveat as
  `dynamic_linking`.
- Single crate, no workspace — no independently-versioned crates or
  multi-binary need here.
- Core vocabulary: `Component`; `System` (plain fn, params injected);
  `Commands` (deferred spawn/despawn); `Query<&T>` /
  `Query<&mut T, With<F>>`; `Res`/`ResMut`; `Startup` (once) vs `Update`
  (every frame) schedules.
- **Version trap:** `Single<D, F>` (auto-skips system if cardinality ≠ 1)
  vs `Query::single()`/`.single_mut()` (returns a `Result`, handled
  explicitly — used throughout §3). Both valid; pick one and stay
  consistent.

## 2. Command cheat sheet

Bootstrap (increment #1):
```bash
cargo new red-rising && cd red-rising
cargo add bevy -F dynamic_linking   # or the per-invocation flag, see §1
cargo build && cargo run            # should open an empty window
```

Daily loop:
```bash
cargo test                                    # red → green → refactor
cargo run --features bevy/dynamic_linking     # manual checks (M0 #6, #7)
```

Commit gates (`.craft-code.yml`):
```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```
(`--all-features` pulls in `dynamic_linking`/`file_watcher` for clippy —
fine for linting, keep both out of release builds.)

## 3. Headless-testing pattern (use for every M0 increment)

Swap `DefaultPlugins` → `MinimalPlugins` (no window/renderer/input
backend), manually insert whatever resources those backends would
otherwise provide (e.g. `ButtonInput<KeyCode>`). Sourced from Bevy's own
`tests/how_to_test_apps.rs` — build increments #2–#5 as separate failing
tests, not one big test like this illustration:

```rust
use bevy::prelude::*;

const SPEED: f32 = 1.0;

#[derive(Component)]
pub struct Player;

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = query.single_mut() else { return; };
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) { direction.y += 1.0; }
    if keyboard_input.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
    if keyboard_input.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
    if keyboard_input.pressed(KeyCode::KeyD) { direction.x += 1.0; }
    transform.translation += direction.normalize_or_zero() * SPEED;
}

pub fn movement_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_player);
    app.add_systems(Update, move_player);
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((Player, Transform::default()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(ButtonInput::<KeyCode>::default());
        app
    }

    #[test]
    fn moves_right_when_d_is_pressed() {
        let mut app = create_test_app();
        app.add_plugins(movement_plugin);
        app.update(); // run Startup so the player exists

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyD);
        app.update();

        let transform = app.world_mut().query::<&Transform>().single(app.world()).unwrap();
        assert!(transform.translation.x > 0.0);
        assert_eq!(transform.translation.y, 0.0);
    }
}
```

- No `Time`/`delta_secs()` here on purpose: under `MinimalPlugins` the
  first tick's delta isn't deterministic (depends on wall-clock time since
  app creation). Increment #4 fixes this by advancing `Time` by a known
  delta.
- Plain `fn(&mut App)` plugins (as above) are valid for small, stateless
  plugins — matches Bevy's own test file. Reach for the full `Plugin`
  trait only once a module needs its own state/config.
- **As-built convention (diverges from this illustration):** tests live in
  the external `tests/` integration crate, not an inline `#[cfg(test)]
  mod tests` block — e.g. `tests/player_movement_tests.rs` for
  headless-`App` tests, `tests/movement/input_tests.rs` for pure-function
  tests (using `rstest` for parameterized cases). Mirror `src/`'s module
  shape under `tests/` (`tests/movement/mod.rs` → `mod input_tests;`)
  instead of nesting `mod tests` inside the source file.

## 4. Dependency injection — the realistic options

- **(a) Default — ECS-as-DI.** `Res`/`ResMut`/`Query`/`Commands`/`Local`/
  custom `SystemParam` in a system's signature; Bevy injects from the
  `World` per schedule run. Zero-ceremony, every official example uses it
  — don't hide it behind an abstraction.
- **(b) `Plugin`/`add_plugins` = composition root.** One small plugin per
  feature module (§5). This is what lets a test swap in `MinimalPlugins`
  at the root instead of branching inside systems.
- **(c) `Box<dyn Trait>` in a `Resource` wrapper — external boundaries
  only.** `Resource` itself isn't dyn-compatible, so wrap it:
  ```rust
  trait SaveBackend: Send + Sync {
      fn save(&self, data: &SaveData) -> Result<(), SaveError>;
  }
  #[derive(Resource)]
  struct ActiveSaveBackend(Box<dyn SaveBackend>);
  ```
  Don't use this for M0's fake keyboard input — insert a real
  `ButtonInput<KeyCode>` and call `.press()` (§3's own idiom). Save this
  pattern for M4's save/load boundary or future networking.
- **(d) Skip general-purpose DI containers** (shaku, teloc) — absent from
  the Bevy ecosystem; would add a second resolution mechanism the
  scheduler can't parallelize or hot-reload around.

Default to (a) everywhere, (b) to structure feature plugins, (c) only at a
genuine external boundary (not M0).

## 5. UI/logic separation

**Rule:** signature has `Query`/`Res`/`ResMut`/`Commands`/
`EventReader`/`EventWriter`/`&World` → it's a *system* (Bevy-coupled glue,
not unit-tested directly). Plain-data-in/plain-data-out → *pure* (unit-
tested, no `App`). Increment #3 is pure and feeds #4/#5's thin systems.

```rust
// PURE — src/movement/input.rs
pub fn wasd_to_direction(pressed: &[KeyCode]) -> Vec2 {
    let mut dir = Vec2::ZERO;
    if pressed.contains(&KeyCode::KeyW) { dir.y += 1.0; }
    if pressed.contains(&KeyCode::KeyS) { dir.y -= 1.0; }
    if pressed.contains(&KeyCode::KeyA) { dir.x -= 1.0; }
    if pressed.contains(&KeyCode::KeyD) { dir.x += 1.0; }
    dir.normalize_or_zero()
}

// SYSTEM — src/player.rs — thin glue only
fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let pressed: Vec<KeyCode> = keys.get_pressed().copied().collect();
    let direction = wasd_to_direction(&pressed);
    for mut transform in &mut query {
        transform.translation += (direction * SPEED * time.delta_secs()).extend(0.0);
    }
}

// COMPOSITION ROOT — src/plugins/player_movement_plugin.rs — Startup/Update wiring only
pub fn player_movement_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_player);
    app.add_systems(Update, move_player);
}
```

**As-built vs. this illustration:** the real function is named
`get_direction_from_keys` (returns `Vec3`, not `Vec2` — harmless, `.z` is
unused) and doesn't yet multiply by `time.delta_secs()` (still
framerate-dependent — see §6 #4 for what's needed to close that gap).
More importantly, the *wiring* doesn't live inside `movement/mod.rs` — it
lives in its own composition-root file under `src/plugins/`, one file per
feature plugin, re-exported from `src/plugins/mod.rs`. That's the
convention increments #6/#7 (a new `CameraPlugin`) should follow too.

**Plugin isolation:** no feature plugin imports another feature plugin —
cross-feature deps go through shared pure modules (`movement::input`) or
events/states (M1+). Convention per Bevy Cheat Book, taintedcoders.com,
and the official `bevy_new_2d` template. Payoff: a test spins up
`MinimalPlugins` + only the plugin under test.

**Module layout (M0 → M4):**
```
src/
  main.rs              // assembles DefaultPlugins + feature plugins only, via `use red_rising::plugins::...`
  lib.rs               // crate root: pub mod player; pub mod movement; pub mod plugins; ... (main.rs consumes this, never redeclares its own `mod` tree)
  player.rs             // Player marker + spawn_player/move_player systems (M0), + mesh/material spawn (M0 #6)
  camera.rs             // spawn_scene: light + fixed Camera3d (M0 #6); follow_camera system (M0 #7)
  movement/
    mod.rs               // pub mod input;
    input.rs              // PURE: get_direction_from_keys (M0 #3)
    collision.rs            // thin system: clamp vs level colliders (M1)
  plugins/
    mod.rs               // pub mod player_movement_plugin; pub mod camera_plugin; ...
    player_movement_plugin.rs  // composition root: Startup->spawn_player, Update->move_player (M0 #5)
    camera_plugin.rs           // composition root: Startup->spawn_scene, Update->follow_camera (M0 #6/#7)
  level.rs             // level geometry loading (M1)
  enemy.rs             // enemy spawn/AI state (M1)
  combat/razor.rs      // attack + combo systems (M1)
  cover/
    mod.rs               // CoverPlugin (M2)
    meter.rs              // PURE: apply_cover_delta(current, delta) -> f32 (M2)
  dialogue/
    mod.rs               // DialoguePlugin: load + runtime systems (M2)
    model.rs              // PURE, serde-only graph/node/choice, zero bevy_ui (M2)
  ui/
    mod.rs               // UiPlugin: bevy_ui node spawning only
    dialogue_box.rs        // renders dialogue::model; imports it, never reverse
  squad/
    mod.rs               // SquadPlugin (M3)
    command.rs            // PURE: resolve_squad_order(...) (M3)
  save/
    mod.rs               // SavePlugin: serde (de)serialization (M4)
    schema.rs             // PURE save/load data model (M4)
```
Pure-enough bar: `input.rs`/`meter.rs`/`model.rs`/`command.rs`/
`schema.rs`-style files compile and `cargo test` with **no Bevy `App`**.

**M2 dialogue/Cover split:** `dialogue/model.rs` is plain
`#[derive(Deserialize, Serialize)]`, zero `bevy_ui`, loaded via `serde`
from RON/JSON. A runtime layer walks the graph and emits events;
`ui/dialogue_box.rs` reacts to those events and imports `dialogue::model`
(never the reverse) — mirrors the real `bevy_talks` crate's own split.
Same pattern for `cover/meter.rs` (plain `f32` math vs. whatever widget
renders it).

## 6. Mapping onto the increment list

Cross-reference only — no change to `milestone-breakdown.md`'s increment
order or files. For each numbered M0 increment: which section to open,
why, **as-built notes** where the real code has drifted from the
illustration (#1–#5 are already built), and full concrete detail for #6/#7
(not yet built, so no worked example existed for them before this pass).

- **#1 (project scaffolds)** → §1 for the `Cargo.toml` shape
  (`bevy = "0.19"`, `edition = "2024"`, `dynamic_linking`, `rpath = true`,
  the dev-profile speedup block) + §2 for the literal bootstrap commands.
  **No new Cargo features or crates are needed for #6 or #7 either** —
  default `bevy` already includes `bevy_pbr` (mesh/material/light types)
  and `bevy_render`'s camera types; nothing to add to `Cargo.toml`.

- **#2 (player entity spawns)** → §3's `create_test_app()` helper is the
  harness shape; **as-built**, the real test app is built in
  `tests/player_movement_tests.rs::setup_testing_app`, not inline in the
  source file — same idea, different location (see #3 below).

- **#3 (WASD → direction, pure)** → this increment *is* §5's PURE half.
  **As-built:** the real function is `get_direction_from_keys` in
  `src/movement/input.rs`, tested in `tests/movement/input_tests.rs` with
  plain `#[test]`/`#[rstest]` cases (no `#[cfg(test)]` block, no `App`) —
  follow that file's shape, not §3's inline illustration.

- **#4 (direction → Transform)** → §5's SYSTEM half. **As-built:** this
  system lives directly in `src/player.rs::move_player` — no separate
  `movement/apply.rs` was created (an accepted simplification while
  there's only one system; split it out if a second movement-adjacent
  system shows up). **Still open:** `move_player` doesn't yet take
  `Res<Time>` or multiply by `delta_secs()` — it applies `SPEED` flat per
  tick, which is framerate-dependent. To close #4 per its own acceptance
  bar ("moves `translation` by `direction * speed * delta_time`"): add
  `time: Res<Time>` to `move_player`'s params, change the update line to
  `transform.translation += direction * SPEED * time.delta_secs()`, and in
  the test advance a known delta before asserting —
  `app.world_mut().resource_mut::<Time>().advance_by(Duration::from_secs_f32(1.0))`
  — since §3 already flagged the first tick's real elapsed time isn't
  deterministic under `MinimalPlugins`.

- **#5 (WASD moves the player end-to-end)** → wires #3 into #4.
  **As-built:** the wiring (`add_systems(Startup, spawn_player)` /
  `add_systems(Update, move_player)`) lives in
  `src/plugins/player_movement_plugin.rs`, a composition-root file under
  `src/plugins/` — not `src/movement/mod.rs` (which only declares
  `pub mod input;`). `main.rs` consumes it via
  `use red_rising::plugins::player_movement_plugin::player_movement_plugin;`.
  **Critical pitfall:** `main.rs` must *only* `use` from the `red_rising`
  lib crate and never redeclare `mod player;`/`mod movement;` itself —
  doing so creates two distinct copies of `Player` (one per crate root),
  and the binary silently stops running the systems your tests exercise.
  `src/lib.rs` is the sole owner of `pub mod player; pub mod movement;
  pub mod plugins;`.

- **#6 (mesh + light + fixed camera)** → still a manual-verification seam
  per `craft-code:verification` — no headless assertion for "does a
  capsule render." Concrete code this increment needs (previously just a
  one-line pointer):
  - **Player mesh/material — add to `spawn_player` in `src/player.rs`:**
    ```rust
    fn spawn_player(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn((
            Player,
            Transform::default(),
            Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        ));
    }
    ```
    **Version trap not flagged elsewhere in this doc:** Bevy 0.19 spawns
    meshes as the `Mesh3d`/`MeshMaterial3d` newtype components shown
    above — *not* the older `PbrBundle { mesh: .., material: .., .. }`
    shape a lot of tutorials/AI output still show. If `cargo build`
    complains `PbrBundle` doesn't exist, that's the mismatch.
  - **Light + fixed camera — new `src/camera.rs`:**
    ```rust
    fn spawn_scene(mut commands: Commands) {
        commands.spawn((
            PointLight { intensity: 2_000_000.0, ..default() },
            Transform::from_xyz(4.0, 8.0, 4.0),
        ));
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 3.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    }
    ```
  - **The biggest real gap: adding `ResMut<Assets<Mesh>>`/
    `ResMut<Assets<StandardMaterial>>` params to the *existing*
    `spawn_player` will break every headless test that already passes.**
    `tests/player_movement_tests.rs` builds its app with `MinimalPlugins`,
    which never inserts an `AssetPlugin` or `Assets<T>` resources — Bevy
    panics with "Resource requested ... does not exist" the instant a
    system asking for them runs. **Do not add those params to the
    existing `spawn_player`.** Put the visual spawn in its own Startup
    system instead (e.g. `spawn_player_mesh` in `player.rs`, or fold the
    player's mesh spawn into `camera.rs`'s `spawn_scene`) so the movement
    tests' `App` — which never registers a render/asset plugin — never
    touches it.
  - **Wiring:** new composition root `src/plugins/camera_plugin.rs`
    (`Startup -> spawn_scene`), registered in `main.rs` alongside
    `player_movement_plugin` — per §5's plugin-isolation rule this is a
    sibling plugin, not folded into `player_movement_plugin`.
  - Deliberately out of scope for M0, not a gap: window title
    (`WindowPlugin` override) stays default; no clippy/fmt concerns beyond
    the usual gate commands in §2.

- **#7 (camera follows the player)** → offset math is pure per §5's rule.
  Concrete shape (previously only gestured at):
  ```rust
  // PURE — src/camera.rs
  pub fn camera_position_for(player_position: Vec3, offset: Vec3) -> Vec3 {
      player_position + offset
  }
  ```
  Test cases to write (headless, no `App`): player at `Vec3::ZERO` with a
  non-zero offset; player moved diagonally (non-axis-aligned position);
  calling it twice with the same inputs returns the same result (no
  hidden state).
  - **Marker + query shape:** add `#[derive(Component)] pub struct
    MainCamera;` alongside #6's camera spawn so the follow system can
    target it unambiguously —
    `Query<&mut Transform, With<MainCamera>>` for the camera side,
    `Query<&Transform, (With<Player>, Without<MainCamera>)>` for the
    player side (Bevy requires disjoint queries when both target
    `Transform`).
  - **Schedule:** #6's `spawn_scene` Startup system stays as-is (it just
    gives the camera its initial `Transform`); add a new `Update` system
    — `follow_camera` — to `camera_plugin.rs` that reads the player's
    `Transform`, calls `camera_position_for`, and writes the result into
    the camera's `Transform` every frame. Nothing from #6 gets deleted.

- **M1 onward** → re-run §5's pure/system split and its plugin-isolation
  rule (no feature plugin imports another feature plugin directly) for
  every new module; §5's module-layout tree already sketches the target
  file locations for M1–M4 (`level.rs`, `enemy.rs`, `combat/razor.rs`,
  `cover/`, `dialogue/`, `ui/`, `squad/`, `save/`) so you're not inventing
  the shape from scratch when you get there.

## Sources

- bevy.org/learn/quick-start/{setup,apps,ecs}
- `bevyengine/bevy` v0.19.1: `Cargo.toml`, `tests/how_to_test_apps.rs`,
  `examples/window/window_settings.rs`, `examples/showcase/breakout.rs`,
  `examples/input/keyboard_input.rs`
- docs.rs/bevy — `SystemParam`, `Resource` dyn-compatibility
- bevy-cheatbook.github.io/programming/plugins.html
- taintedcoders.com/bevy/{patterns/plugin-organization,code-organization,how-to/headless-mode}
- `TheBevyFlock/bevy_new_2d` (official/maintained template)
- `bevy_talks` crate + docs (giusdp.github.io/bevy_talks)
- crates.io — shaku, teloc (checked; unused in the Bevy ecosystem)
- johanhelsing.studio/posts/extreme-bevy-2
