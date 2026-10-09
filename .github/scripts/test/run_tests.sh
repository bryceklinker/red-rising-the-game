#!/usr/bin/env bash
set -uo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"

status=0
for test_file in test_*.sh; do
  echo "== ${test_file} =="
  bash "${test_file}" || status=1
done
exit "${status}"
