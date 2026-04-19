#!/usr/bin/env bash
# sync-agents-md.sh
# PostToolUse hook として使用
# CLAUDE.md が編集されたとき、全セクションを AGENTS.md に同期する

# jq の存在確認
if ! command -v jq &> /dev/null; then
    echo "[sync-agents-md] jq が見つかりません。同期をスキップします" >&2
    exit 0
fi

# stdin から JSON を読み取る
INPUT=$(cat)

# tool_input.file_path を取得
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // ""')

# CLAUDE.md 以外はスキップ
if [[ "$(basename "$FILE_PATH")" != "CLAUDE.md" ]]; then
    exit 0
fi

# cwd を使ってファイルパスを解決
CWD=$(echo "$INPUT" | jq -r '.cwd // ""')
if [ -n "$CWD" ]; then
    CLAUDE_MD="$CWD/CLAUDE.md"
    AGENTS_MD="$CWD/AGENTS.md"
else
    CLAUDE_MD="CLAUDE.md"
    AGENTS_MD="AGENTS.md"
fi

if [ ! -f "$CLAUDE_MD" ]; then
    exit 0
fi

# CLAUDE.md から除外するセクション（Claude Code 固有で AGENTS.md に不要なセクション）
# 現時点では除外なし。必要に応じてセクション名を追加する。
EXCLUDE_SECTIONS=()

# セクション内容を抽出する関数（先頭・末尾の空白行を除去）
extract_section() {
    local file="$1"
    local section="$2"
    awk -v sec="$section" '
        $0 ~ "^## " sec "$" { found=1; n=0; next }
        found && /^## / { exit }
        found { buf[n++] = $0 }
        END {
            start = 0
            for (i = 0; i < n; i++) {
                if (buf[i] ~ /[^[:space:]]/) { start = i; break }
            }
            last = -1
            for (i = n - 1; i >= 0; i--) {
                if (buf[i] ~ /[^[:space:]]/) { last = i; break }
            }
            for (i = start; i <= last; i++) print buf[i]
        }
    ' "$file"
}

# CLAUDE.md のセクション名一覧を出現順に収集（除外リストを除く）
claude_sections=()
while IFS= read -r line; do
    if [[ "$line" =~ ^##\ (.+)$ ]]; then
        section_name="${BASH_REMATCH[1]}"
        is_excluded=false
        for excl in "${EXCLUDE_SECTIONS[@]}"; do
            if [ "$section_name" = "$excl" ]; then
                is_excluded=true
                break
            fi
        done
        if [ "$is_excluded" = false ]; then
            claude_sections+=("$section_name")
        fi
    fi
done < "$CLAUDE_MD"

# AGENTS.md の固有セクション名を収集（CLAUDE.md に存在しないセクション、順序を保持）
agents_unique_sections=()
if [ -f "$AGENTS_MD" ]; then
    while IFS= read -r line; do
        if [[ "$line" =~ ^##\ (.+)$ ]]; then
            section_name="${BASH_REMATCH[1]}"
            in_claude=false
            for cs in "${claude_sections[@]}"; do
                if [ "$section_name" = "$cs" ]; then
                    in_claude=true
                    break
                fi
            done
            if [ "$in_claude" = false ]; then
                agents_unique_sections+=("$section_name")
            fi
        fi
    done < "$AGENTS_MD"
fi

# AGENTS.md を再生成
{
    printf "# AGENTS.md\n"
    printf "\n"
    printf "This file provides guidance to OpenAI Codex when working with code in this repository.\n"

    # CLAUDE.md の全セクション（除外対象を除く、出現順）
    for section in "${claude_sections[@]}"; do
        content=$(extract_section "$CLAUDE_MD" "$section")
        if [ -n "$content" ]; then
            printf "\n"
            printf "## %s\n" "$section"
            printf "\n"
            printf "%s\n" "$content"
        fi
    done

    # AGENTS.md 固有セクション（元の内容を保持、元の順序を維持）
    for section in "${agents_unique_sections[@]}"; do
        content=$(extract_section "$AGENTS_MD" "$section")
        if [ -n "$content" ]; then
            printf "\n"
            printf "## %s\n" "$section"
            printf "\n"
            printf "%s\n" "$content"
        fi
    done
} > "${AGENTS_MD}.tmp"

mv "${AGENTS_MD}.tmp" "$AGENTS_MD"

echo "[sync-agents-md] AGENTS.md を CLAUDE.md の変更に合わせて更新しました" >&2
