#!/usr/bin/env bash
set -euo pipefail

# blur | mirror
plugin="${1:-blur}"
# dev | release
mode="${2:-dev}"

case "$plugin" in
  mirror)
    input=".cp/girls_good.png"
    output=".cp/mirror"
    params=".cp/mirror/mirror.cfg"
    ;;
  blur)
    input=".cp/girls_good.png"
    output=".cp/blur"
    params=".cp/blur/blur.cfg"
    ;;
  *)
    echo "Unknown plugin: $plugin"
    echo "Usage: ./run.sh [blur|mirror] [dev|release]"
    exit 1
    ;;
esac

if [[ "$mode" == "release" ]]; then
  cargo run --release --bin iproc -- -i "$input" -o "$output" -p "$params" -n "$plugin"
else
  cargo run --bin iproc -- -i "$input" -o "$output" -p "$params" -n "$plugin"
fi
