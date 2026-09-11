# Plan: Milestone breakdown into actionable increments

> Companion to `docs/GDD.md` §9 (milestone roadmap). Each milestone below is
> sliced into thin, independently-testable increments per
> `craft-code:planning` — one behavior, one failing test, minimal code to
> green, refactor, commit. Work top to bottom within a milestone; don't start
> increment N+1 until N is green and committed.
>
> Since all code here is hand-written solo (no sub-agent implementers per
> `docs/GDD.md` §10), the `[independent]`/`[depends: N]` tags below are for
> your own sequencing clarity, not for parallel dispatch — you can still use
> them to know which increments could be reordered if one gets stuck.
>
> **Bevy-testing tip used throughout:** most of these increments don't need a
> window or renderer to test. Build an `App` with `MinimalPlugins` (or just
> `App::new()` + the specific plugin under test), insert the
> entities/resources you need by hand, call `app.update()` once (or a few
> times to simulate ticks), then assert on component/resource state. That's a
> real, in-process, non-mocked way to test ECS systems — no doubles needed.

## M0 — Skeleton

Goal per GDD §9: open a window, render a placeholder capsule, move it with
WASD, basic camera. First real Cargo/Bevy contact.

> **See `docs/craft-code/plans/2026-09-11-m0-implementation-plan.md`** for the
> concrete "how" underneath these increments: exact Bevy integration/setup
> steps, a command cheat sheet, the full headless-`App` + `MinimalPlugins`
> testing pattern, the realistic dependency-injection options in Bevy, and
> the pure-logic-vs-system module layout to use from increment #1 onward.

1. **[independent] Project scaffolded and builds clean**
   - Behavior: `cargo new red-rising` (or workspace member) with Bevy added
     as a dependency, `dynamic_linking` feature enabled for dev builds.
   - "Test": there's no behavior to unit-test yet — the acceptance bar is
     `cargo build` and `cargo run` both succeed and open an empty window.
     Treat this as the one pre-TDD scaffolding step; TDD starts at #2.
   - Files: `Cargo.toml`, `src/main.rs`.

2. **[depends: 1] Player entity spawns with identifying marker**
   - Behavior: on startup, exactly one entity exists with a `Player` marker
     component and a `Transform` at the origin.
   - Test: headless `App` (`MinimalPlugins`), run the startup system once via
     `app.update()`, query for `With<Player>`, assert count == 1 and
     `Transform::translation == Vec3::ZERO`.
   - Files: `src/player.rs` (new), `src/main.rs` (register plugin).

