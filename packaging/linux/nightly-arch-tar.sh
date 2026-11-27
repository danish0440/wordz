#!/usr/bin/env bash
# Stage the WordZ Nightly-only handoff for the Arch packager.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
source_tree="$1"
work="$2"
destination="$3"
basename="$4"
stage="$work/$basename"

mkdir -p "$work"
cp -R "$source_tree" "$stage"
mkdir "$stage/bin/.wordz-nightly"
mv "$stage/bin/wordz" "$stage/bin/.wordz-nightly/wordz"
cp "$here/nightly-launcher.sh" "$stage/bin/wordz"
chmod 755 "$stage/bin/wordz"
tar -C "$work" -czf "$destination/$basename.tar.gz" "$basename"
echo "wrote $destination/$basename.tar.gz"
