# Pipeline change: PR screenshot/video capture stage

Targeted addition to the existing `.github/workflows/ci.yml` (`fmt → build →
test → clippy`, confirmed working end-to-end as of PR #1/#2). This note
covers only the areas the change touches; everything else about the
pipeline is unchanged.

## Artifact strategy

**New artifact type, not the deployable one.** The existing pipeline has no
deploy/promotion yet (single build binary, local-run game) — this change
doesn't add one either. The capture stage produces ephemeral PNG frames
(+ optional stitched mp4/gif) that exist only to give a PR reviewer visual
context. They are:
- **Not versioned or content-addressed** — no promotion path needs to find
  them again by digest.
- **Not uploaded as a build artifact (`actions/upload-artifact`)** — that
  produces a buried zip download, not visible review context, which is the
  actual ask.
- **Delivered by attaching directly to the PR via `gh pr comment --attach`**
  (native `gh` CLI feature, ships `≥2.99.0`; GH-hosted `ubuntu-latest`
  already has `2.101.0`, confirmed). GitHub hosts the attached media itself
  — nothing for this repo to retain, clean up, or version.

**Why not an orphan branch + raw.githubusercontent.com:** extra repo
pollution, manual cleanup, camo-proxy auth quirks on private repos. The
native `--attach` flag is strictly simpler and didn't exist when that
workaround pattern was established.

## Stage ordering

New job runs **after** the existing 4-gate job, **only on `pull_request`**
(there's no PR to comment on for a push-to-`main` run). It must build and
run the real compiled game binary in capture mode, so it depends on a
successful build — no value in capturing screenshots of code that doesn't
compile.

Within the new job: `apt-get install` the two new system deps → build the
capture binary/feature → run the scripted session (produces PNG frames,
optionally ffmpeg-stitches a clip) → `gh pr comment --attach`.

## Gate map — the one decision that matters most here

**The capture stage is explicitly NOT a hard gate.** It must not block
merge if screenshot generation fails (headless/software-rendering paths
are inherently more fragile than a pure compile — e.g. a lavapipe driver
hiccup on the runner). Implementation: separate job from the existing
4-gate job, `continue-on-error: true` on the capture/attach steps. The
existing `fmt`/`build`/`test`/`clippy` gate keeps its current blocking
behavior, completely unchanged by this addition.

**Why:** the ask is "add context to PRs," not "add a new reason PRs can't
merge." A flaky rendering environment failing the whole PR would train
exactly the wrong behavior (ignoring or disabling the check).

## Promotion flow

**Not implicated.** No deploy/promotion exists in this pipeline yet.

## Reproducibility seams

New system deps (`mesa-vulkan-drivers`, `ffmpeg`) are installed unpinned via
`apt-get`, matching the existing convention for `pkg-config`/
`libwayland-dev`/etc. already in the workflow — no existing precedent for
pinning apt package versions in this repo, so this doesn't introduce a new
inconsistency.

## Secrets & config boundary

No new secrets. `gh pr comment --attach` runs against the default
`GITHUB_TOKEN`, which needs the job to declare
`permissions: pull-requests: write` explicitly (least-privilege — the
existing 4-gate job needs no write permissions and keeps none).

## Evidence of done

The PR comment itself, with inline-rendered screenshots/video, **is** the
evidence of done for this stage — there's no separate health check to
define; a human looking at the comment is the verification.

## Failure diagnostics

Since this stage is non-blocking by design, a failure must still be
*visible*, not silent: if the capture/attach step fails, post a short PR
comment saying so explicitly (e.g. "screenshot capture failed this run —
see job logs") rather than just leaving no comment at all, so a reviewer
doesn't mistake "no comment" for "nothing changed visually." Standard job
log output covers debugging the cause; no extra artifact capture is
warranted for a non-blocking, low-stakes dev-experience feature.

## Not implicated (unchanged)

Existing `fmt`/`build`/`test`/`clippy` stage ordering, caching
(`Swatinem/rust-cache`), and gate behavior are untouched by this change.

## Correction (2026-10-09): `--attach` does not work with `GITHUB_TOKEN`

The assumption above — that `gh pr comment --attach` would work against the
default `GITHUB_TOKEN` — was wrong, confirmed by a real CI failure (job
113725484038 of run 37900866074) and by reading `gh`'s own source
(`internal/attachments/client.go`): `GITHUB_TOKEN` is a server-to-server
installation token (`ghs_`), and `checkUploadTokenType` allowlists only
`gho_`/`ghp_`/`github_pat_`/`ghu_`, rejecting `ghs_` client-side with
"unsupported authentication type" before any network call. The upload
endpoint itself also 404s installation tokens even with write access
(`cli/cli#14309`), so there is no auth-mechanism, version, or permissions
fix available — this is a deliberate, permanent restriction, not a bug.

No new secret was introduced to work around it (would need a PAT with
write access stored as a repo secret, trading "no new secrets" for inline
media). Instead the stage now uses `actions/upload-artifact` — the option
originally ruled out above — and the PR comment links to the run's
artifacts page instead of embedding media inline. This is a real
regression from the original goal (reviewers must download a zip, not see
an inline image), accepted here because the capture stage is explicitly
non-blocking/best-effort and getting *some* visible signal beats either
silence or re-introducing the orphan-branch workaround this design
deliberately avoided. Revisit with a scoped PAT secret if inline rendering
becomes worth that trade.
