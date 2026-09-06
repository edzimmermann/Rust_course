#!/bin/sh
set -eu
OUT="${1:-rust_fixit_student_pack.zip}"
TMP="${TMPDIR:-/tmp}/rust_fixit_student_pack"
rm -rf "$TMP"
mkdir -p "$TMP"
cp README.md "$TMP/README.md"
for d in [0-9][0-9]_*/; do
  mkdir -p "$TMP/$d"
  cp "$d/README.md" "$TMP/$d/README.md"
  cp -R "$d/exercises" "$TMP/$d/exercises"
done
(cd "$TMP" && zip -qr "$OLDPWD/$OUT" .)
echo "$OUT"