3. **[depends: 2] Raw WASD key state maps to a movement direction**
   - Behavior: a pure function/system takes `ButtonInput<KeyCode>` and
     produces a normalized `Vec2`/`Vec3` direction (e.g. W+D → diagonal,
     normalized so it isn't faster than a single key).
   - Test: no Bevy app needed for the pure-function version — call it
     directly with a fake `ButtonInput` state built via Bevy's own
     `ButtonInput::press`; assert direction and normalization (including the
     "no diagonal speed boost" case explicitly, since it's an easy bug).
   - Files: `src/movement/input.rs` (new).

4. **[depends: 2] Movement direction updates player Transform over time**
   - Behavior: given a non-zero direction and a fixed speed, one `app.update()`
     tick moves the player's `Transform.translation` by
     `direction * speed * delta_time`.
   - Test: headless `App`, insert a `Time` resource advanced by a known delta
     (Bevy supports manually advancing `Time` in tests), assert the resulting
     translation matches the expected math.
   - Files: `src/movement/apply.rs` (new).

5. **[depends: 3, 4] WASD end-to-end moves the player entity**
   - Behavior: wire #3's input system → #4's movement system via a shared
     component/event so pressing W actually moves the spawned player entity
     one full tick.
   - Test: headless `App` with both systems registered, simulate a pressed
     `KeyCode::KeyW`, call `app.update()`, assert the player's `Transform`
     moved in the expected (forward) direction by the expected amount.
   - Files: `src/movement/mod.rs`, `src/player.rs`.

6. **[depends: 5] Capsule mesh renders at the player's Transform**
   - Behavior: spawn a capsule `Mesh3d`/`MeshMaterial3d` on the player entity
     so it's visible on screen at the position #2–#5 already drive correctly.
   - "Test": this is rendering glue with no meaningful headless assertion —
     verify by eye (`cargo run`, confirm a capsule appears and WASD moves it).
     Note this explicitly as a manual-verification step, not a gap in TDD
     discipline (per `craft-code:verification` — some seams are genuinely
     UI/rendering and get proven by running the app, not a unit test).
   - Files: `src/player.rs`.

7. **[depends: 6] Basic camera follows the player**
   - Behavior: a fixed-offset third-person (or simple orbit) camera whose
     position is a deterministic function of the player's `Transform`.
   - Test: the offset-calculation itself is pure and testable headless
     (given player position P and offset O, camera position == P + O);
     visual confirmation of "does it look right" is manual per #6's note.
   - Files: `src/camera.rs` (new).

**M0 exit condition:** `cargo run` opens a window, shows a capsule, WASD
moves it smoothly, a camera trails it — and every non-rendering behavior
above has a green headless test behind it.

## M1 — Lykos vertical slice

Goal per GDD §9: one hand-built level (mine tunnel), Darrow placeholder,
one enemy type, one weapon (razor) with a 2–3 move combo, scripted intro/outro.
Broken down further once M0 lands and its actual APIs/patterns are known —
sketched now at the grain the current design supports:

1. **[depends: M0] Game-state machine: Intro → Playing → Outro**
   - Behavior: an explicit `States` enum drives which systems run; starting
     state is `Intro`, a trigger (timer or input) transitions to `Playing`.
   - Test: headless `App`, assert initial state, simulate the trigger,
     assert transition to `Playing`.
   - Files: `src/game_state.rs` (new).

2. **[independent of 1] Static level geometry loads (the mine tunnel)**
   - Behavior: a level-loading system spawns fixed collidable geometry from
     a data description (even a hardcoded list of boxes at this stage).
   - Test: headless `App`, assert the expected number/shape of collider
     entities exist after the load system runs.
   - Files: `src/level.rs` (new).

3. **[depends: 1, 2] Player collides with level geometry (can't walk through walls)**
   - Behavior: player movement (M0 #4/#5) is clamped/blocked by level colliders.
   - Test: headless `App`, place a wall collider directly in the movement
     path, simulate forward input for N ticks, assert translation stops at
     the wall boundary rather than passing through.
   - Files: `src/movement/collision.rs` (new).

4. **[depends: 1] Enemy entity spawns with health and a simple AI state**
   - Behavior: one enemy archetype spawns with `Health(n)` and an
     `EnemyState` (e.g. `Idle`/`Aggro`) driven by distance to the player.
   - Test: headless `App`, place player within/outside aggro radius across
     two runs, assert `EnemyState` flips accordingly.
   - Files: `src/enemy.rs` (new).

5. **[depends: 4] Razor attack reduces enemy health on hit**
   - Behavior: a basic attack input applies damage to an enemy within range;
     health hitting 0 marks the enemy for despawn.
   - Test: headless `App`, position enemy in range, simulate attack input,
     assert health decreases by the expected amount; assert despawn marker
     appears once health <= 0.
   - Files: `src/combat/razor.rs` (new).

6. **[depends: 5] Razor combo: second/third attack within a window deals bonus damage**
   - Behavior: chaining attacks within a timing window increases damage or
     changes the animation state (combo counter).
   - Test: headless `App`, simulate attack, advance a small delta, simulate
     a second attack inside the window, assert combo counter/bonus applied;
     simulate outside the window, assert combo resets instead.
   - Files: `src/combat/razor.rs`.

7. **[depends: 3, 6] Scripted intro/outro plays around the encounter**
   - Behavior: entering `Playing` triggers a short scripted sequence (camera
     pan or fixed dialogue beat); defeating the enemy transitions to `Outro`.
   - Test: state-transition logic tested per #1's pattern; the actual
     camera-pan visuals are a manual-verification seam like M0 #6.
   - Files: `src/game_state.rs`, `src/level.rs`.

## M2 — Cover meter + dialogue

Goal per GDD §9: Darrow's Cover meter, one branching dialogue interaction,
one reactive NPC. Sketched at milestone grain for now — re-slice into
increments (per this same template) once M1 lands, since the actual dialogue
data shape (RON/JSON via `serde`) and event wiring should follow real M1
patterns rather than be guessed today. Known must-have increments to include
when it's re-sliced:
- Cover meter as a resource/component with explicit raise/lower events.
- Cover crossing a threshold gates at least one dialogue branch.
- Dialogue content loads from a data file (not hardcoded strings) via `serde`.
- One NPC reacts (visibly/behaviorally) to current Cover state.

## M3 — Sevro + squad command

Goal per GDD §9: second playable character, 2–3-Howler squad-command demo.
Re-slice once M2 lands. Known must-have increments:
- Character-select or character-swap mechanism (Darrow ↔ Sevro).
- Sevro's distinct movement/combat kit (stealth pounce, no razor combo).
- Howler squad entities with a command-target component (Hold/Flank).
- One command issued by the player changes at least one Howler's behavior
  observably (position or engagement state).

## M4 — Institute set-piece

Goal per GDD §9: scaled-down House war-game skirmish. Re-slice once M3
lands. Known must-have increments:
- Multiple simultaneous enemy/ally entities without frame-time regression
  (perf-aware ECS — establish a basic benchmark/test budget here).
- Save/load of encounter state via `serde` + file I/O.
- Win/lose condition detection for the skirmish.

## Why M2–M4 aren't fully sliced yet

Slicing increments this thin requires knowing the real APIs/module shapes a
prior milestone produced (per `craft-code:planning`'s independence rule: two
increments are only independent if they touch disjoint files, which you can't
judge accurately for code that doesn't exist yet). Re-run this same
breakdown exercise for M2 once M1 is green, for M3 once M2 is green, and so
on — each milestone's plan should land in this same file (or a new
`docs/craft-code/plans/YYYY-MM-DD-<milestone>.md`) right before you start it.
