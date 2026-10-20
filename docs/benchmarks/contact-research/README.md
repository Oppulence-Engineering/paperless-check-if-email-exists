# Contact research benchmark

Run `python3 scripts/benchmark_contacts.py docs/benchmarks/contact-research/synthetic.csv`.
The fixture is invented test data, not evidence of provider performance or pricing.
Run checks with `python3 -m unittest discover -s scripts -p test_benchmark_contacts.py`.

For the pilot, sample the same approximately 1,200 accounts across three segments
for every provider. Keep one candidate per row and one `no_match` row for accounts
with no candidates. Use stable private account IDs and identical account cohorts.
Deduplicate contacts within each provider before evaluation. Keep contact addresses
and adjudication evidence in a private tenant workspace; the aggregate input does
not need names or email addresses.

A reviewer must independently adjudicate person/company matching, current
employment and the target role as `true`, `false` or `unknown`. A verified mailbox
alone does not establish those claims. Record the actual verification result,
actual per-candidate provider expenditure (including unsuccessful research), and
manual cleanup minutes. Allocate shared account research costs once across its
candidate rows. Record true zero costs as `0`; leave unmeasured cost or time blank.
Do not treat run credit reservations as measured dollar spend.

The report refuses unequal provider cohorts. It counts a usable contact only when
all three judgments are true and the mailbox result is `ok`. It reports missing
measurements rather than treating them as zero. Cost per usable contact includes
all recorded provider costs, including failed results; labor and infrastructure
must be measured separately before setting prices or claiming gross margins.
Use multiple reviewers on a blinded sample and retain disagreement as `unknown`.

Paid datasets, competitor accounts, source licenses and pilot recruitment remain
commercial follow-up. The tool makes no provider requests and contains no contact
database or Parallel-derived shared data.
