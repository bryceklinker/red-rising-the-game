
# Plan: M1 — Lykos: The Laurel and the Vale

> Redefines M1 (superseding the old "mine tunnel + enemy + razor combo" slice
> in `2026-09-11-milestone-breakdown.md`). Decision: **M1 stays on the real
> book path** instead of a decoupled combat demo. Grounded in book-canon
> research (`research-book-beats-post-gas-pocket`, 2026-10-07) — see that
> report for full citations/confidence notes.

## Why this scope, and not further

Canon beats from the gas-pocket scene through the Institute span ~16 of 44
chapters (~35% of book 1) and are tonally/mechanically distinct in two
clusters:

1. **Ch.1–4: Laurel snub → Vale/Gift → caught.** Low-risk to adapt — the
   quota/gas-pocket scene is *already our own invented content* (canon has no
   radioed warning), and the Vale scene's "how Darrow reacts" is genuinely
   under-specified interior space. Nothing here is plot-load-bearing in a way
   a game choice could break.
2. **Ch.5–7: whipping → Reaping Song → Eo's execution → Darrow's fake-hanging
   rescue.** This is the emotional core of the book, entirely fixed/linear —
   Eo's death cannot branch, and Darrow's role in it (forced to pull her legs
   down because Mars's lower gravity won't break her neck) is a forced act,
   not a choice of outcome. It deserves its own careful milestone, not to be
   bolted onto this one.

**M1 (this doc) = cluster 1 only.** Cluster 2 becomes the next milestone
once this lands — tentatively "M1.5" until the GDD's M2–M4 table is
renumbered (see **Roadmap note** at the bottom; not solved here).

## Increment 0 — Rename uncle Nero → Narol

**Why:** canon's **Nero au Augustus** is the ArchGovernor who later sentences
Eo to hang — in *this exact milestone cluster*. Keeping the uncle named
"Nero" means two named "Nero"s appear across M1/M1.5. Canon's actual name for
Darrow's uncle (who warns him in the mines, later fakes his death) is
**Narol**.

- Rename `"Nero"` → `"Narol"` in `call_event.rs`'s dialogue string and any
  other reference.
- Update doc mentions (`milestone-breakdown.md`, `m0-prologue-vertical-slice.md`,
  `GDD.md` §9 table) — mechanical find/replace, not a design change.
- No test needed (string content, not behavior) — but re-run the existing
  `call_event` tests to confirm nothing asserts on the literal string.

## Increment 1 — Decision has real branching consequences

**Current bug:** `end_state_for_choice` is a 1:1 relabel; all 3 choices are
mechanically identical. Canon gives real freedom here (research §3) as long
as Darrow survives and the Laurel snub still happens regardless.

- Replace the flat relabel with a `DrillingOutcome` carrying real divergent
  data per choice:
  - `KeepDrilling` → higher measured yield, a burn/injury flag, Narol's
    reaction is alarmed/angry.
  - `GetOutAndCheck` → safe, moderate yield, Narol's reaction is relieved.
  - `WaitForTheTeam` → safe, lowest yield (lost time), Narol's reaction is
    approving-but-tired.
- Test: pure function `drilling_outcome_for(DecisionOption) -> DrillingOutcome`
  — plain `#[test]`, no `App`, one case per variant, assert distinct fields.
- Files: `src/decision/outcome.rs`.

## Increment 2 — Laurel snub (replaces `End` as a mid-slice beat, not terminal)

Regardless of `DrillingOutcome`, Lambda clan is denied the Laurel despite the
numbers (canon: systemic betrayal, not a death) — this is what feeds
Darrow's resentment into the Vale scene.

- New state `GameState::LaurelSnub`. Reads `DrillingOutcome`, picks flavor
  text (yield number, burn mention if any, Narol's reaction), always
  transitions to the same next state (`Vale`) — the outcome changes *what's
  said*, never *what happens next*.
- Test: headless `App`, one case per `DrillingOutcome` variant → assert the
  correct flavor text is selected AND the transition target is identical
  across all three.
- Files: `src/laurel_snub.rs`.

## Increment 3 — Vale/Gift scene: reach the window

Eo leads Darrow through a vent to a hidden garden with a view of Mars's
terraformed surface — first time he's seen the sky.

- New state `GameState::Vale`. Reuse the existing WASD movement
  (`move_player`/`direction_from_pressed_keys`) for a short corridor; a
  trigger volume/depth threshold (same pattern as `check_drill_depth`) fires
  on reaching the window.
- Test: headless `App`, drive input to the trigger point, assert transition
  to the next state (`VaultReaction`, increment 4).
- Files: `src/vale.rs`.

## Increment 4 — Player reaction choice at the window (flavor only)

Canon leaves Darrow's interior reaction under-specified — safe ground for a
real player choice, as long as it doesn't change the outcome (they get caught
either way).

- 2–3 reaction options (e.g. awe / anger / grief), each recorded in a
  resource/log; all transition identically to the next state (`Caught`,
  increment 5).
- Test: headless `App`, one case per reaction choice → assert it's recorded
  AND the transition target is identical across all choices (mirrors
  increment 2's "outcome varies, transition doesn't" pattern).
- Files: `src/vale/reaction.rs`.

## Increment 5 — Caught (M1's end-of-slice cliffhanger)

Grays catch Darrow and Eo on the way back — a true dead end for this
milestone; cluster 2 (whipping → execution → rescue) picks up from here next.

- New terminal state `GameState::Caught` — scripted text/banner, no further
  transition (same pattern as the current `End` state).
- Test: transition correctness only (reached from increment 4 regardless of
  choice). Visuals/scripted banner are manual-verification per the M0
  convention — not a TDD gap.
- Files: `src/caught.rs`, `src/game_state.rs` (extend the enum, wire new
  plugins into `main.rs`).

**Exit:** `cargo run` plays CharacterSelect → Drilling → CallEvent (Narol) →
Decision (3 real-flavor outcomes) → LaurelSnub → Vale → window reaction
choice → Caught. Every transition has a green headless test; rendering/text
content is eyeballed per the existing M0 manual-verification convention.

## Roadmap note (not resolved here)

The existing GDD §9 table's **M2 (Cover meter + dialogue)** assumes Darrow is
already a Carved, infiltrating Gold — but Carving (Ch.12) and the Institute
(Ch.17+) are now well past this milestone and its sequel. Once cluster 2
(whipping → Eo's death → Darrow's rescue, "M1.5") lands, the GDD's M2–M4
sequence needs a renumbering pass — likely inserting "Sons of Ares +
Carving" before Cover-meter content can make sense. Re-run that planning
exercise then, per this project's own existing convention of re-slicing each
milestone right before it starts — not guessed now.
