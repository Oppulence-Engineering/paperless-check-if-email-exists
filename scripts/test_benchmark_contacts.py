import unittest
from benchmark_contacts import report


def row(**updates):
    return {"account_id":"a1", "segment":"manufacturing", "provider":"fixture", "person_company_match":"true", "role_match":"true", "current_employment":"true", "mailbox_result":"ok", "provider_cost_usd":"0.01", "cleanup_minutes":"2", **updates}


class BenchmarkTests(unittest.TestCase):
    def test_verified_mailbox_alone_is_not_a_usable_contact(self):
        result = report([row(), row(account_id="a2", role_match="unknown"), row(account_id="a3", mailbox_result="catch_all")])["results"][0]
        self.assertEqual(result["correct_usable_contacts"], 1)
        self.assertEqual(result["cost_per_usable_contact_usd"], "0.03")
        self.assertEqual(result["decision_maker_coverage_per_100_accounts"], 33.33)

    def test_missing_cost_is_not_zero(self):
        result = report([row(provider_cost_usd="")])["results"][0]
        self.assertIsNone(result["cost_per_usable_contact_usd"])
        self.assertEqual(result["rows_missing_cost"], 1)

    def test_rejects_unequal_cohorts_and_nonfinite_cost(self):
        with self.assertRaisesRegex(ValueError, "same accounts"):
            report([row(), row(provider="other", account_id="a2")])
        for value in ("NaN", "Infinity", "-1"):
            with self.assertRaises(ValueError):
                report([row(provider_cost_usd=value)])


if __name__ == "__main__":
    unittest.main()
