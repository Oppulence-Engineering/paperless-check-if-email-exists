#!/usr/bin/env python3
"""Aggregate manually adjudicated, equivalent contact-research cohorts; no provider calls."""
import argparse
import csv
import json
from collections import defaultdict
from decimal import Decimal, InvalidOperation
from pathlib import Path

COLUMNS = {"account_id", "segment", "provider", "person_company_match", "role_match", "current_employment", "mailbox_result", "provider_cost_usd", "cleanup_minutes"}
FLAGS = ("person_company_match", "role_match", "current_employment")


def measure(value, field, line):
    if not value:
        return None
    try:
        number = Decimal(value)
    except InvalidOperation as error:
        raise ValueError(f"Line {line}: invalid {field}") from error
    if not number.is_finite() or number < 0:
        raise ValueError(f"Line {line}: {field} must be finite and nonnegative")
    return number


def report(rows):
    groups = defaultdict(list)
    accounts = defaultdict(set)
    segments = {}
    for line, row in enumerate(rows, 2):
        if not COLUMNS <= row.keys():
            raise ValueError("Missing required benchmark columns")
        row = {key: (row[key] or "").strip() for key in COLUMNS}
        if any(not row[key] for key in ("account_id", "segment", "provider")):
            raise ValueError(f"Line {line}: account_id, segment and provider are required")
        if any(row[key] not in {"true", "false", "unknown"} for key in FLAGS):
            raise ValueError(f"Line {line}: match judgments must be true, false or unknown")
        if row["mailbox_result"] not in {"ok", "catch_all", "invalid", "unknown", "no_match"}:
            raise ValueError(f"Line {line}: unsupported mailbox_result")
        account, segment, provider = row["account_id"], row["segment"], row["provider"]
        if account in segments and segments[account] != segment:
            raise ValueError(f"Line {line}: an account belongs to multiple segments")
        segments[account] = segment
        accounts[provider].add(account)
        row["cost"] = measure(row["provider_cost_usd"], "provider_cost_usd", line)
        row["minutes"] = measure(row["cleanup_minutes"], "cleanup_minutes", line)
        groups[(provider, segment)].append(row)
    if not groups:
        raise ValueError("The benchmark has no rows")
    cohorts = list(accounts.values())
    if any(cohort != cohorts[0] for cohort in cohorts[1:]):
        raise ValueError("Every provider must include the same accounts, including no-match rows")
    output = []
    for (provider, segment), entries in sorted(groups.items()):
        usable = [row for row in entries if all(row[key] == "true" for key in FLAGS) and row["mailbox_result"] == "ok"]
        covered = len({row["account_id"] for row in usable})
        targets = len({row["account_id"] for row in entries})
        measured_cost = sum((row["cost"] for row in entries if row["cost"] is not None), Decimal(0))
        missing_cost = sum(row["cost"] is None for row in entries)
        missing_time = sum(row["minutes"] is None for row in entries)
        output.append({
            "provider": provider, "segment": segment, "target_accounts": targets,
            "candidate_rows": len(entries), "correct_usable_contacts": len(usable),
            "accounts_with_usable_contact": covered,
            "decision_maker_coverage_per_100_accounts": round(100 * covered / targets, 2),
            "false_person_company_matches": sum(row["person_company_match"] == "false" for row in entries),
            "outdated_employment_rows": sum(row["current_employment"] == "false" for row in entries),
            "unresolved_judgment_rows": sum(any(row[key] == "unknown" for key in FLAGS) for row in entries),
            "mailbox_ok_rows": sum(row["mailbox_result"] == "ok" for row in entries),
            "catch_all_rows": sum(row["mailbox_result"] == "catch_all" for row in entries),
            "measured_provider_cost_usd": str(measured_cost), "rows_missing_cost": missing_cost,
            "cost_per_usable_contact_usd": str(measured_cost / len(usable)) if usable and not missing_cost else None,
            "cleanup_minutes": str(sum((row["minutes"] for row in entries), Decimal(0))) if not missing_time else None,
            "rows_missing_cleanup_time": missing_time,
        })
    return {"method": "Manual adjudication; identical account cohorts; costs exclude unrecorded labor and infrastructure.", "results": output}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("input", type=Path)
    args = parser.parse_args()
    try:
        with args.input.open(newline="", encoding="utf-8-sig") as source:
            result = report(list(csv.DictReader(source)))
    except (OSError, ValueError) as error:
        parser.exit(1, f"Benchmark error: {error}\n")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
