#!/usr/bin/env bash
set -uo pipefail

TESTS_RUN=0
TESTS_FAILED=0

assert_eq() {
  local expected="$1" actual="$2" message="${3:-assert_eq}"
  TESTS_RUN=$((TESTS_RUN + 1))
  if [[ "${expected}" != "${actual}" ]]; then
    TESTS_FAILED=$((TESTS_FAILED + 1))
    echo "FAIL: ${message}"
    echo "  expected: ${expected}"
    echo "  actual:   ${actual}"
  fi
}

assert_success() {
  local message="$1"
  shift
  TESTS_RUN=$((TESTS_RUN + 1))
  if ! "$@"; then
    TESTS_FAILED=$((TESTS_FAILED + 1))
    echo "FAIL: ${message} (expected success, command: $*)"
  fi
}

assert_failure() {
  local message="$1"
  shift
  TESTS_RUN=$((TESTS_RUN + 1))
  if "$@"; then
    TESTS_FAILED=$((TESTS_FAILED + 1))
    echo "FAIL: ${message} (expected failure, command: $*)"
  fi
}

report() {
  echo "${TESTS_RUN} run, ${TESTS_FAILED} failed"
  [[ "${TESTS_FAILED}" -eq 0 ]]
}
