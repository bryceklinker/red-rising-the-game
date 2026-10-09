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

main() {
  local pr_number="$1"
  local capture_dir="$2"

  local attachments=()
  while IFS= read -r file; do
    attachments+=("${file}")
  done < <(select_existing_attachments "${capture_dir}")

  if [[ "${#attachments[@]}" -eq 0 ]]; then
    echo "attach-capture-to-pr: no capture output found in ${capture_dir}" >&2
    return 1
  fi

  local attach_args=()
  local file
  for file in "${attachments[@]}"; do
    attach_args+=(--attach "${file}")
  done

  gh pr comment "${pr_number}" --body "Scripted playthrough capture for this PR:" "${attach_args[@]}"
}

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  main "$@"
fi
