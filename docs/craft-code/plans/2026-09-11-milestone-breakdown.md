# Plan: Milestone breakdown into actionable increments

> Companion to `docs/GDD.md` §9. Sliced per `craft-code:planning`: one
> behavior, one failing test, minimal code to green, refactor, commit.
> Work top to bottom; don't start N+1 before N is green and committed.
> Hand-written solo — `[independent]`/`[depends: N]` are sequencing hints
> only, not parallel-dispatch tags.

**Testing tip:** `App::new()` + `MinimalPlugins` (or just the plugin under
test), insert entities/resources by hand, `app.update()`, assert state.
Real in-process ECS testing, no doubles needed.

## M0 — Skeleton

Goal: window, placeholder capsule, WASD movement, basic camera.
See `2026-09-11-m0-implementation-plan.md` for the "how" (commands, DI,
module layout, full headless-test example).

1. **[independent] Project scaffolds and builds**
   - `cargo new` + `bevy` dep, `dynamic_linking` enabled for dev.
   - No test yet — bar is `cargo build`/`cargo run` open an empty window.
   - Files: `Cargo.toml`, `src/main.rs`.

2. **[depends: 1] Player entity spawns**
   - `Player` marker + `Transform::ZERO` on startup.
   - Test: headless `App`, `app.update()`, query `With<Player>` → count 1,
     translation `== ZERO`.
   - Files: `src/player.rs`, `src/main.rs`.

3. **[depends: 2] WASD → direction (pure)**
   - `ButtonInput<KeyCode>` → normalized direction; no diagonal speed boost.
   - Test: plain unit test, no `App`.
   - Files: `src/movement/input.rs`.

4. **[depends: 2] Direction → Transform**
   - One tick moves `translation` by `direction * speed * delta_time`.
   - Test: headless `App`, advance `Time` by a known delta, assert math.
   - Files: `src/movement/apply.rs`.

5. **[depends: 3, 4] WASD moves the player end-to-end**
   - Wire #3 → #4 so pressing W moves the spawned player one tick.
   - Test: headless `App`, press `KeyCode::KeyW`, `app.update()`, assert
     `Transform` moved forward by the expected amount.
   - Files: `src/movement/mod.rs`, `src/player.rs`.

6. **[depends: 5] Scene visible: mesh + light + fixed camera**
   - Bundled, not split, because none of the three is individually
     verifiable by eye until all three exist:
     - capsule `Mesh3d`/`MeshMaterial3d` on the player entity
     - a light (`PointLight`/`DirectionalLight`) — a `StandardMaterial`
       capsule with no light renders solid black, indistinguishable from
       "nothing spawned"
     - `Camera3d` at a fixed position pointed at the origin (follow logic
       is #7, not this)
   - Test: manual — `cargo run`, confirm a lit capsule is visible and WASD
     moves it. Rendering glue has no meaningful headless assertion; this is
     a documented manual-verification seam, not a TDD gap.
   - Files: `src/player.rs`, `src/camera.rs`.

7. **[depends: 6] Camera follows the player**
   - Camera position becomes `f(player Transform)` every frame, replacing
     #6's static position.
   - Test: offset math is pure and testable headless (`P + O`); "looks
     right"/"tracks smoothly" is manual.
   - Files: `src/camera.rs`.

**Exit:** `cargo run` opens a window, shows a lit capsule, WASD moves it
smoothly, camera trails it — every non-rendering behavior has a green
headless test.

## M1 — Lykos vertical slice

Goal: one hand-built level (mine tunnel), Darrow placeholder, one enemy,
razor + 2–3 move combo, scripted intro/outro.

1. **[depends: M0] State machine: Intro → Playing → Outro**
   - `States` enum gates which systems run; starts `Intro`.
   - Test: headless `App`, assert initial state, trigger, assert transition.
   - Files: `src/game_state.rs`.

2. **[independent of 1] Level geometry loads**
   - Loader spawns fixed collidable geometry from a data description.
   - Test: headless `App`, assert expected collider entity count/shape.
   - Files: `src/level.rs`.

3. **[depends: 1, 2] Player can't walk through walls**
   - Movement clamped/blocked by level colliders.
   - Test: headless `App`, wall in path, N ticks forward, assert stopped
     at the boundary.
   - Files: `src/movement/collision.rs`.

4. **[depends: 1] Enemy spawns with health + AI state**
   - `Health(n)` + `EnemyState` (`Idle`/`Aggro`) driven by player distance.
   - Test: headless `App`, player in/out of aggro radius, assert state flips.
   - Files: `src/enemy.rs`.

5. **[depends: 4] Razor attack damages enemy**
   - Attack input damages an in-range enemy; 0 health → despawn marker.
   - Test: headless `App`, enemy in range, attack, assert damage + despawn
     marker at 0 health.
   - Files: `src/combat/razor.rs`.

6. **[depends: 5] Razor combo bonus**
   - Chained attacks within a timing window add bonus damage/anim state.
   - Test: headless `App` — 2 attacks inside window → bonus applied;
     outside window → combo resets.
   - Files: `src/combat/razor.rs`.

7. **[depends: 3, 6] Scripted intro/outro**
   - Entering `Playing` triggers a scripted beat; defeating the enemy →
     `Outro`.
   - Test: transitions tested per #1; visuals are manual per M0 #6.
   - Files: `src/game_state.rs`, `src/level.rs`.

## M2 — Cover meter + dialogue

Goal: Cover meter, one branching dialogue interaction, one reactive NPC.
Sketch only — re-slice once M1 lands (needs M1's real file shapes/APIs).
Must-haves:
- Cover meter resource/component + explicit raise/lower events.
- Cover crossing a threshold gates ≥1 dialogue branch.
- Dialogue content from a data file via `serde`, not hardcoded strings.
- ≥1 NPC reacts (visibly/behaviorally) to current Cover state.

## M3 — Sevro + squad command

Goal: second playable character, 2–3-Howler squad-command demo. Re-slice
once M2 lands. Must-haves:
- Character-select/swap (Darrow ↔ Sevro).
- Sevro's distinct kit (stealth pounce, no razor combo).
- Howler entities with a command-target component (Hold/Flank).
- ≥1 player command observably changes a Howler's position/engagement state.

## M4 — Institute set-piece

Goal: scaled-down House war-game skirmish. Re-slice once M3 lands.
Must-haves:
- Multiple simultaneous entities, no frame-time regression (perf
  budget/benchmark established here).
- Save/load of encounter state via `serde` + file I/O.
- Win/lose condition detection.

## Why M2–M4 aren't sliced yet

Increment independence (`craft-code:planning`) requires knowing real file
shapes a prior milestone produced — can't judge that for code that doesn't
exist yet. Re-run this breakdown for M2 once M1 is green, M3 once M2 is
green, and so on. Land each milestone's slice in this file (or a new
`docs/craft-code/plans/YYYY-MM-DD-<milestone>.md`) right before starting it.
