# Agent conventions

## Limit cargo concurrency

This machine is resource-constrained: 12 CPU cores but only ~15GiB RAM, and
most of it is already committed to other work. A default `cargo build`,
`cargo test`, or `cargo clippy` invocation fans out roughly one `rustc` job
per core; running that unthrottled — especially with more than one worktree
compiling at once (e.g. multiple agents working in parallel) — exhausts
available memory and swaps, making every build far slower than a capped one
would be.

**Rule: always cap cargo's parallelism explicitly.** Pass `--jobs 4` on every
`cargo build`, `cargo test`, and `cargo clippy` invocation:

    cargo build --jobs 4
    cargo test --jobs 4
    cargo clippy --all-targets --all-features --jobs 4 -- -D warnings
    cargo fmt --check   # formatting is not parallel-compile-heavy, no cap needed

If you know another cargo build/test is already running concurrently
(another worktree, another agent), drop to `--jobs 2` for your own
invocation rather than competing for the full machine.

This also applies to the equivalent commands in `.craft-code.yml` — those
already bake in `--jobs 4` so skill-driven runs get the cap automatically.

## Worktree build cache (shared target-dir)

Every git worktree under this repo (main plus anything in `.worktrees/`)
shares ONE cargo `target-dir` via `.cargo/config.toml` at the repo root:

    [build]
    target-dir = "/home/bryce/code/personal/red-rising-the-game/red-rising/target"

Cargo discovers `.cargo/config.toml` by walking up from the current working
directory, and every worktree's `red-rising/` subdir is a descendant of the
repo root, so this applies automatically — there is no per-worktree setup
step. A brand-new worktree's first `cargo build`/`test`/`clippy` reuses
main's already-compiled dependency tree (Bevy is the expensive part) instead
of recompiling everything from scratch, which is what was previously costing
huge amounts of time per worktree.

Side effect: cargo's target-dir lock file now serializes concurrent builds
across worktrees (one invocation waits for the other's lock) rather than
letting them run fully in parallel. Combined with the `--jobs` cap above,
this is a deliberate trade: slightly less parallelism, far less memory
pressure and far less redundant compilation on this resource-constrained
machine. If you see a build "hang" briefly at the start, it's very likely
waiting on this lock, not stuck — check for another live cargo process
before assuming a problem.

## Hard rule: only one worktree compiles at a time

We hit this for real: two worktrees each running a `--jobs`-capped
`cargo build`/`cargo test` at the same time still exhausted RAM and swap
badly enough to crash the entire machine and force a reboot. The shared
`target-dir` lock (above) only serializes cargo's own internal bookkeeping
for a moment at startup — it does **not** prevent two separate `cargo`
invocations from each fanning out their own capped-but-still-real `rustc`
process trees and running concurrently for the full build. Capping `--jobs`
alone is not sufficient; the real fix is mutual exclusion across worktrees.

**Rule: before starting any `cargo build`, `cargo test`, or `cargo clippy`,
check for a live cargo/rustc process anywhere else on the machine first:**

    ps aux | grep -E 'cargo|rustc' | grep -v grep

If that shows ANY cargo/rustc process belonging to a different worktree,
do **not** start your own build/test/clippy run. Wait until it's gone (poll,
don't guess) before proceeding. This applies regardless of `--jobs` value —
`--jobs 2` concurrent with another `--jobs 2` run is what crashed the
machine; it is not a safe combination on this host.

If you are an orchestrating agent (e.g. polly) dispatching multiple
sub-agents into different worktrees of this repo, this means you must
actively serialize their heavy cargo steps yourself: dispatch one sub-agent
to carry its build/test/clippy pass through to green first, hold the other
idle (don't send it a task that triggers a build) until the first confirms
completion, then dispatch the next one. Do not rely on sub-agents to
self-coordinate across separate worktrees — they cannot see each other's
state, only the orchestrator can.

