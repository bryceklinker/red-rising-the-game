# M0 Implementation Plan — Integration, Commands, DI, and UI/Logic Boundaries

> Companion to `docs/craft-code/plans/2026-09-11-milestone-breakdown.md` (the
> "what/when," sliced into 7 TDD increments) and `docs/GDD.md` §9. This doc
> is the "how" layer underneath it: concrete Bevy integration steps, a
> command cheat sheet, the realistic dependency-injection options in a
> Bevy/Rust project, and the architectural decisions that keep pure logic
> separable from Bevy-coupled glue from increment #1 onward.
>
> Grounded against **Bevy 0.19.1** (latest stable at time of writing,
> 2026-09-11; next unreleased is `0.20.0-dev` on `main` — don't copy APIs
> only visible there). Bevy is pre-1.0 with real API churn between minor
> versions (see `docs/GDD.md` §11) — re-verify version-sensitive details
> below (exact `Cargo.toml` pin, `Single<D>` vs `Query::single()`, feature
> flag names) against whatever `bevy = "0.NN"` actually resolves to when you
> run `cargo add`, and again before starting M1.

## 1. Integration setup

**Cargo.toml shape** (official quick-start form):
```toml
[dependencies]
bevy = "0.19"
```
Target `edition = "2024"` in your `Cargo.toml` to match Bevy's own manifest
(`2021` still compiles, but there's no reason to start behind).

**`dynamic_linking`** — cuts iterative compile times, but must never ship.
Two valid forms:
- Per-invocation (recommended default for this project): `cargo run --features bevy/dynamic_linking`
- Permanent, via `cargo add bevy -F dynamic_linking` (adds it to `Cargo.toml`
  directly)

Prefer the **per-invocation flag**, not the permanent `Cargo.toml` feature —
that way a plain `cargo build --release` can never accidentally ship the
dev-only dylib. On Windows, dynamic linking without the profile split below
can throw a "too many exported symbols" link error.

**Dev-profile speedup** (official recommendation — add to `Cargo.toml`):
```toml
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```
Your own crate stays fast to compile; Bevy and other dependencies run at
full optimization so the engine isn't sluggish during iteration — this
matters a lot under TDD's constant `cargo test`/`cargo run` cycle.

**Asset hot-reload** (not needed for M0, worth knowing for M2's dialogue
data): the `file_watcher` feature, or Bevy's bundling `dev` meta-feature
(`file_watcher` + `bevy_dev_tools`). Same "must not ship" caveat as
`dynamic_linking` — keep it dev-only.

**Workspace vs. single crate:** single crate (plain `cargo new`, no
workspace). A workspace earns its complexity when you need multiple
independently-versioned crates or shared code across binaries — neither
applies to a solo learning project, and Bevy's own `examples/` directory is
just flat files in one crate.

**Core vocabulary refresher (current, 0.19.1)** — you'll use all of these in
M0 increment #2 onward:
- **Component**: `#[derive(Component)] struct Player;`
- **System**: a plain Rust function whose parameters Bevy injects — see §4.
- **Commands**: `commands.spawn((Player, Transform::default()))` — deferred
  spawn/despawn, applied at the next sync point.
- **Query**: `Query<&Transform>` (read), `Query<&mut Transform, With<Player>>`
  (write + filter).
- **Res / ResMut**: read-only / mutable access to a singleton `Resource`.
- **Startup vs Update schedules**: `app.add_systems(Startup, spawn_player)`
  runs once at launch; `app.add_systems(Update, move_player)` runs every
  frame.

**One version-dependent trap to know before you hit it:** newer Bevy example
code favors `Single<&mut Transform, With<Player>>` as a system parameter
over `Query<&mut Transform, With<Player>>` + manual `.single_mut()`.
`Single<D, F>` auto-skips the system if cardinality isn't exactly 1;
`Query::single()`/`.single_mut()` return a `Result` you handle explicitly
(this is what the official headless-test pattern in §3 uses). Both are
correct — pick based on whether you want Bevy to silently skip or want to
handle the "wrong cardinality" case yourself. Increment #2's test (exactly
one `Player` exists) is a natural place to notice which one you prefer.

