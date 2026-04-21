#!/bin/bash
set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: $(basename "$0") <title>" >&2
    exit 1
fi

title="$1"
kanban_dir="$(cd "$(dirname "$0")/.." && pwd)/kanban"

# 既存ディレクトリから最大番号を取得
max_num=-1
for d in "$kanban_dir"/[0-9][0-9][0-9][0-9]_*/; do
    [ -d "$d" ] || continue
    num="$(basename "$d")"
    num="${num%%_*}"
    num=$((10#$num))
    if [ "$num" -gt "$max_num" ]; then
        max_num="$num"
    fi
done

next_num=$((max_num + 1))
padded=$(printf "%04d" "$next_num")
dir="${kanban_dir}/${padded}_${title}"
filename="${dir}/${padded}_${title}.md"

mkdir -p "$dir"
touch "$filename"
echo "Created: $filename"
