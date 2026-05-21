#!/usr/bin/env bash
set -euo pipefail

# Sync REXPaint image files from your Windows REXPaint images directory
# into the project's assets/ directory.
#
# Usage: ./sync_assets.sh
#
# Override default source dir with: REXPAINT_IMAGES_DIR=/some/path ./sync_assets.sh

SOURCE_DIR="${REXPAINT_IMAGES_DIR:-/mnt/c/Users/grant/Downloads/REXPaint-v1.70/REXPaint-v1.70/images}"
DEST_DIR="assets"

if [ ! -d "$SOURCE_DIR" ]; then
  echo "Error: source directory not found: $SOURCE_DIR"
  echo "Set REXPAINT_IMAGES_DIR to the correct path."
  exit 1
fi

mkdir -p "$DEST_DIR"

echo "Syncing REXPaint assets from: $SOURCE_DIR"
echo "                      to: $DEST_DIR"
echo

copied=0
while IFS= read -r -d '' src_file; do
  rel="${src_file#$SOURCE_DIR/}"
  dest="$DEST_DIR/$rel"

  if [ -f "$dest" ] && cmp -s "$src_file" "$dest"; then
    : # unchanged, skip
  else
    mkdir -p "$(dirname "$dest")"
    cp -v "$src_file" "$dest"
    copied=$((copied + 1))
  fi
done < <(find "$SOURCE_DIR" \( -name '*.xp' -o -name '*.png' \) -type f -print0 2>/dev/null)

if [ "$copied" -eq 0 ]; then
  echo "All assets up to date."
else
  echo "Done — copied $copied files."
fi
