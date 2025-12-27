#!/usr/bin/env bash
# Common functions and variables for all scripts

get_repo_root() {
    if git rev-parse --show-toplevel >/dev/null 2>&1; then
        git rev-parse --show-toplevel
    else
        local script_dir
        script_dir="$(CDPATH="" cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
        (cd "$script_dir/../../.." && pwd)
    fi
}

get_current_branch() {
    if [[ -n "${SPECIFY_FEATURE:-}" ]]; then
        echo "$SPECIFY_FEATURE"
        return
    fi

    if git rev-parse --abbrev-ref HEAD >/dev/null 2>&1; then
        git rev-parse --abbrev-ref HEAD
        return
    fi

    local repo_root specs_dir latest_feature="" highest=0
    repo_root="$(get_repo_root)"
    specs_dir="$repo_root/specs"

    if [[ -d "$specs_dir" ]]; then
        for dir in "$specs_dir"/*; do
            [[ -d "$dir" ]] || continue
            local name number
            name="$(basename "$dir")"
            if [[ "$name" =~ ^([0-9]{3})- ]]; then
                number=$((10#${BASH_REMATCH[1]}))
                (( number > highest )) && highest=$number && latest_feature="$name"
            fi
        done
    fi

    [[ -n "$latest_feature" ]] && echo "$latest_feature" || echo "main"
}

has_git() {
    git rev-parse --show-toplevel >/dev/null 2>&1
}

find_feature_dir_by_prefix() {
    local repo_root="$1" branch="$2" specs_dir="$repo_root/specs"

    [[ "$branch" =~ ^([0-9]{3})- ]] || {
        echo "$specs_dir/$branch"
        return
    }

    local prefix="${BASH_REMATCH[1]}" match=""
    for dir in "$specs_dir"/"$prefix"-*; do
        [[ -d "$dir" ]] || continue
        [[ -n "$match" ]] && {
            echo "ERROR: Multiple spec directories with prefix $prefix" >&2
            echo "$specs_dir/$branch"
            return
        }
        match="$(basename "$dir")"
    done

    [[ -n "$match" ]] && echo "$specs_dir/$match" || echo "$specs_dir/$branch"
}

get_feature_paths() {
    local repo_root current_branch has_git_repo="false"
    repo_root="$(get_repo_root)"
    current_branch="$(get_current_branch)"
    has_git && has_git_repo="true"

    local feature_dir
    feature_dir="$(find_feature_dir_by_prefix "$repo_root" "$current_branch")"

    cat <<EOF
REPO_ROOT='$repo_root'
CURRENT_BRANCH='$current_branch'
HAS_GIT='$has_git_repo'
FEATURE_DIR='$feature_dir'
FEATURE_SPEC='$feature_dir/spec.md'
IMPL_PLAN='$feature_dir/plan.md'
TASKS='$feature_dir/tasks.md'
RESEARCH='$feature_dir/research.md'
DATA_MODEL='$feature_dir/data-model.md'
QUICKSTART='$feature_dir/quickstart.md'
CONTRACTS_DIR='$feature_dir/contracts'
EOF
}

check_file() {
    [[ -f "$1" ]] && echo "  ✓ $2" || echo "  ✗ $2"
}

check_dir() {
    shopt -s nullglob dotglob
    local files=("$1"/*)
    shopt -u nullglob dotglob
    [[ -d "$1" && ${#files[@]} -gt 0 ]] && echo "  ✓ $2" || echo "  ✗ $2"
}
