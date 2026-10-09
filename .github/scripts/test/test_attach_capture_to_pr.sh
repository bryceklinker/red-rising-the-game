#!/usr/bin/env bash
set -uo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"
source ./_harness.sh
source ../attach_capture_to_pr.sh

tmp="$(mktemp -d)"
trap 'rm -rf "${tmp}"' EXIT

test_select_existing_attachments_filters_to_present_files_in_fixed_order() {
  local dir="${tmp}/select"
  mkdir -p "${dir}"
  touch "${dir}/clip.mp4" "${dir}/character_select.png"
  mapfile -t files < <(select_existing_attachments "${dir}")
  assert_eq "${dir}/character_select.png ${dir}/clip.mp4" "${files[*]}" \
    "only existing expected files are selected, screenshots before clip"
}

test_main_fails_when_no_capture_output_exists() {
  local dir="${tmp}/empty"
  mkdir -p "${dir}"
  assert_failure "main must fail with no attachments to post" main 123 "${dir}"
}

test_main_invokes_gh_with_an_attach_flag_per_existing_file() {
  local dir="${tmp}/full"
  mkdir -p "${dir}"
  touch "${dir}/character_select.png" "${dir}/end.png"

  local captured="${tmp}/gh-args"
  gh() { printf '%s\n' "$*" > "${captured}"; }

  main 456 "${dir}"
  assert_success "gh was invoked" bash -c "[[ -s '${captured}' ]]"
  assert_success "gh was called with the PR number" bash -c "grep -q '456' '${captured}'"
  assert_success "both existing files were attached" \
    bash -c "grep -q '${dir}/character_select.png' '${captured}' && grep -q '${dir}/end.png' '${captured}'"
}

test_select_existing_attachments_filters_to_present_files_in_fixed_order
test_main_fails_when_no_capture_output_exists
test_main_invokes_gh_with_an_attach_flag_per_existing_file

report
