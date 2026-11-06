#!/bin/sh
# Creates or updates the GitHub rulesets for the draft -> showcase flow.
#
#   GITHUB_TOKEN=ghp_... tools/github-rulesets.sh [owner/repo]
#
# The token needs repository Administration write (a classic PAT with `repo`).
# No gh CLI required: curl + jq. Idempotent: rulesets are matched by name and updated.
set -eu
repo="${1:-wshrx/scoria}"
token="${GITHUB_TOKEN:-${GH_TOKEN:-}}"
[ -n "$token" ] || { echo "Set GITHUB_TOKEN (repo administration write)." >&2; exit 1; }
command -v curl >/dev/null 2>&1 || { echo "curl is required" >&2; exit 1; }
command -v jq >/dev/null 2>&1 || { echo "jq is required" >&2; exit 1; }

api="https://api.github.com/repos/$repo"
auth="Authorization: Bearer $token"
accept="Accept: application/vnd.github+json"
version="X-GitHub-Api-Version: 2022-11-28"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

cat > "$tmp/main.json" <<'JSON'
{
  "name": "protect-main",
  "target": "branch",
  "enforcement": "active",
  "bypass_actors": [],
  "conditions": {
    "ref_name": {
      "include": ["refs/heads/main"],
      "exclude": []
    }
  },
  "rules": [
    { "type": "deletion" },
    { "type": "non_fast_forward" },
    {
      "type": "pull_request",
      "parameters": {
        "required_approving_review_count": 0,
        "dismiss_stale_reviews_on_push": false,
        "require_code_owner_review": false,
        "require_last_push_approval": false,
        "required_review_thread_resolution": false,
        "allowed_merge_methods": ["squash", "rebase", "merge"]
      }
    },
    {
      "type": "required_status_checks",
      "parameters": {
        "strict_required_status_checks_policy": true,
        "do_not_enforce_on_create": false,
        "required_status_checks": [
          { "context": "launcher UI" },
          { "context": "launcher engine" },
          { "context": "engine check" },
          { "context": "UI bundle" }
        ]
      }
    }
  ]
}
JSON

cat > "$tmp/staging.json" <<'JSON'
{
  "name": "protect-dev",
  "target": "branch",
  "enforcement": "active",
  "bypass_actors": [],
  "conditions": {
    "ref_name": {
      "include": ["refs/heads/dev"],
      "exclude": []
    }
  },
  "rules": [
    { "type": "deletion" },
    { "type": "non_fast_forward" }
  ]
}
JSON

cat > "$tmp/tags.json" <<'JSON'
{
  "name": "protect-release-tags",
  "target": "tag",
  "enforcement": "active",
  "bypass_actors": [],
  "conditions": {
    "ref_name": {
      "include": ["refs/tags/v*"],
      "exclude": []
    }
  },
  "rules": [
    { "type": "deletion" },
    { "type": "non_fast_forward" }
  ]
}
JSON

apply() {
  name=$1
  body=$2
  id=$(curl -fsS -H "$auth" -H "$accept" -H "$version" "$api/rulesets" \
    | jq -r --arg n "$name" '.[] | select(.name == $n) | .id' | head -n 1)
  if [ -n "$id" ]; then
    echo "updating $name (id $id)"
    curl -fsS -X PUT -H "$auth" -H "$accept" -H "$version" \
      --data @"$body" "$api/rulesets/$id" >/dev/null
  else
    echo "creating $name"
    curl -fsS -X POST -H "$auth" -H "$accept" -H "$version" \
      --data @"$body" "$api/rulesets" >/dev/null
  fi
}

apply protect-main "$tmp/main.json"
apply protect-dev "$tmp/staging.json"
apply protect-release-tags "$tmp/tags.json"

echo "Rulesets are active on $repo."
echo "Required checks appear after the first dev run of public-ci.yml."
