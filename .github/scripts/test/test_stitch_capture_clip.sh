#!/usr/bin/env bash
set -uo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"
source ./_harness.sh
source ../stitch_capture_clip.sh

tmp="$(mktemp -d)"
trap 'rm -rf "${tmp}"' EXIT

test_select_existing_frames_filters_and_orders_by_playthrough_sequence() {
  local dir="${tmp}/select"
  mkdir -p "${dir}"
  touch "${dir}/end.png" "${dir}/character_select.png" "${dir}/drilling.png"
  mapfile -t frames < <(select_existing_frames "${dir}")
  assert_eq "character_select.png drilling.png end.png" "${frames[*]}" \
    "orders by playthrough sequence, skips missing frames"
}

test_main_skips_clip_when_fewer_than_two_frames_exist() {
  local dir="${tmp}/only-one"
  mkdir -p "${dir}"
  touch "${dir}/end.png"

  local output
  output="$(main "${dir}")"
  assert_eq "" "${output}" "fewer than 2 frames produces no clip path"
  assert_failure "clip.mp4 must not exist" bash -c "[[ -f '${dir}/clip.mp4' ]]"
}

test_main_invokes_ffmpeg_with_ordered_concat_list_when_enough_frames_exist() {
  local dir="${tmp}/full"
  mkdir -p "${dir}"
  touch "${dir}/character_select.png" "${dir}/drilling.png" "${dir}/call_event.png" \
    "${dir}/decision.png" "${dir}/end.png"

  local captured="${tmp}/ffmpeg-args"
  ffmpeg() { printf '%s\n' "$*" > "${captured}"; touch "${@: -1}"; }

  local output
  output="$(main "${dir}")"
  assert_eq "${dir}/clip.mp4" "${output}" "reports the clip path it created"
  assert_success "frames.txt lists the first frame in playthrough order" \
    bash -c "grep -q \"file 'character_select.png'\" '${dir}/frames.txt'"
  assert_success "frames.txt lists the last frame last" \
    bash -c "tail -n1 '${dir}/frames.txt' | grep -q \"file 'end.png'\""
  assert_success "ffmpeg was invoked" bash -c "[[ -s '${captured}' ]]"
}

test_select_existing_frames_filters_and_orders_by_playthrough_sequence
test_main_skips_clip_when_fewer_than_two_frames_exist
test_main_invokes_ffmpeg_with_ordered_concat_list_when_enough_frames_exist

report
