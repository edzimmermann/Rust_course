#!/bin/sh
set -eu
TMP="${TMPDIR:-/tmp}/rust_cpp_fixit_100_check"
rm -rf "$TMP"
mkdir -p "$TMP"
count=0
find . -path '*/solutions/*.rs' -type f | sort | while IFS= read -r f; do
  count=$((count + 1))
  out="$TMP/bin_$count"
  echo "[$count] rustc $f"
  rustc --edition=2021 "$f" -o "$out"
done
echo "All Rust solution files compiled successfully."
