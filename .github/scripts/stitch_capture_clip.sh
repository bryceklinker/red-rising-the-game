#!/usr/bin/env bash
set -euo pipefail

readonly FRAME_ORDER=(character_select drilling call_event decision end)
readonly FRAME_DURATION_SECONDS=2

select_existing_frames() {
  local capture_dir="$1"
  local frame
  for frame in "${FRAME_ORDER[@]}"; do
    [[ -f "${capture_dir}/${frame}.png" ]] && echo "${frame}.png"
  done
  return 0
}

write_concat_list() {
  local capture_dir="$1"
  shift
  local list_path="${capture_dir}/frames.txt"
  : > "${list_path}"
  local frame
  for frame in "$@"; do
    printf "file '%s'\nduration %s\n" "${frame}" "${FRAME_DURATION_SECONDS}" >>"${list_path}"
  done
  # ffmpeg's concat demuxer ignores the last entry's duration; repeat the
  # final frame with no duration so it actually holds on screen.
  printf "file '%s'\n" "${*: -1}" >>"${list_path}"
  echo "${list_path}"
}

main() {
  local capture_dir="$1"
  local frames=()
  while IFS= read -r frame; do
    frames+=("${frame}")
  done < <(select_existing_frames "${capture_dir}")

  if [[ "${#frames[@]}" -lt 2 ]]; then
    echo "stitch-capture-clip: fewer than 2 frames found in ${capture_dir}, skipping clip" >&2
    return 0
  fi

  local list_path
  list_path="$(write_concat_list "${capture_dir}" "${frames[@]}")"
  local clip_path="${capture_dir}/clip.mp4"
  ffmpeg -y -f concat -safe 0 -i "${list_path}" -vf "fps=1,format=yuv420p" "${clip_path}" >/dev/null 2>&1
  echo "${clip_path}"
}

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  main "$@"
fi
