# Red Rising: The Game — Game Design Document (Draft v0.1)

> Status: **Draft for review.** This document lays out a first pass at features and
> gameplay for a game based on Pierce Brown's *Red Rising Saga*. Assumptions made
> to keep momentum are called out explicitly — flag any of them and we refine.

## 1. Pitch

A single-player, story-driven **action-RPG** that lets you live the Red Rising
saga from inside the skin of its two most iconic voices: **Darrow au Andromedus
(the Reaper)** and **Sevro au Barca (the Goblin)**. The campaign follows the
books' actual timeline — the Institute, the Academy, the war for Mars, the Rim
Rebellion — and reflects the story's core theme back into the mechanics: you
are always outnumbered, outgunned, and pretending to be something you're not,
and you win through cunning, loyalty, and violence, in that order.

**Assumed defaults (flag to change):**
- Single-player campaign is the core product; asynchronous/co-op multiplayer is a
  post-launch pillar, not required for v1.
- Third-person action-RPG, PC/console, similar tonal weight to *God of War* /
  *Horizon* — not an open-world sandbox, not a strategy game.
- Campaign covers *Red Rising* through *Morning Star* (the original trilogy) for
  v1, with *Iron Gold*→*Light Bringer* reserved as a sequel/DLC arc, since the
  POV cast expands significantly (Lysander, Ephraim, Lyria) past book 3.

## 2. Playable Characters

### 2.1 Darrow au Andromedus — "The Reaper"
The primary protagonist and default campaign character. Darrow is a **hybrid**:
raw physical gifts (post-Carving) plus a Red miner's guile. He is the character
who most needs the player to balance *three* pressures at once — combat,
disguise, and leadership.

- **Fighting style:** Razor mastery (a weapon that is stiff for cutting, whip-flexible
  for reach, this is the signature "read the room, then explode" moveset),
  gravBoots for aerial/repositioning combat, brute unarmed strength scaled by
  Gold-tier Carve stats.
- **Unique pillar — The Mask:** Darrow's core tension is *living a lie in enemy
  territory*. A persistent "Cover" meter tracks how well he's passing as a
  legitimate Gold; NPCs, dialogue choices, and how loudly you fight all move it.
  Blow the mask and you trigger story-critical, harder combat states (everyone
  now knows the Reaper is here) — sometimes intentional, sometimes a fail-state
  you have to fight or flee out of.
- **Unique pillar — Command:** In set-piece battles (Iron Rain, the siege of
  Agea, fleet actions) Darrow can issue **real-time squad orders** to Howlers or
  allied Legions on the field — a lightweight command layer, not full RTS, more
  like *Mass Effect* squad commands fused with a battlefield-flow minimap.

### 2.2 Sevro au Barca — "The Goblin"
The second playable character, unlocked structurally (see §4) and also
selectable for specific story chapters told from his POV (mirroring the books'
occasional Sevro-центric sequences and the Iron Gold era where he narrates).

- **Fighting style:** Small, fast, vicious — stealth-first with a "wolf" pounce
  ability, poison/blade combo kills, camouflage cloak use. Sevro is the
  counterpoint to Darrow's Carved-Gold power fantasy: he's the guy who was never
  supposed to win a fair fight, so the kit rewards ambush, terrain, and pack
  tactics over toe-to-toe brawling.
