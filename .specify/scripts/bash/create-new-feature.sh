#!/usr/bin/env bash
set -euo pipefail

JSON_MODE=false
SHORT_NAME=""
BRANCH_NUMBER=""
ARGS=()

# -------------------------
# Argument parsing
# -------------------------
i=1
while (( i <= $# )); do
    arg="${!i}"
    case "$arg" in
        --json)
            JSON_MODE=true
            ;;
        --short-name)
            (( i++ ))
            [[ ${!i:-} == --* || -z ${!i:-} ]] && {
                echo "Error: --short-name requires a value" >&2
                exit 1
            }
            SHORT_NAME="${!i}"
            ;;
        --number)
            (( i++ ))
            [[ ${!i:-} == --* || -z ${!i:-} ]] && {
                echo "Error: --number requires a value" >&2
                exit 1
            }
            BRANCH_NUMBER="${!i}"
            ;;
        --help|-h)
            cat <<EOF
Usage: $0 [--json] [--short-name <name>] [--number N] <feature_description>
EOF
            exit 0
            ;;
        *)
            ARGS+=("$arg")
            ;;
    esac
    (( i++ ))
done

FEATURE_DESCRIPTION="${ARGS[*]}"
[[ -z "$FEATURE_DESCRIPTION" ]] && {
    echo "Error: feature description is required" >&2
    exit 1
}

# -------------------------
# Repo detection
# -------------------------
SCRIPT_DIR="$(CDPATH="" cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

find_repo_root() {
    local dir="$1"
    while [[ "$dir" != "/" ]]; do
        [[ -d "$dir/.git" || -d "$dir/.specify" ]] && {
            echo "$dir"
            return
        }
        dir="${dir%/*}"
    done
    return 1
}

if git rev-parse --show-toplevel >/dev/null 2>&1; then
    REPO_ROOT="$(git rev-parse --show-toplevel)"
    HAS_GIT=true
else
    REPO_ROOT="$(find_repo_root "$SCRIPT_DIR")" || {
        echo "Error: cannot determine repository root" >&2
        exit 1
    }
    HAS_GIT=false
fi

cd "$REPO_ROOT"

SPECS_DIR="$REPO_ROOT/specs"
mkdir -p "$SPECS_DIR"

# -------------------------
# Helpers
# -------------------------
clean_branch_name() {
    local s="${1,,}"
    s="${s//[^a-z0-9]/-}"
    s="${s//--/-}"
    s="${s#-}"
    s="${s%-}"
    echo "$s"
}

get_highest_from_specs() {
    local highest=0
    shopt -s nullglob
    for dir in "$SPECS_DIR"/*; do
        [[ -d "$dir" ]] || continue
        name="${dir##*/}"
        [[ "$name" =~ ^([0-9]+) ]] || continue
        num=$((10#${BASH_REMATCH[1]}))
        (( num > highest )) && highest=$num
    done
    shopt -u nullglob
    echo "$highest"
}

get_highest_from_branches() {
    local highest=0 branch name num
    while IFS= read -r branch; do
        branch="${branch#\* }"
        branch="${branch##*/}"
        [[ "$branch" =~ ^([0-9]{3})- ]] || continue
        num=$((10#${BASH_REMATCH[1]}))
        (( num > highest )) && highest=$num
    done < <(git branch -a 2>/dev/null || true)
    echo "$highest"
}

next_branch_number() {
    local a b
    a="$(get_highest_from_branches)"
    b="$(get_highest_from_specs)"
    (( a > b )) && echo $((a+1)) || echo $((b+1))
}

generate_branch_suffix() {
    local text="${1,,}"
    local words=() w
    for w in $text; do
        [[ "$w" =~ ^(a|an|the|to|for|of|in|on|at|by|with|from|is|are|was|were|be|been|have|has|had|do|does|did|will|would|should|could|can|may|might|must|shall|this|that|these|those|add|get|set)$ ]] && continue
        [[ ${#w} -lt 3 ]] && continue
        words+=("$w")
        (( ${#words[@]} >= 4 )) && break
    done

    if (( ${#words[@]} )); then
        printf "%s-" "${words[@]}" | sed 's/-$//'
    else
        clean_branch_name "$1"
    fi
}

# -------------------------
# Branch name construction
# -------------------------
if [[ -n "$SHORT_NAME" ]]; then
    BRANCH_SUFFIX="$(clean_branch_name "$SHORT_NAME")"
else
    BRANCH_SUFFIX="$(generate_branch_suffix "$FEATURE_DESCRIPTION")"
fi

if [[ -z "$BRANCH_NUMBER" ]]; then
    if $HAS_GIT; then
        git fetch --all --prune >/dev/null 2>&1 || true
        BRANCH_NUMBER="$(next_branch_number)"
    else
        BRANCH_NUMBER="$(( $(get_highest_from_specs) + 1 ))"
    fi
fi

FEATURE_NUM="$(printf "%03d" "$((10#$BRANCH_NUMBER))")"
BRANCH_NAME="${FEATURE_NUM}-${BRANCH_SUFFIX}"

# -------------------------
# Length enforcement
# -------------------------
MAX=244
if (( ${#BRANCH_NAME} > MAX )); then
    BRANCH_NAME="${FEATURE_NUM}-${BRANCH_SUFFIX:0:$((MAX-4))}"
    BRANCH_NAME="${BRANCH_NAME%-}"
    echo "[specify] Warning: branch name truncated" >&2
fi

# -------------------------
# Git + filesystem
# -------------------------
if $HAS_GIT; then
    git checkout -b "$BRANCH_NAME"
else
    echo "[specify] Warning: git not detected, branch not created" >&2
fi

FEATURE_DIR="$SPECS_DIR/$BRANCH_NAME"
mkdir -p "$FEATURE_DIR"

TEMPLATE="$REPO_ROOT/.specify/templates/spec-template.md"
SPEC_FILE="$FEATURE_DIR/spec.md"
[[ -f "$TEMPLATE" ]] && cp "$TEMPLATE" "$SPEC_FILE" || : > "$SPEC_FILE"

export SPECIFY_FEATURE="$BRANCH_NAME"

# -------------------------
# Output
# -------------------------
if $JSON_MODE; then
    printf '{"BRANCH_NAME":"%s","SPEC_FILE":"%s","FEATURE_NUM":"%s"}\n' \
        "$BRANCH_NAME" "$SPEC_FILE" "$FEATURE_NUM"
else
    echo "BRANCH_NAME: $BRANCH_NAME"
    echo "SPEC_FILE: $SPEC_FILE"
    echo "FEATURE_NUM: $FEATURE_NUM"
fi
