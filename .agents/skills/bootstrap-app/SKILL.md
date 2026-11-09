---
name: bootstrap-app
description: Turn a fresh copy of the pkgforge-apps template into a new app. Renames the crate, designs the icon, personalizes the docs, fills in AGENTS.md and prepares the GitHub repository for releases. Use when starting a new app from the template, or when scripts/bootstrap.sh still exists in the repository.
---

# Bootstrap a new app

Run this once, right after the repository is created from
`pkgforge-apps/app-template`. `scripts/bootstrap.sh` only exists until
then.

## 1. Settle the identity

You need three values. Ask the user for any that are not given; do not
invent them.

- **name**: crate, binary, repository and Linux app id. Lowercase
  letters, digits and dashes, such as `mdview`. It should match the
  repository name.
- **display name**: what the window title, menus and docs show, such as
  `MD View`.
- **description**: one line, no trailing period, such as
  `A fast Markdown viewer`.

## 2. Rename

```sh
scripts/bootstrap.sh <name> "<display name>" "<description>"
```

It rewrites the placeholders in the manifest, sources, packaging, onelf
recipe, README and docs, then deletes itself. Review the diff.

## 3. Icon

Design the app's own icon with the `app-icon` skill and run
`scripts/icons.sh`. Never ship the template's icon.

## 4. Docs

Personalize the site with the `app-docs` skill: palette from the new
icon, type, home page copy, guide pages.

## 5. README and AGENTS.md

Replace the template wording in the README with what the app does.
When it is an alternative to an existing app, name that app too. Keep
the install table.

Ask whether the app should ship an AppImage too; the default is no.
With no, delete the AppImage row of the README's install table and the
parts of `README.md` and `docs/guide/installation.md` between
`<!-- AppImage` and `<!-- /AppImage -->`. With yes, set `appimage = true`
in `Cargo.toml` and `appimage: true` in `docs/.vitepress/app.ts`; the
`anylinux-appimage` skill covers the rest.

In `AGENTS.md`, rewrite the `## This app` section after the
`<!-- END SHARED GUIDELINES -->` marker: what the app is, and anything
about it the shared guidelines do not cover. Never edit the marked
block; it belongs to pkgforge-apps/.github.

Then pull the current shared guidelines and skills, since the template's
copy may be behind:

```sh
sh scripts/sync-guidelines.sh
```

## 6. Verify

```sh
scripts/check.sh
(cd docs && bun install && bun run build)
cargo run -- --smoke
```

## 7. Repository settings

These change the GitHub repository. Show the commands to the user and
run them only after they confirm.

```sh
repo=pkgforge-apps/<name>

# release-plz opens its PRs with GITHUB_TOKEN, which needs this switch.
# The guidelines sync opens its PRs as pkgforge-apps-bot, which is
# installed on every repository of the organization.
gh api -X PUT "repos/$repo/actions/permissions/workflow" \
  -f default_workflow_permissions=read \
  -F can_approve_pull_request_reviews=true

# One conventional commit per pull request, titled as the PR was
# reviewed, which is what release-plz reads.
gh api -X PATCH "repos/$repo" \
  -F allow_squash_merge=true -F allow_merge_commit=false -F allow_rebase_merge=false \
  -f squash_merge_commit_title=PR_TITLE -f squash_merge_commit_message=BLANK \
  -F delete_branch_on_merge=true

# Pages deploys from the docs workflow.
gh api -X POST "repos/$repo/pages" -f build_type=workflow

gh repo edit "$repo" \
  --description "<description>" \
  --homepage "https://pkgforge-apps.github.io/<name>/"

gh label create release --repo "$repo" --color 5b4cf0 --force

# The release environment, usable by main alone, holds the key that signs
# the onelf bundle's self-updates. pkgforge-apps-bot generates the key and
# opens a pull request with its public half; it never creates this itself.
gh api -X PUT "repos/$repo/environments/release" --input - <<'EOF'
{"deployment_branch_policy": {"protected_branches": false, "custom_branch_policies": true}}
EOF
gh api -X POST "repos/$repo/environments/release/deployment-branch-policies" \
  -f name=main -f type=branch
gh workflow run provision-apps.yml -R pkgforge-apps/.github
```

If the first call fails with a 409, the organization blocks Actions from
creating pull requests. An org owner enables it under
Settings > Actions > General > Workflow permissions.

The first push to `main` opens a release PR for `0.1.0`. Merging it
releases the app; see the `release` skill.

## 8. Remove this skill

Bootstrapping happens once. Delete `.agents/skills/bootstrap-app` and
its symlink `.claude/skills/bootstrap-app` in the same change as the
rest, so later agents do not try to run it again.