- **Unique pillar — The Howlers:** Sevro's chapters are built around **squad-based
  infiltration** — you physically place and direct your Howler pack (Clown,
  Pebble, Thistle, Screwface, etc.) before and during an engagement: who breaches,
  who watches the exit, who takes the shot. Think *Commandos*/*Shadow Tactics*
  lite grafted onto third-person stealth-action.
- **Unique pillar — Loyalty over Law:** Sevro's dialogue/approval system tracks
  loyalty to *people* (Darrow, Mustang, the Howlers) rather than Darrow's
  Cover/faction meter — choices ripple through which Howlers survive later
  chapters.

### 2.3 Additional POV characters (post-launch / late-campaign unlocks)
Short, focused "interlude" chapters (2–4 hours each, not full campaigns) let you
briefly play other core cast members at key story beats, mirroring the books'
multi-POV structure without diluting the two main character arcs:
- **Mustang (Virginia au Augustus)** — political-intrigue-heavy chapters: the
  Institute House Minerva sequences, court maneuvering, the Society reveal.
- **Cassius au Bellona** — a redemption-arc interlude post-*Red Rising*, playable
  in flashback/parallel-track story beats.
- **Ragnar volarus** — a brute-force interlude on Luna, heavier melee/no-razor
  kit reflecting his Obsidian fighting style.

*(This roster is intentionally deferred past v1 — flagged as a stretch goal so
the two-character core stays tight and shippable.)*

## 3. Core Gameplay Pillars

1. **Combat that punishes 1-vs-many by design.** You are almost never fighting
   fair. Combat systems (dodge/parry windows, gravBoot repositioning, terrain
   kills, Howler assists) exist specifically to make "outnumbered" a survivable,
   skill-expressible state rather than a loss condition.
2. **Identity as a mechanic, not flavor text.** Darrow's Cover meter and Sevro's
   Loyalty meter aren't cosmetic — they gate dialogue options, which NPCs will
   help you, and which combat/stealth tools are available in a given scene.
3. **The Color hierarchy is systemic.** Every Color (Red, Gold, Silver, Copper,
   Obsidian, Pink, Grey, etc.) has a visibly different role in the world: Greys
   as visible security/patrol AI archetypes, Coppers as bureaucratic
   puzzle-gates (forging papers, manipulating logistics), Obsidians as
   high-threat melee enemies/allies, Golds as the political and physical apex.
   This isn't just re-skinned enemies — it's the game's core "read the board"
   literacy, same as the books.
4. **Loyalty has a body count.** Characters you meet can die based on player
   choices and performance in set-pieces (mirroring the books' willingness to
   kill beloved characters). No forced-canon deaths beyond the books' own fixed
   points (e.g., Fitchner, Quinn) — but *secondary* Howlers and allies are
   genuinely at risk based on how a mission is played.

## 4. Structure: Campaign as "Books"

The campaign is organized into **Books**, each subdivided into **Chapters**,
directly mirroring the source structure so the game can be marketed and
patched book-by-book (also gives a natural season-pass/DLC shape later).

| Book (v1 scope)      | Primary POV      | Signature sequence                                  |
|-----------------------|-------------------|------------------------------------------------------|
| I — *Red Rising*      | Darrow            | The Passage (Carving), the Institute Houses war game |
| II — *Golden Son*     | Darrow / Sevro     | Iron Rain (orbital assault set-piece), the Jackal duel |
| III — *Morning Star*  | Darrow / Sevro / Mustang interlude | The Rim siege, the final duel on the Citadel |

Each Book ends in a large, mechanically distinct **set-piece finale** (the
Institute war games, the Iron Rain drop assault, the Citadel siege) that
recombines every system taught that Book — this is the "boss fight" equivalent.

## 5. Systems & Progression

- **Gene-mods / Carve tree:** A skill tree framed as *Carving* enhancements —
  strength, speed, regeneration, reflexes — unlocked with narrative-justified
  currency (medical resources, favor, stolen tech), not generic XP grinding.
- **Weapons:** Razor (signature, always available, deep combo tree), pulseFist/
  pulseKnife (Sevro's kit), slingBlade and terrain weapons picked up
  situationally, gravBoots as a traversal + combat-repositioning tool for
  Darrow specifically.
- **Cover / Loyalty meters** (per §2) as the two characters' unique resource,
  visible on the HUD, driving branch-locked dialogue and mission approaches.
- **Squad tactics (Howlers):** A lightweight command wheel — Hold, Breach,
  Flank, Regroup — usable in both Darrow's large-battle chapters and Sevro's
  infiltration chapters, with different tactical weight in each.
- **Difficulty framed in-world:** Difficulty tiers are named after Colors
  ("Pink" = story mode, up to "Obsidian" = hardest), reinforcing the world's
  own status hierarchy rather than generic Easy/Normal/Hard labels.

## 6. World & Settings

Confined, hand-authored levels rather than an open world, spanning the book's
real locations: Lykos mines (Mars), the Institute (Luna), Agea and the Citadel
(Mars), an Iron Rain drop sequence (orbit-to-surface), void/ship interiors for
fleet-battle chapters. Locations are chosen to give each Book a visually
distinct identity (mine-tunnel verticality → Institute's brutal wilderness
war-games → war-torn Mars cities).

## 7. Multiplayer (post-v1 pillar)

- **Iron Rain / Fleet skirmish mode:** Asymmetric small-squad PvP echoing the
  Institute's House-vs-House war games — Reds/Golds/Obsidians as
  player-selectable loadouts with the Color-based asymmetry from the campaign
  carried into PvP identity.
- **Howler co-op raids:** 2–4 player co-op missions built on Sevro's
  squad-infiltration kit, non-canon "what if" side ops.

## 8. Feature List (summary)

- Two fully playable campaign protagonists (Darrow, Sevro) with distinct kits,
  meters, and mission types.
- Book-structured single-player campaign (3 Books at launch, mapped to the
  original trilogy) with book-ending set-piece finales.
- Cover (Darrow) and Loyalty (Sevro) meters that gate dialogue, mission
  approach, and which allies live or die.
- Color-hierarchy-driven enemy/ally design (Grey security AI, Copper
  logistics puzzles, Obsidian heavy melee, Gold apex duelists, etc.).
- Squad-command layer (Howlers) usable in both large battles and stealth
  infiltration, context-shifted per character.
- Carve-tree progression (strength/speed/reflex/regen) framed diegetically.
- Signature weapon kit: razor combo system, gravBoots traversal-combat, Sevro's
  stealth/pounce/poison kit.
- In-world-named difficulty tiers.
- Interlude chapters for Mustang, Cassius, Ragnar (stretch goal, post-v1).
- Post-launch multiplayer: Iron Rain skirmish PvP + Howler co-op raids.
- Sequel/DLC arc reserved for *Iron Gold → Light Bringer* once the POV cast
  expands.

## 9. Technical Approach — Rust

The implementation language is **Rust**, chosen explicitly as a learning
vehicle — the tech plan below is written to *teach* Rust incrementally rather
than to front-load the biggest architecture possible.

- **Engine (decided): [Bevy](https://bevyengine.org/).** Chosen over Fyrox
  (the other finalist per §11's research) after weighing the trade-offs —
  it's the most idiomatic way to learn Rust through game code: everything is
  plain Rust (no visual editor/scripting language to hide the language
  behind), it's built on an ECS (entities/components/systems) which is a
  great forcing function for learning Rust's ownership/borrowing model in a
  concrete way, and its ecosystem has named, real crates for exactly what the
  milestone roadmap needs later (dialogue, squad AI, physics, third-person
  camera) plus the largest community of the pure-Rust engines to get unstuck
  with. Enable the `dynamic_linking` feature from the start to tame its
  comparatively worse compile times, given how often TDD means re-running
  `cargo test`/`cargo run`.
- **Why this changes scope, not ambition:** the full three-Book, two-character
  campaign in §4 is the *destination*, not the first deliverable. For a
  from-scratch Rust learner, that design is broken into a **learning roadmap**
  of small, always-playable milestones (below) instead of one big build.

### Learning-oriented milestone roadmap (supersedes a single "v1" cut)

| Milestone | Scope | Rust concepts it forces |
|---|---|---|
| **M0 — Skeleton** | Open a window, render a placeholder capsule, move it with WASD, basic camera. No combat yet. | Cargo project structure, Bevy App/Plugin basics, ECS components/systems, `Query`/`Res` borrowing |
| **M1 — Lykos vertical slice** | One small hand-built level (a mine tunnel), Darrow placeholder model, a single enemy type, one weapon (razor) with a 2–3 move combo, a scripted intro/outro. This is the first "book demo." | State machines (game states), input handling, collision, basic animation, enums/pattern matching for combat states |
| **M2 — Cover meter + dialogue** | Add Darrow's Cover meter, a small branching dialogue interaction, one NPC that reacts to it. | Data-driven design (loading dialogue from data files, e.g. RON/JSON via `serde`), event systems, UI (`bevy_ui`) |
| **M3 — Sevro + squad command** | Second playable character, a tiny 2–3-Howler squad-command demo (Hold/Flank on one small encounter). | More complex ECS relationships, trait objects vs. enums for character-specific behavior, shared vs. character-specific systems design |
| **M4 — Institute set-piece** | The first real "Book-ending" set-piece at small scale (a scaled-down House war-game skirmish). | Performance-aware ECS (many entities), possibly async/threading, save/load (`serde` + file I/O) |

Later milestones (Book II/III content, multiplayer, the full Color-hierarchy
systems from §5) stay in the design as the long-term target, but are
deliberately *not* scheduled yet — we'll re-plan past M4 once the learning
pace and Bevy comfort level are clearer.

**Actionable increments:** each milestone above is sliced into small,
independently-testable, red-green-refactor-sized steps in
[`docs/craft-code/plans/2026-09-11-milestone-breakdown.md`](craft-code/plans/2026-09-11-milestone-breakdown.md)
(M0 and M1 fully sliced now; M2–M4 sketched at milestone grain and re-sliced
as each one is reached, per `craft-code:planning`). Work that file top to
bottom alongside `craft-code:strict-tdd` — it's the concrete "what do I write
a test for next" answer this document intentionally stays too high-level for.

## 10. Process & Review — decided

**Ownership (decided):** you write all game code by hand, milestone by
milestone, to learn Rust. No coding sub-agent is dispatched to implement game
code at this stage. That will shift over time — as you get comfortable with
the language, we hand progressively more milestones to the coding team — but
the starting default is 100% hand-written.

**TDD is non-optional (decided).** Every increment of game code follows the
`craft-code:strict-tdd` discipline, translated to this Rust/Cargo project:

1. Write one failing test first (`cargo test`), for one small behavior.
2. Watch it fail (red) — confirms the test actually exercises the new
   behavior and isn't vacuously passing.
3. Write the minimum code to make it pass (green).
4. Refactor with the test green as a safety net (`craft-code:code-style` /
   `craft-code:refactoring` as the lens — immutability by default, small
   functions, no dead comments, clean module boundaries).
5. Commit at green, and again after refactor.

No production code is written ahead of a failing test — including "just this
one small thing." Real (non-mocked) collaborators are used wherever they run
deterministically in-process; test doubles are reserved for genuine external
seams (which, for a local single-player game, will be rare early on —
things like the OS clock/filesystem or, later, a network layer).

**Review process (decided):** review runs through the `craft-code` lens from
day one, even though the `dev-workflow` pipeline's full intake→plan→worktree
ceremony is deferred until it's earning its keep. Concretely, on request (or
at the end of each milestone) I dispatch an independent sub-agent with
`purpose: review` — currently `claude_code`, the only coding harness ready on
this host — to review your diff against:
- `craft-code:code-style` (immutability, naming, small functions, no
  explanatory comments for non-awkward code, results/null-objects over
  exceptions/nulls where idiomatic in Rust)
- `craft-code:strict-tdd` adherence (tests-first, one behavior at a time, real
  collaborators over mocks)
- the smell catalog behind `craft-code:refactoring`

The reviewer only reports findings — it never edits this repo. You decide
what to act on. Once a second harness (codex/opencode/cursor/hermes/pi/agy) is
available on this host, cross-vendor review becomes an option too, though with
a human author instead of a sub-agent implementer, that distinction matters
less here than it does for agent-authored PRs.

**Conventions file:** `.craft-code.yml` is now committed at the repo root with
the standard Cargo commands (`cargo build`/`test`/`clippy`/`fmt`) so every
future skill/agent invocation reads the same commands instead of guessing.

## 11. Engine Trade-off Research (2026-09-10)

Researched via crates.io/GitHub/official docs before locking the engine
choice. Full option set considered: **Bevy**, **macroquad**, **ggez**, and
**Fyrox** (the one other actively-maintained option judged clearly relevant).

| | **Bevy** | **macroquad** | **ggez** | **Fyrox** |
|---|---|---|---|---|
| Latest release | 0.19 (Jun 2026) | 0.4.16 (Jul 2026) | 0.10.0 (Jun 2026) | 1.0.0 (Mar 2026) |
| Activity | Very high, ~quarterly | High, funding-constrained | Low, ~annual, losing mindshare to Bevy | Moderate, steady, just hit 1.0 |
| Community | 47.5k★, 1,527 contributors, 23k Discord | 4.3k★, "Quads" Discord | 4.7k★, no dedicated Discord | 8.8k★, active Discord |
| Architecture | ECS | Immediate-mode | Minimal retained (`EventHandler`) | Retained scene graph + visual editor |
| Beginner borrow-checker friction | Real, ~2-3 wks to adjust, but well-documented | Lowest — no imposed data model | Low — you design your own structs | Low-moderate — arena handles instead of ECS queries |
| 3D maturity | Production-viable, used in shipping/near-shipping titles | Rudimentary, not production-ready | Deliberately capped, no advanced rendering planned | Production-viable, editor-supported |
| Official docs | New official Book (2026) + mature Unofficial Cheat Book | Thin | FAQ + thin docs | Official Book; editor reduces docs-dependency |
| Compile times | Worst of the four (mitigated by `dynamic_linking`/Cranelift) | Best of the four | Better than Bevy, worse than macroquad | Bevy-ish, but has logic hot-reloading |
| API churn | High — pre-1.0, migration guide every release | Low, small stable surface | Low, infrequent releases | Just hit 1.0 — churn should drop, still young |
| Built-in UI | `bevy_ui` + `bevy_egui` | None (`egui` bolt-on) | None | Own 30+-widget UI framework |
| Built-in animation/physics | Animation: early but native. Physics: 3rd-party (avian/rapier), mature | Neither | Neither | Both bundled and native |
| M0–M4 milestone fit | Good, slightly heavy for M0 alone | Good for M0, strains by M2–M3 | OK for M0–M1, strains fast after | Good — editor helps level/UI milestones |
| Long-term 3D action-RPG fit | Strong — largest ecosystem to grow into | Likely requires a rewrite | Likely requires a rewrite | Strong — native 3D + editor, no-rewrite path |

**Framing:** it's effectively a two-horse race between **Bevy** and **Fyrox**
for the multi-year foundation; macroquad/ggez are both easier on day one but
have maintainer-acknowledged 3D ceilings, so committing years of hand-written
game code to either risks a full engine migration exactly when the project
reaches "actually a 3D action-RPG" — the most expensive point to rewrite.

- **Bevy** — largest community/ecosystem, and crucially has *named, real*
  crates for exactly what the milestone roadmap needs later: `bevy_yarnspinner`
  (dialogue), `big-brain` (squad/utility AI), `avian`/`bevy_rapier3d`
  (physics), `bevy_tnua` + `bevy_third_person_camera` (character control).
  Costs: pre-1.0 churn you'll eat repeatedly, ECS is a second thing to learn
  on top of Rust itself, worse compile times (real but mitigated — enable the
  `dynamic_linking` feature from the start given how often TDD means running
  `cargo test`/`cargo run`).
- **Fyrox** — the more surprising finding: just hit 1.0 (lower future churn
  risk than Bevy), 3D-native with a real Unity/Godot-style visual editor
  (helps specifically with the level-building and UI/dialogue milestones),
  ships animation/physics/UI in-house rather than as a patchwork of
  third-party crates. Cost: much smaller community/crate ecosystem — more
  glue code (e.g. squad AI) falls to you rather than an existing crate.
- **macroquad / ggez** — best for a short-lived Rust warm-up or scratch
  prototyping, not recommended as the permanent foundation given the explicit
  3D ceiling both maintainers have stated.

*Sub-agent's bet, offered as input, not a decision made for you:* Bevy, on
ecosystem depth for the specific systems this project's own milestones name
(dialogue, AI, third-person camera) and the much larger pool of people to get
unstuck with as a simultaneous Rust+gamedev beginner. Fyrox is a legitimate
alternative if the visual editor and lower post-1.0 churn matter more to you
than ecosystem size — M0 is small enough to trial in either before committing.

**Decision (2026-09-11): Bevy.** See §9 for the finalized rationale. M0 is
unblocked — `cargo new`, Bevy dependency (with `dynamic_linking` enabled),
window, placeholder capsule, WASD movement, test-first per `craft-code:strict-tdd`.

## 12. Open Questions For You

1. ~~Engine pick~~ — **resolved: Bevy** (§9, §11).
2. Genre lock beyond "the Rust learning roadmap": is action-RPG (my original
   assumption) still right for the long-term destination, or did you want
   tactics/strategy, narrative adventure, or multiplayer-first PvP instead?
3. Scope: original trilogy as the eventual full-build target, or *Iron Gold*
   onward in scope too (bigger cast, bigger world)?

Neither (2) nor (3) blocks starting M0 — the engine is picked.
