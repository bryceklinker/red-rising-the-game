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
  symbols" link error.)
- Dev-profile speedup (Bevy's own recommendation):
  ```toml
  [profile.dev]
  opt-level = 1
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

// SYSTEM — src/movement/apply.rs — thin glue only
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
```

**Plugin isolation:** no feature plugin imports another feature plugin —
cross-feature deps go through shared pure modules (`movement::input`) or
events/states (M1+). Convention per Bevy Cheat Book, taintedcoders.com,
and the official `bevy_new_2d` template. Payoff: a test spins up
`MinimalPlugins` + only the plugin under test.

**Module layout (M0 → M4):**
```
src/
  main.rs              // assembles DefaultPlugins + feature plugins only
  player.rs             // PlayerPlugin: mesh/material, movement (M0)
  camera.rs             // CameraPlugin: light + fixed camera (M0 #6), then follow (M0 #7)
  movement/
    mod.rs               // wires input.rs + apply.rs (M0 #5)
    input.rs              // PURE: wasd_to_direction (M0 #3)
    apply.rs               // thin system: direction -> Transform (M0 #4)
    collision.rs            // thin system: clamp vs level colliders (M1)
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

No change to increment order/files — this doc is the *how* underneath it.
- #1 → §1 (setup) + §2 (bootstrap).
- #2–#5 → §3's headless pattern; §5's pure/system split is #3 (pure)
  feeding #4/#5 (systems).
- #6 → manual-verification seam; bundles mesh + light + fixed camera,
  since a lit `StandardMaterial` needs a light and a camera to be seen
  through — any one alone is unverifiable by eye.
- #7 → camera-offset math is pure per §5 (`P + O`), testable like #3;
  adds follow behavior on top of #6's already-visible fixed camera.
- M1+ → §5's module layout/plugin-isolation are the going-forward
  convention once each milestone is re-sliced.

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
