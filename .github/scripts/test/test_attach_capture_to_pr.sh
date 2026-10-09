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

test_build_artifact_comment_links_to_the_run_artifacts_and_explains_why() {
  local body
  body="$(build_artifact_comment "https://github.com" "bryceklinker/red-rising-the-game" "999")"
  assert_success "comment links to this run's actions page" \
    bash -c "printf '%s' '${body}' | grep -q 'https://github.com/bryceklinker/red-rising-the-game/actions/runs/999'"
  assert_success "comment explains the GITHUB_TOKEN attach limitation" \
    bash -c "printf '%s' '${body}' | grep -q 'cli/cli#14309'"
}

test_main_posts_a_comment_linking_the_run_artifact_without_attach_flags() {
  local dir="${tmp}/full"
  mkdir -p "${dir}"
  touch "${dir}/character_select.png" "${dir}/end.png"

  local captured="${tmp}/gh-args"
  gh() { printf '%s\n' "$*" > "${captured}"; }

  main 456 "${dir}" "https://github.com" "bryceklinker/red-rising-the-game" "999"
  assert_success "gh was invoked" bash -c "[[ -s '${captured}' ]]"
  assert_success "gh was called with the PR number" bash -c "grep -q '456' '${captured}'"
  assert_success "the comment links to the run's artifacts, not --attach" \
    bash -c "grep -q 'actions/runs/999' '${captured}' && ! grep -q -- '--attach' '${captured}'"
}

test_select_existing_attachments_filters_to_present_files_in_fixed_order
test_main_fails_when_no_capture_output_exists
test_build_artifact_comment_links_to_the_run_artifacts_and_explains_why
test_main_posts_a_comment_linking_the_run_artifact_without_attach_flags

report
