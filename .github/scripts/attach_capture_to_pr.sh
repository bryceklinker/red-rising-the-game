#!/usr/bin/env bash
set -euo pipefail

readonly EXPECTED_FILES=(character_select.png drilling.png call_event.png decision.png end.png clip.mp4)

select_existing_attachments() {
  local capture_dir="$1"
  local name
  for name in "${EXPECTED_FILES[@]}"; do
    [[ -f "${capture_dir}/${name}" ]] && echo "${capture_dir}/${name}"
  done
  return 0
}

# GitHub Actions' default GITHUB_TOKEN is a server-to-server installation
# token (ghs_), which `gh pr comment --attach` always rejects client-side
# ("unsupported authentication type") and which the upload endpoint itself
# 404s even with write access — see https://github.com/cli/cli/issues/14309.
# There is no supported way to get inline-rendered media onto the PR with
# only GITHUB_TOKEN, so this links to the run's workflow artifact instead.
build_artifact_comment() {
  local server_url="$1"
  local repository="$2"
  local run_id="$3"
  cat <<EOF
Scripted playthrough capture for this PR is ready as a workflow artifact (screenshots + clip.mp4): ${server_url}/${repository}/actions/runs/${run_id}

(GitHub Actions' GITHUB_TOKEN can't upload attachments directly to PR comments — see [cli/cli#14309](https://github.com/cli/cli/issues/14309) — so this links to the run artifact instead of embedding the media inline.)
EOF
}

main() {
  local pr_number="$1"
  local capture_dir="$2"
  local server_url="${3:-${GITHUB_SERVER_URL:-https://github.com}}"
  local repository="${4:-${GITHUB_REPOSITORY:-}}"
  local run_id="${5:-${GITHUB_RUN_ID:-}}"

  local attachments=()
  while IFS= read -r file; do
    attachments+=("${file}")
  done < <(select_existing_attachments "${capture_dir}")

  if [[ "${#attachments[@]}" -eq 0 ]]; then
    echo "attach-capture-to-pr: no capture output found in ${capture_dir}" >&2
    return 1
  fi

  gh pr comment "${pr_number}" --body "$(build_artifact_comment "${server_url}" "${repository}" "${run_id}")"
}

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  main "$@"
fi
