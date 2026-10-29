#!/bin/sh
# Creates or updates the GitHub rulesets for the draft -> showcase flow.
#
#   tools/github-rulesets.sh [owner/repo]
#
# Defaults to wshrx/scoria. Needs a token with repository Administration write
# (`gh auth refresh -h github.com -s repo` is usually enough for a personal repo).
# It is idempotent: an existing ruleset with the same name is updated.
set -eu
repo="${1:-wshrx/scoria}"
command -v gh >/dev/null 2>&1 || { echo "gh is required" >&2; exit 1; }

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
  "name": "protect-staging",
  "target": "branch",
  "enforcement": "active",
  "bypass_actors": [],
  "conditions": {
    "ref_name": {
      "include": ["refs/heads/staging"],
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
  id=$(gh api "repos/$repo/rulesets" --jq ".[] | select(.name == \"$name\") | .id" 2>/dev/null || true)
  if [ -n "$id" ]; then
    echo "updating $name (id $id)"
    gh api --method PUT "repos/$repo/rulesets/$id" --input "$body" >/dev/null
  else
    echo "creating $name"
    gh api --method POST "repos/$repo/rulesets" --input "$body" >/dev/null
  fi
}

apply protect-main "$tmp/main.json"
apply protect-staging "$tmp/staging.json"
apply protect-release-tags "$tmp/tags.json"

echo "Rulesets are active on $repo."
echo "If the status checks are not reported yet, push staging once; the names come from public-ci.yml."
