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

- **Engine (proposed default): [Bevy](https://bevyengine.org/).** It's the
  most idiomatic way to learn Rust through game code: everything is plain
  Rust (no visual editor/scripting language to hide the language behind),
  it's built on an ECS (entities/components/systems) which is a great forcing
  function for learning Rust's ownership/borrowing model in a concrete way,
  and its community + docs are the strongest of the pure-Rust engines.
  - Lighter alternatives if a smaller learning surface is preferred:
    **macroquad** (minimal, immediate-mode, closest to "just write a game
    loop") or **ggez** (simple 2D, more traditional API). Bevy is the better
    pick if the medium-term goal is the full 3D action-RPG in §1–§8; macroquad/
    ggez are better if the near-term goal is "learn Rust with something small
    and 2D first, worry about the big vision later."
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

## 10. Open Questions For You

1. **Who writes the Rust?** This is the big one, given "learning experience"
   is the stated goal: do you want to hand-write the Rust yourself (with the
   team producing design docs, architecture/ECS guidance, and code review
   only — no sub-agent commits to the game code), or do you want the coding
   sub-agents to implement milestones for you (faster progress, less hands-on
   learning)? A middle path also works well: you write it, and a sub-agent
   acts purely as a **reviewer** (cross-checking idiomatic Rust/Bevy patterns
   on your commits) without ever writing code itself. This determines how I
   run the rest of this project, so I'm pausing on it rather than guessing.
2. Engine pick: Bevy (my default above) vs. macroquad/ggez for a gentler
   start — or something else you have in mind?
3. Genre lock beyond "the Rust learning roadmap": is action-RPG (my original
   assumption) still right for the long-term destination, or did you want
   tactics/strategy, narrative adventure, or multiplayer-first PvP instead?
4. Scope: original trilogy as the eventual full-build target, or *Iron Gold*
   onward in scope too (bigger cast, bigger world)?

Once (1) is answered I'll know whether to start dispatching implementation
milestones to the coding team, or instead switch into a design-doc-and-review
role while you write the Rust yourself.
