import { readFileSync } from "node:fs";
import path from "node:path";

import { describe, expect, it } from "vitest";

import {
	bounceRisk,
	catchAllSeverity,
	category,
	confidenceLevel,
	freshness,
	confidenceFactor,
	humanize,
	labelFor,
	outcomeType,
	PROVIDER_LABELS,
	providerConfidence,
	reachability,
	REASON_LABELS,
	recommendation,
	SIGNALS,
	SMTP_UNCERTAINTY_LABELS,
	suppressionReason,
} from "./labels";

type Schema = { enum?: string[]; properties?: Record<string, unknown> };

const spec = JSON.parse(
	readFileSync(
		path.resolve(import.meta.dirname, "../../config/contracts/backend.openapi.json"),
		"utf8",
	),
) as { components: { schemas: Record<string, Schema> } };
const schemas = spec.components.schemas;

function enumOf(name: string): string[] {
	const values = schemas[name]?.enum;
	if (!values) throw new Error(`${name} is not an enum in the contract`);
	return values;
}

describe("verification labels", () => {
	it.each([
		["EmailCategory", category.values],
		["Reachable", reachability.values],
		["RecommendedAction", recommendation.values],
		["BounceRiskCategory", bounceRisk.values],
		["Freshness", freshness.values],
		["ConfidenceLevel", confidenceLevel.values],
		["ProviderConfidence", providerConfidence.values],
		["CatchAllSeverity", catchAllSeverity.values],
		["OutcomeType", outcomeType.values],
		["Provider", Object.keys(PROVIDER_LABELS)],
		["SmtpUncertaintyClass", Object.keys(SMTP_UNCERTAINTY_LABELS)],
		["ReasonCode", Object.keys(REASON_LABELS)],
		["SubReason", Object.keys(REASON_LABELS)],
		["SuppressionReason", suppressionReason.values],
	])("labels every %s value", (name, labelled) => {
		expect(enumOf(name).filter((value) => !labelled.includes(value))).toEqual([]);
	});

	it("covers every boolean scoring signal", () => {
		const signals = Object.keys(schemas.ScoringSignals.properties ?? {}).filter(
			(key) => key !== "reachable",
		);
		expect(signals.filter((key) => !SIGNALS.some((signal) => signal.key === key))).toEqual([]);
	});

	it("falls back to readable text for unknown values", () => {
		expect(humanize("smtp_is_catch_all")).toBe("Smtp is catch all");
		expect(confidenceFactor("provider_reputation:gmail")).toBe("Provider reputation: gmail");
		expect(confidenceFactor("tenant_history:safe_repeat")).toBe("Your earlier checks: safe repeat");
		expect(confidenceFactor("hard_invalid_signal")).toBe("Hard invalid signal");
		expect(labelFor(REASON_LABELS, "brand_new_code")).toBe("Brand new code");
		expect(labelFor(REASON_LABELS, null)).toBe("Not available");
		expect(category.tone("brand_new")).toBe("gray");
		expect(category.label("valid")).toBe("Valid");
		expect(recommendation.tone("do_not_send")).toBe("red");
	});
});
