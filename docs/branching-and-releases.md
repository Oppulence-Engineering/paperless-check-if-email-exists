# Branching and releases

`develop` is the integration branch and intended repository default. `main` is
the release branch. Keep both long-lived branches; do not delete `develop` after
a release or squash/rebase a release promotion.

| Work | PR base | Result after merge |
| --- | --- | --- |
| Feature, fix, dependency update, documentation | `develop` | CI and staging deployment |
| Automated version/changelog preparation | `develop` | Prepared release; no publication |
| Release promotion, with head `develop` in this repository | `main` | Production deployment; publication when the backend version increases |

## Normal changes

```sh
git fetch origin
git switch -c feat/audit-export origin/develop
# Make changes, commit, and push the feature branch.
gh pr create --base develop
```

The `branch-policy` check uses trusted base-branch workflow code and PR metadata.
It accepts normal PRs targeting `develop`. It rejects PRs targeting `main` unless
the head is this repository's `develop` branch, including forks named `develop`.
It also rejects PRs targeting the retired `master` branch. Dependabot and
release-please explicitly target `develop`; contributors should still check the
base when opening a PR.

## Releasing

1. Merge the intended work into `develop` and verify CI and staging.
2. For a versioned release, merge the release-please version/changelog PR into
   `develop`. Preparing versions does not create a GitHub release or publish SDKs.
3. Open `gh pr create --base main --head develop --title "Release: <version or summary>"`.
4. Review the complete diff and passing checks. Merge with **Create a merge commit**
   (`gh pr merge <number> --merge`). Do not delete the head branch. A merge commit
   preserves shared ancestry so later release PRs contain only new work.
5. Check the production deployments in all regions and, for a new version, the
   GitHub release, CLI artifacts, and SDK publication jobs.

The backend workflow deploys immutable `sha-<commit>` images. On `main` pushes,
production deployment and publication verify that the pushed commit belongs to a
merged, same-repository `develop → main` PR. Creating a branch does not deploy it.
The reusable Helm workflow also enforces the branch/namespace pairing before
accessing deployment credentials.

Publication compares the backend version in `.release-please-manifest.json` with
the previous `main` commit. An unchanged version deploys the promoted commit
without creating a semantic-version release. An increased version creates `vX.Y.Z`
at that exact merge commit; a retry can reuse a release only at the same commit.
Version rollback fails publication. CLI and SDK jobs require an existing version
tag reachable from `main`; SDK checkouts use the validated tag's immutable commit,
never the default branch. The existing `RELEASE_PLEASE_TOKEN` must permit PR and
release creation, so release/tag events can start the publishing workflows.

For a failed deployment, rerun the failed workflow. The backend workflow also
allows explicit staging dispatch on `develop`, production dispatch on `main`,
and `build-only` on another branch. It rejects production dispatch from
`develop`. Make code fixes and reverts through `develop`, then promote them to
`main`; never move release tags or force-push either long-lived branch.

## Repository settings

These settings are part of the branch migration and require GitHub repository
administration access in addition to permission to push code:

- Set the default branch to `develop`.
- Protect `develop` and `main`: require PRs and the `branch-policy` status check,
  include administrators, block force pushes and deletion. Preserve any existing
  required checks and review requirements when migrating protection from `master`.
- Allow merge commits for release PRs. Where rulesets are available, require the
  merge-commit method on `main`; do not require a linear history there.
- Retarget existing ordinary PRs to `develop`. Retire `master` after the migration;
  it must no longer receive changes or run production deployment automation.

A workflow failure alone does not prevent a merge: `branch-policy` must be a
required check in branch protection. Check the repository settings if a PR can be
merged despite a failing policy check.

## Migration baseline

The one-time configuration change starts from the existing `master` release
history. `main` starts at that configured baseline. Synchronizing this history into
the existing `develop` branch preserves its independent migration-history fix
(PR #140). That development change reaches production only in a later reviewed
`develop → main` release PR. The historical `master` name is not a continuing
release path.

Run the policy regressions locally with:

```sh
python3 -m pip install PyYAML==6.0.3
python3 -m unittest discover -s .github/scripts -p 'test_*.py' -v
```
