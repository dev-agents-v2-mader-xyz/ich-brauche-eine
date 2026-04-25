#!/bin/sh
# Post-deploy smoke tests. Non-zero exit = rollback.
# Usage: BASE_URL=https://my-app.mader.xyz ./tests/smoke.sh
set -e

BASE="${BASE_URL:-http://localhost:8000}"
FAIL=0

check() {
  local desc="$1"
  local url="$2"
  local expected_status="${3:-200}"

  actual=$(curl -s -o /dev/null -w "%{http_code}" "$url")
  if [ "$actual" = "$expected_status" ]; then
    echo "  ✓ $desc ($actual)"
  else
    echo "  ✗ $desc — expected $expected_status, got $actual"
    FAIL=1
  fi
}

echo "Smoke tests against $BASE"
echo ""

check "health endpoint"         "$BASE/health"  200
check "frontend loads"          "$BASE/"        200
check "404 for unknown path"    "$BASE/definitely-not-a-path-xyzzy" 404

# Add application-specific checks here:
# check "items API"  "$BASE/api/items"  200

echo ""
if [ "$FAIL" = "1" ]; then
  echo "SMOKE TESTS FAILED — triggering rollback"
  exit 1
else
  echo "All smoke tests passed."
fi
