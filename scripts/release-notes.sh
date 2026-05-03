#!/usr/bin/env bash
set -euo pipefail

output_file="${1:-release-notes.md}"
release_tag="${RELEASE_TAG:-${GITHUB_REF_NAME:-}}"

if [[ -z "$release_tag" ]]; then
  release_tag="$(git describe --tags --abbrev=0 2>/dev/null || true)"
fi

if [[ -z "$release_tag" ]]; then
  release_tag="$(git rev-parse --short HEAD)"
fi

current_ref="$release_tag"
current_sha="$(git rev-list -n 1 "$current_ref" 2>/dev/null || git rev-parse HEAD)"
previous_tag="${PREVIOUS_TAG:-}"

if [[ -z "$previous_tag" ]]; then
  previous_tag="$(git describe --tags --abbrev=0 "${current_sha}^" 2>/dev/null || true)"
fi

if [[ -n "$previous_tag" ]] && ! git rev-parse -q --verify "${previous_tag}^{commit}" >/dev/null; then
  previous_tag=""
fi

if [[ -n "$previous_tag" ]]; then
  git_range="${previous_tag}..${current_sha}"
else
  git_range="$current_sha"
fi

features=()
fixes=()
performance=()
refactors=()
documentation=()
tests_and_tooling=()
other_changes=()

append_commit() {
  local subject="$1"
  local normalized="${subject,,}"

  case "$normalized" in
    feat:* | feat\(*)
      features+=("- $subject")
      ;;
    fix:* | fix\(*)
      fixes+=("- $subject")
      ;;
    perf:* | perf\(*)
      performance+=("- $subject")
      ;;
    refactor:* | refactor\(*)
      refactors+=("- $subject")
      ;;
    docs:* | docs\(*)
      documentation+=("- $subject")
      ;;
    test:* | test\(* | chore:* | chore\(* | ci:* | ci\(* | build:* | build\(*)
      tests_and_tooling+=("- $subject")
      ;;
    *)
      other_changes+=("- $subject")
      ;;
  esac
}

commit_lines="$(git log "$git_range" --pretty=format:'%h%x09%s')"

while IFS=$'\t' read -r _sha subject; do
  [[ -n "${subject:-}" ]] || continue
  append_commit "$subject"
done <<< "$commit_lines"

write_section() {
  local title="$1"
  shift
  local commits=("$@")

  if [[ "${#commits[@]}" -eq 0 ]]; then
    return
  fi

  printf '## %s\n' "$title"
  printf '%s\n' "${commits[@]}"
  printf '\n'
}

{
  printf '# Stremio Enhanced %s\n\n' "$release_tag"

  if [[ -n "$previous_tag" ]]; then
    printf 'Changes since %s.\n\n' "$previous_tag"
  else
    printf 'Initial tagged release.\n\n'
  fi

  total_commits=$((
    ${#features[@]} +
    ${#fixes[@]} +
    ${#performance[@]} +
    ${#refactors[@]} +
    ${#documentation[@]} +
    ${#tests_and_tooling[@]} +
    ${#other_changes[@]}
  ))

  if [[ "$total_commits" -eq 0 ]]; then
    printf -- '- No commits found for this tag.\n'
  else
    write_section "Features" "${features[@]}"
    write_section "Fixes" "${fixes[@]}"
    write_section "Performance" "${performance[@]}"
    write_section "Refactors" "${refactors[@]}"
    write_section "Documentation" "${documentation[@]}"
    write_section "Tests and Tooling" "${tests_and_tooling[@]}"
    write_section "Other Changes" "${other_changes[@]}"
  fi
} > "$output_file"

printf 'Release notes written to %s\n' "$output_file"
