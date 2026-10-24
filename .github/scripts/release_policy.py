#!/usr/bin/env python3
"""Validate deployment sources and publish versions promoted from develop to main."""
import json
import os
import re
import subprocess
import sys
from pathlib import Path


PRERELEASE_IDENTIFIER = r"(?:0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*)"
SEMVER = re.compile(
    rf"^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)"
    rf"(?:-({PRERELEASE_IDENTIFIER}(?:\.{PRERELEASE_IDENTIFIER})*))?$"
)


def version_key(version):
    match = SEMVER.fullmatch(version) if isinstance(version, str) else None
    if not match:
        raise ValueError("backend release manifest must contain a valid semantic version")
    core = tuple(map(int, match.group(1, 2, 3)))
    prerelease = match.group(4)
    identifiers = tuple((0, int(item)) if item.isdigit() else (1, item)
                        for item in prerelease.split(".")) if prerelease else ()
    return core, prerelease is None, identifiers


def release_version(before, after):
    old, version = before.get("backend"), after.get("backend")
    old_key, new_key = version_key(old), version_key(version)
    if old == version:
        return None
    if new_key <= old_key:
        raise ValueError("release version must increase")
    return version


def promotion_matches(pr, repository, sha):
    return (
        pr.get("merged_at") is not None
        and pr.get("merge_commit_sha") == sha
        and pr["base"]["ref"] == "main"
        and pr["head"]["ref"] == "develop"
        and (pr["head"].get("repo") or {}).get("full_name") == repository
    )


def deployment_target(event_name, ref, event):
    if event_name == "push":
        if ref.startswith("refs/heads/") and event.get("created"):
            return "bootstrap"
        return {"refs/heads/main": "production", "refs/heads/develop": "staging"}.get(
            ref, "tag" if ref.startswith("refs/tags/v") else "invalid")
    if event_name == "workflow_dispatch":
        target = event.get("inputs", {}).get("environment", "build-only")
        if target == "build-only":
            return target
        if (target, ref) in (("production", "refs/heads/main"), ("staging", "refs/heads/develop")):
            return target
    raise ValueError("Production requires main; staging requires develop")


def verify_promotion(repository, sha):
    prs = json.loads(subprocess.check_output(
        ["gh", "api", f"repos/{repository}/commits/{sha}/pulls?per_page=100"], text=True))
    if not any(promotion_matches(pr, repository, sha) for pr in prs):
        raise ValueError("main commit must be the merge of this repository's develop PR")


def validate_deployment():
    event = json.loads(Path(os.environ["GITHUB_EVENT_PATH"]).read_text())
    event_name = os.environ["GITHUB_EVENT_NAME"]
    target = deployment_target(event_name, os.environ["GITHUB_REF"], event)
    if target in ("bootstrap", "invalid"):
        raise ValueError("Branch creation and unrelated branches do not deploy")
    if target == "production" and event_name == "push":
        verify_promotion(os.environ["GITHUB_REPOSITORY"], os.environ["GITHUB_SHA"])
    if target == "tag":
        subprocess.run(["git", "merge-base", "--is-ancestor", os.environ["GITHUB_SHA"],
                        "refs/remotes/origin/main"], check=True)
    print(f"Validated {target} source")


def complete_preparation_prs(repository, sha):
    """Unblock release-please only for preparation commits included in this release."""
    prs = json.loads(subprocess.check_output(
        ["gh", "pr", "list", "--repo", repository, "--base", "develop", "--state", "merged",
         "--label", "autorelease: pending", "--limit", "100", "--json", "number,headRefName,mergeCommit"],
        text=True))
    for pr in prs:
        if not pr["headRefName"].startswith("release-please--branches--"):
            continue
        commit = (pr.get("mergeCommit") or {}).get("oid")
        if not commit:
            continue
        included = subprocess.run(["git", "merge-base", "--is-ancestor", commit, sha],
                                  capture_output=True, text=True)
        if included.returncode == 1:
            continue  # A newer preparation on develop must remain pending.
        included.check_returncode()
        subprocess.run(["gh", "pr", "edit", str(pr["number"]), "--repo", repository,
                        "--add-label", "autorelease: tagged", "--remove-label", "autorelease: pending"],
                       check=True)


def main():
    event = json.loads(Path(os.environ["GITHUB_EVENT_PATH"]).read_text())
    if os.environ.get("GITHUB_REF") != "refs/heads/main" or event.get("created"):
        raise ValueError("publish requires an existing main branch promotion")
    sha = os.environ["GITHUB_SHA"]
    repository = os.environ["GITHUB_REPOSITORY"]
    verify_promotion(repository, sha)
    before = json.loads(subprocess.check_output(
        ["git", "show", f"{event['before']}:.release-please-manifest.json"], text=True))
    after = json.loads(Path(".release-please-manifest.json").read_text())
    version = release_version(before, after)
    if version is None:
        print("Promotion has no backend version change; production deployment uses its commit SHA.")
        return
    tag = f"v{version}"
    # A retried publication may only reuse a tag for this exact release commit.
    tag_result = subprocess.run(["git", "rev-parse", "--verify", f"refs/tags/{tag}^{{commit}}"],
                                capture_output=True, text=True)
    if tag_result.returncode == 0 and tag_result.stdout.strip() != sha:
        raise ValueError(f"{tag} already points to another commit")
    existing = subprocess.run(["gh", "release", "view", tag, "--repo", repository],
                              capture_output=True, text=True)
    if existing.returncode == 0:
        if tag_result.returncode != 0:
            raise ValueError("existing release tag was not fetched; refusing ambiguous replay")
        print(f"{tag} already published at the approved commit")
        complete_preparation_prs(repository, sha)
        return
    args = ["gh", "release", "create", tag, "--repo", repository, "--target", sha,
            "--title", tag, "--generate-notes"]
    if "-" in version:
        args.append("--prerelease")
    subprocess.run(args, check=True)
    complete_preparation_prs(repository, sha)


if __name__ == "__main__":
    if sys.argv[1:] == ["validate-deployment"]:
        validate_deployment()
    elif sys.argv[1:] == ["publish"]:
        main()
    else:
        raise SystemExit("usage: release_policy.py {validate-deployment|publish}")