## 2. Command cheat sheet

**Bootstrap (increment #1):**
```bash
cargo new red-rising
cd red-rising
cargo add bevy -F dynamic_linking   # or add the flag per-invocation instead, see §1
cargo build
cargo run   # should open an empty window
```

**Daily loop (increment #2 onward):**
```bash
cargo test                                          # red -> green -> refactor
cargo run --features bevy/dynamic_linking            # manual/visual checks (M0 #6, #7)
```

**Gates before every commit** (per `.craft-code.yml`):
```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```
Note: `--all-features` pulls in `dynamic_linking`/`file_watcher` for
clippy's sake — fine for linting, just don't let that combination leak into
a release build.

## 3. The headless-testing pattern (use this for every M0 increment)

This is Bevy's own documented answer to "how do I test an App" — sourced
from `tests/how_to_test_apps.rs` in the Bevy repo. Swap `DefaultPlugins` for
`MinimalPlugins` to run with no window/renderer/input backend, then manually
insert whatever resources those backends would otherwise provide (e.g.
`ButtonInput<KeyCode>`).

Full worked example (WASD movement — this is effectively M0 increments
#2–#5 fused into one illustration; build them as separate failing tests per
the milestone breakdown, don't write this as one big test):

```rust
use bevy::prelude::*;

const SPEED: f32 = 1.0;

#[derive(Component)]
pub struct Player;

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };

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

Deliberate simplification worth knowing: this example doesn't use
`Time`/`delta_secs()` in the movement math. Under `MinimalPlugins`,
`TimePlugin` is present, but the delta on the very first `app.update()`
depends on real wall-clock time since app creation — not fully deterministic
for a first test. M0 increment #4 in the milestone breakdown already calls
this out ("insert a `Time` resource advanced by a known delta") — do that
once you get there; a fixed step is fine for the earliest increments.

Registering a plugin as a plain `fn(&mut App)` (as above) rather than a
`struct` implementing `Plugin` is valid and is what Bevy's own test file
does for small, stateless plugins — use whichever reads better per-module;
don't feel obligated to reach for the full `Plugin` trait until a module
needs its own state or configuration.

## 4. Dependency injection in Bevy — the realistic options

**(a) ECS-as-DI via `Resource`/`SystemParam`/`Query` in system signatures —
your primary mechanism, use this by default.** A system is a plain Rust
function; each parameter type (`Res<T>`, `ResMut<T>`, `Query<D, F>`,
`Commands`, `Local<T>`, or a custom `#[derive(SystemParam)]` struct)
implements `SystemParam`. Bevy inspects a system's parameter types when it's
registered and constructs/injects exactly what the function asked for from
the `World` on every schedule run — the same shape as constructor-injection
DI, except the "container" is the ECS scheduler and injection happens
per-call. This is zero-ceremony, it's what every official example uses, and
it's the thing you need fluency in to be productive in Bevy at all — don't
hide it behind an abstraction layer.

**(b) `Plugin`/`App::add_plugins` as the composition root.** This is where
you decide which concrete systems/resources get wired in — and it's exactly
what makes §3's `MinimalPlugins` swap possible: the test picks a different
plugin *group* at the composition root, not different code inside your
systems. Use one small plugin per feature module (§5).

**(c) `Box<dyn Trait>` wrapped in a concrete `Resource` newtype — reach for
this only at genuine external boundaries, not for engine-provided input.**
One precise correction worth knowing up front: `Resource` itself is **not
dyn-compatible** (its own docs say so explicitly), so you can't insert
`Box<dyn Resource>`. The real pattern is a concrete wrapper struct that
derives `Resource` and holds the trait object inside:
```rust
trait SaveBackend: Send + Sync {
    fn save(&self, data: &SaveData) -> Result<(), SaveError>;
}

#[derive(Resource)]
struct ActiveSaveBackend(Box<dyn SaveBackend>);
```
Production inserts a real filesystem-backed implementation; a test inserts
a deterministic in-memory fake. For the *specific* "fake keyboard input"
case you'll hit in M0, don't reach for this — insert a real
`ButtonInput<KeyCode>` and call `.press()`/`.release()` on it directly (as
§3 does), which is Bevy's own idiom and needs no trait-object indirection.
Save this pattern for M4's save/load boundary or any future networking
client, where the thing being swapped is genuinely your own abstraction
over something non-Bevy.

**(d) Skip general-purpose Rust DI containers (shaku, teloc) entirely.**
Neither appears in the Bevy ecosystem, and both solve a problem — hand-wired
long-lived object graphs — that Bevy's scheduler already solves natively at
a finer grain (per-system, per-frame, straight from the `World`). Layering
one on top would add a second, parallel resolution mechanism the scheduler
doesn't know about, can't parallelize around, and can't hot-reload — added
complexity with no compensating benefit here.

**Bottom line for this project:** default to (a) everywhere, use (b) to
structure the app into feature plugins (§5), and reserve (c) for the day a
real external boundary (save/load, networking) shows up — which isn't M0.

## 5. UI/logic separation — decisions to lock in now

**The one rule to hold yourself to from increment #1:** any function whose
signature contains `Query`, `Res`, `ResMut`, `Commands`, `EventReader`/
`EventWriter`, or `&World` is a *system* — Bevy-coupled glue, kept small,
not unit-tested by inspecting its internals. Any function that only
takes/returns plain data (`Vec2`, `f32`, your own structs/enums) is *pure* —
that's where strict-TDD's fast, no-`App` unit tests live. The milestone
breakdown already applies this instinctively: increment #3
(`src/movement/input.rs`, pure WASD→direction mapping, tested with no `App`
at all) feeds increment #4/#5 (`src/movement/apply.rs`, `src/movement/mod.rs`,
the thin systems that touch `Transform`). Keep that shape as you go.

```rust
// PURE — src/movement/input.rs — zero Bevy-ECS types beyond Vec2, unit-testable directly
pub fn wasd_to_direction(pressed: &[KeyCode]) -> Vec2 {
    let mut dir = Vec2::ZERO;
    if pressed.contains(&KeyCode::KeyW) { dir.y += 1.0; }
    if pressed.contains(&KeyCode::KeyS) { dir.y -= 1.0; }
    if pressed.contains(&KeyCode::KeyA) { dir.x -= 1.0; }
    if pressed.contains(&KeyCode::KeyD) { dir.x += 1.0; }
    dir.normalize_or_zero()
}

// SYSTEM — src/movement/apply.rs — thin glue, calls the pure fn, not unit-tested by itself
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

**Plugin isolation rule, worth adopting from M0 onward:** no feature plugin
imports from another feature plugin — every cross-feature dependency should
go through either (a) shared, plugin-independent pure modules (like
`movement::input`), or (b) events/states, once those exist from M1 onward.
This is a widely-restated Bevy community convention (unofficial Bevy Cheat
Book; taintedcoders.com's code-organization guide), and it's also what an
official current Bevy project template (`TheBevyFlock/bevy_new_2d`, which
Bevy's own site links to) structures around: a thin `lib.rs`/`main.rs` that
only assembles feature plugins, with each feature living in its own folder.
The concrete payoff for you: it's what lets a test spin up only
`MinimalPlugins` + the one plugin under test, without dragging in
unrelated systems.

**Module layout — keep M0's already-planned files (they already follow this
shape), and use this as the going-forward pattern for M1–M4:**

```
src/
  main.rs                 // assembles DefaultPlugins + feature plugins, nothing else
  player.rs                // PlayerPlugin: spawns capsule, registers movement (M0)
  camera.rs                // CameraPlugin: follow-cam (M0)
  movement/
    mod.rs                  // wires input.rs + apply.rs into the app (M0 #5)
    input.rs                 // PURE: wasd_to_direction (M0 #3)
    apply.rs                  // thin system: direction -> Transform (M0 #4)
    collision.rs              // thin system: clamp against level colliders (M1)
  level.rs                 // level-geometry loading (M1)
  enemy.rs                 // enemy spawn/AI state (M1)
  combat/
    razor.rs                 // attack + combo systems (M1)
  cover/
    mod.rs                   // CoverPlugin (M2)
    meter.rs                  // PURE: apply_cover_delta(current, delta) -> f32, clamped (M2)
  dialogue/
    mod.rs                   // DialoguePlugin: loading + runtime systems (M2)
    model.rs                  // PURE, serde-only: DialogueGraph/Node/Choice, zero bevy_ui import (M2)
  ui/
    mod.rs                   // UiPlugin: bevy_ui node spawning only (M2+)
    dialogue_box.rs            // renders dialogue::model data; imports dialogue::model, never the reverse
  squad/
    mod.rs                   // SquadPlugin (M3)
    command.rs                 // PURE: resolve_squad_order(...) -> SquadCommand (M3)
  save/
    mod.rs                   // SavePlugin: serde (de)serialization (M4)
    schema.rs                  // PURE, serde-only save/load data model (M4)
```

Acceptance bar for "is this pure enough": the `input.rs`, `meter.rs`,
`model.rs`, `command.rs`, and `schema.rs`-style files should compile and
`cargo test` with **no Bevy `App` at all**.

**M2's dialogue/Cover split, specifically:** keep the dialogue *data model*
(`dialogue/model.rs`) as plain `#[derive(Deserialize, Serialize)]` structs
with zero `bevy_ui` dependency, deserialized from a data file (RON/JSON) via
`serde`. A separate runtime layer walks that graph and emits events;
`ui/dialogue_box.rs` only reacts to those events to spawn `bevy_ui` nodes —
it imports `dialogue::model`, never the other way around. (This mirrors how
the real, actively-maintained `bevy_talks` crate structures its own
"Dialogue Database" vs. UI layer — a genuine existing precedent, not a
hypothetical.) Same split applies to `cover/meter.rs`: the clamped-delta math
is plain `f32` arithmetic, independently testable from whatever `bevy_ui`
widget later renders it.

## 6. How this maps onto the existing M0 increment list

No changes to `docs/craft-code/plans/2026-09-11-milestone-breakdown.md`'s
increment order or file plan — this doc just fills in the *how* underneath
it:

- Increment #1 → §1 (integration setup) + §2 (bootstrap commands).
- Increments #2–#5 → §3's headless-`App` pattern, directly; §5's pure/system
  split is exactly increment #3 (pure) feeding #4/#5 (systems).
- Increment #6 → manual-verification seam, as already noted in the
  milestone breakdown; no test pattern changes that.
- Increment #7 → the camera-offset math is pure per §5's rule (`P + O`),
  testable the same way as increment #3.
- M1 onward → §5's module layout and plugin-isolation rule are the
  going-forward convention once you re-slice M1–M4 (per the milestone
  breakdown's own note that re-slicing happens right before each
  milestone starts).

## Sources

- Bevy official quick-start: bevy.org/learn/quick-start/getting-started/{setup,apps,ecs}
- `bevyengine/bevy` repo, `v0.19.1` tag: `Cargo.toml` (feature flags, profile
  recommendations), `tests/how_to_test_apps.rs` (headless testing pattern),
  `examples/window/window_settings.rs`, `examples/showcase/breakout.rs`,
  `examples/input/keyboard_input.rs`
- `docs.rs/bevy` — `SystemParam`, `Resource` (dyn-compatibility) trait docs
- Unofficial Bevy Cheat Book — bevy-cheatbook.github.io/programming/plugins.html
  (plugin isolation convention)
- taintedcoders.com/bevy/{patterns/plugin-organization,code-organization,how-to/headless-mode}
- `TheBevyFlock/bevy_new_2d` — official/maintained Bevy project template
  (linked from bevy.org), confirms plugin-per-feature folder structure
- `bevy_talks` crate (crates.io, actively maintained) and its docs
  (giusdp.github.io/bevy_talks) — real precedent for dialogue-data-model vs.
  `bevy_ui` separation
- crates.io — `shaku`, `teloc` (checked for Bevy-ecosystem usage; none found)
- johanhelsing.studio/posts/extreme-bevy-2 — pure-function/thin-system split
  precedent
