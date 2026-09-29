# Connecting your ESP to the outcome feedback loop

This guide walks you through wiring your email service provider (SendGrid, Postmark, Mailgun, AWS SES, or anything else that emits delivery webhooks) into Reacher's outcome feedback loop. By the end, every hard bounce, complaint, or engagement event your campaigns produce will flow into Reacher and shape future verifications of those addresses.

**Time to first event flowing:** 5–15 minutes for the recommended path. CSV backfill of historical data takes longer depending on file size.

---

## Why this matters

Email verification is a probabilistic exercise — we probe an SMTP server and infer whether mail will deliver. A real bounce reported by your ESP is **ground truth**, the strongest possible signal. Once Reacher knows that `bob@acme.com` actually bounced, every future verification of `bob@acme.com` returns `category: "invalid"` with confidence 95, regardless of what the SMTP probe says today. A real `click` event boosts the score because it proves a human is on the other end. The platform learns from your sends.

You don't have to send us anything for verification to work. But every event you do send compounds the value of every verification that follows.

---

## Prerequisites

- A Reacher tenant with API access (you already have this if you're verifying emails today)
- Admin access to your ESP's webhook configuration
- An HTTPS endpoint somewhere in your stack that can receive ESP webhooks (or use Zapier/Make/n8n — covered below)

---

## Step 1 — Provision an API key with the new scope

Outcome ingest requires a separate scope from verification. This lets you keep your existing verification keys untouched.

```bash
# Patch an existing key to add outcomes.write
curl -X PATCH https://api.reacher.email/v1/me/api-keys/$KEY_ID \
  -H "Authorization: Bearer $YOUR_EXISTING_FULL_ACCESS_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scopes": ["bulk", "lists", "outcomes.write"]}'
```

Or create a brand-new key dedicated to outcome ingest:

```bash
curl -X POST https://api.reacher.email/v1/me/api-keys \
  -H "Authorization: Bearer $YOUR_EXISTING_FULL_ACCESS_KEY" \
  -H "Content-Type: application/json" \
  -d '{"name":"esp-outcomes","scopes":["outcomes.write"]}'
# Response: { "key": "rch_live_xxx...", "id": "..." }
```

Save `rch_live_…` somewhere your ESP webhook handler can read it (env var, secret manager). Add `outcomes.read` too if you want a read-only key for dashboards or audits.

---

## Step 2 — Choose your ingestion path

| Path | Use when | Latency | Effort |
|---|---|---|---|
| **Direct API push** (recommended) | You already have a backend that receives ESP webhooks | Real-time | ~10 lines of code |
| **Workflow tool** (Zapier / Make / n8n) | You have no backend or want zero code | Near real-time (seconds) | ~10 minutes of clicking |
| **CSV backfill** | You have historical bounce data to import | Batch | One-time per file |

You can mix all three — for example, CSV-import historical bounces today, then point your live webhook at the API push path for new events.

---

## Step 3 — The normalized payload shape

Every outcome you send takes this shape:

```json
{
  "outcomes": [
    {
      "email": "user@example.com",
      "type": "hard_bounce",
      "occurred_at": "2026-05-10T12:00:00Z",
      "source": "sendgrid",
      "campaign_id": "camp_42",
      "metadata": { "anything_extra": "you want to record" }
    }
  ]
}
```

| Field | Required | Description |
|---|---|---|
| `email` | yes | The recipient address. Reacher canonicalizes it (Gmail dot/plus stripped, all providers lowercased) before storage. |
| `type` | yes | One of `delivered`, `hard_bounce`, `soft_bounce`, `complaint`, `open`, `click`, `unsubscribe`. |
| `occurred_at` | yes | RFC 3339 / ISO 8601 timestamp from the ESP. |
| `source` | no but recommended | Free-form ESP/system identifier (`sendgrid`, `postmark`, `mailgun`, `ses`, `internal-job-runner`). |
| `campaign_id` | no | Your internal campaign or message identifier. Indexed for filter queries. |
| `metadata` | no | Arbitrary JSON for your own debugging / analytics. |

**Idempotency.** Reacher dedupes on `(tenant, canonical_email, type, occurred_at, source)`. Your handler can re-send the same event safely — duplicates are silently ignored. Critically, this lets you retry on transient failures without worrying about double-counting.

**Batching.** Up to 5,000 outcomes per request. We strongly recommend you batch — 1 request with 100 events is dramatically cheaper than 100 requests with 1 event.

---

## Step 4 — Per-ESP recipes

Pick the ESP you use. Each recipe is the **mapping table** plus a **complete reference handler** in Node.js. Translating to Python/Go/Ruby is mechanical.

### SendGrid

SendGrid POSTs an array of events to a single endpoint you configure under **Settings → Mail Settings → Event Webhook**. Set the HTTP Post URL to your handler at `https://your-app.com/webhooks/sendgrid` and enable the events `Delivered`, `Bounced`, `Dropped`, `Spam Reports`, `Opened`, `Clicked`, `Unsubscribes`.

**Mapping:**

| SendGrid `event` | Reacher `type` |
|---|---|
| `delivered` | `delivered` |
| `bounce` (with `type: "bounce"`) | `hard_bounce` |
| `bounce` (with `type: "blocked"` or transient) | `soft_bounce` |
| `dropped` | `hard_bounce` |
| `spamreport` | `complaint` |
| `unsubscribe`, `group_unsubscribe` | `unsubscribe` |
| `open` | `open` |
| `click` | `click` |

```js
// Node.js / Express
import express from 'express';
const app = express();
app.use(express.json());

const REACHER_API_KEY = process.env.REACHER_API_KEY; // rch_live_...
const REACHER_API = 'https://api.reacher.email';

const SENDGRID_TO_REACHER = {
  delivered: 'delivered',
  bounce: (e) => (e.type === 'bounce' ? 'hard_bounce' : 'soft_bounce'),
  dropped: 'hard_bounce',
  spamreport: 'complaint',
  unsubscribe: 'unsubscribe',
  group_unsubscribe: 'unsubscribe',
  open: 'open',
  click: 'click',
};

app.post('/webhooks/sendgrid', async (req, res) => {
  const events = Array.isArray(req.body) ? req.body : [req.body];

  const outcomes = events
    .map((e) => {
      const mapper = SENDGRID_TO_REACHER[e.event];
      const type = typeof mapper === 'function' ? mapper(e) : mapper;
      if (!type) return null;
      return {
        email: e.email,
        type,
        occurred_at: new Date(e.timestamp * 1000).toISOString(),
        source: 'sendgrid',
        campaign_id: e.sg_message_id,
      };
    })
    .filter(Boolean);

  if (outcomes.length === 0) return res.sendStatus(200);

  const r = await fetch(`${REACHER_API}/v1/outcomes`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${REACHER_API_KEY}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ outcomes }),
  });
  if (!r.ok) console.error('reacher ingest failed', await r.text());
  res.sendStatus(200);
});
```

### Postmark

Configure under **Servers → \[your server\] → Webhooks → Add Webhook**. Use one webhook per event type or a single combined endpoint. Recommended events: `Delivery`, `Bounce`, `SpamComplaint`, `Open`, `Click`, `SubscriptionChange`.

**Mapping:**

| Postmark `RecordType` | Reacher `type` |
|---|---|
| `Delivery` | `delivered` |
| `Bounce` (with `Type: "HardBounce"`) | `hard_bounce` |
| `Bounce` (with `Type: "SoftBounce"`/`Transient`) | `soft_bounce` |
| `SpamComplaint` | `complaint` |
| `SubscriptionChange` (`SuppressSending: true`) | `unsubscribe` |
| `Open` (first open per recipient — Postmark dedupes) | `open` |
| `Click` | `click` |

```js
app.post('/webhooks/postmark', async (req, res) => {
  const e = req.body;
  let type;
  switch (e.RecordType) {
    case 'Delivery':       type = 'delivered'; break;
    case 'Bounce':         type = e.Type === 'HardBounce' ? 'hard_bounce' : 'soft_bounce'; break;
    case 'SpamComplaint':  type = 'complaint'; break;
    case 'SubscriptionChange':
      type = e.SuppressSending ? 'unsubscribe' : null; break;
    case 'Open':  type = 'open'; break;
    case 'Click': type = 'click'; break;
  }
  if (!type) return res.sendStatus(200);

  const outcome = {
    email: e.Recipient || e.Email,
    type,
    occurred_at: e.ReceivedAt || e.DeliveredAt || e.BouncedAt || new Date().toISOString(),
    source: 'postmark',
    campaign_id: e.MessageID,
  };
  await fetch(`${REACHER_API}/v1/outcomes`, {
    method: 'POST',
    headers: { 'Authorization': `Bearer ${REACHER_API_KEY}`, 'Content-Type': 'application/json' },
    body: JSON.stringify({ outcomes: [outcome] }),
  });
  res.sendStatus(200);
});
```

### Mailgun

Configure under **Sending → Webhooks**. Mailgun POSTs one event per request with a verification signature in the body — verify it before processing.

**Mapping:**

| Mailgun `event` | Reacher `type` |
|---|---|
| `delivered` | `delivered` |
| `failed` (with `severity: "permanent"`) | `hard_bounce` |
| `failed` (with `severity: "temporary"`) | `soft_bounce` |
| `complained` | `complaint` |
| `unsubscribed` | `unsubscribe` |
| `opened` | `open` |
| `clicked` | `click` |

```js
import crypto from 'node:crypto';

function verifyMailgunSignature(timestamp, token, signature, signingKey) {
  const expected = crypto
    .createHmac('sha256', signingKey)
    .update(timestamp + token)
    .digest('hex');
  return crypto.timingSafeEqual(Buffer.from(expected), Buffer.from(signature));
}

app.post('/webhooks/mailgun', async (req, res) => {
  const sig = req.body.signature || {};
  if (!verifyMailgunSignature(sig.timestamp, sig.token, sig.signature, process.env.MAILGUN_SIGNING_KEY)) {
    return res.sendStatus(401);
  }

  const e = req.body['event-data'];
  const map = {
    delivered: 'delivered',
    complained: 'complaint',
    unsubscribed: 'unsubscribe',
    opened: 'open',
    clicked: 'click',
  };
  let type = map[e.event];
  if (e.event === 'failed') {
    type = e.severity === 'permanent' ? 'hard_bounce' : 'soft_bounce';
  }
  if (!type) return res.sendStatus(200);

  await fetch(`${REACHER_API}/v1/outcomes`, {
    method: 'POST',
    headers: { 'Authorization': `Bearer ${REACHER_API_KEY}`, 'Content-Type': 'application/json' },
    body: JSON.stringify({
      outcomes: [{
        email: e.recipient,
        type,
        occurred_at: new Date(e.timestamp * 1000).toISOString(),
        source: 'mailgun',
        campaign_id: e.message?.headers?.['message-id'],
      }],
    }),
  });
  res.sendStatus(200);
});
```

### AWS SES (via SNS)

SES publishes notifications to SNS topics. Subscribe an HTTPS endpoint to your `Bounce`, `Complaint`, and `Delivery` topics. SNS sends a `SubscriptionConfirmation` first — auto-confirm it by GET-ing the `SubscribeURL`.

**Mapping:**

| SES notification | Reacher `type` |
|---|---|
| `Delivery` | `delivered` |
| `Bounce` (with `bounceType: "Permanent"`) | `hard_bounce` |
| `Bounce` (with `bounceType: "Transient"`) | `soft_bounce` |
| `Complaint` | `complaint` |

```js
app.post('/webhooks/ses', express.text({ type: '*/*' }), async (req, res) => {
  const sns = JSON.parse(req.body);

  // Auto-confirm subscription
  if (sns.Type === 'SubscriptionConfirmation') {
    await fetch(sns.SubscribeURL);
    return res.sendStatus(200);
  }

  const message = JSON.parse(sns.Message);
  const outcomes = [];
  const occurred_at = sns.Timestamp;

  if (message.notificationType === 'Delivery') {
    for (const recipient of message.delivery.recipients) {
      outcomes.push({ email: recipient, type: 'delivered', occurred_at, source: 'ses', campaign_id: message.mail.messageId });
    }
  } else if (message.notificationType === 'Bounce') {
    const type = message.bounce.bounceType === 'Permanent' ? 'hard_bounce' : 'soft_bounce';
    for (const r of message.bounce.bouncedRecipients) {
      outcomes.push({ email: r.emailAddress, type, occurred_at, source: 'ses', campaign_id: message.mail.messageId });
    }
  } else if (message.notificationType === 'Complaint') {
    for (const r of message.complaint.complainedRecipients) {
      outcomes.push({ email: r.emailAddress, type: 'complaint', occurred_at, source: 'ses', campaign_id: message.mail.messageId });
    }
  }

  if (outcomes.length > 0) {
    await fetch(`${REACHER_API}/v1/outcomes`, {
      method: 'POST',
      headers: { 'Authorization': `Bearer ${REACHER_API_KEY}`, 'Content-Type': 'application/json' },
      body: JSON.stringify({ outcomes }),
    });
  }
  res.sendStatus(200);
});
```

SES doesn't push opens/clicks via SNS by default — enable Configuration Sets with **Event publishing → Open/Click** if you want engagement signals.

### Generic / custom ESP

If your provider isn't listed, you only need three pieces:

1. A trigger that fires when an event happens (any ESP webhook works).
2. A mapping from their event taxonomy to ours (`delivered`, `hard_bounce`, `soft_bounce`, `complaint`, `open`, `click`, `unsubscribe`).
3. POST to `https://api.reacher.email/v1/outcomes` with the normalized payload.

When in doubt about classification: if the email server explicitly rejected the message permanently (5xx code, "no such mailbox"), use `hard_bounce`. If it was a temporary failure (4xx, mailbox full, timeout), use `soft_bounce`.

---

## Path B — Workflow tool (no code)

If you don't have a backend stack:

**Zapier example (works almost identically in Make and n8n):**

1. **Trigger:** "New event in SendGrid" (or your ESP's native trigger). Configure for the events you care about.
2. **Optional Filter step:** Only continue for `event` in `delivered, bounce, dropped, spamreport, open, click, unsubscribe`.
3. **Optional Code step (JavaScript):** Map the ESP event name → Reacher `type` per the table above. Output an object matching the normalized payload.
4. **Action: Webhooks → Custom Request:**
   - Method: `POST`
   - URL: `https://api.reacher.email/v1/outcomes`
   - Headers: `Authorization: Bearer {{REACHER_API_KEY}}`, `Content-Type: application/json`
   - Body: `{"outcomes": [{...your mapped outcome...}]}`

For volume above ~10 events per second, the API-push path is more efficient because Zapier charges per task and doesn't batch.

---

## Path C — CSV backfill

Use this once at onboarding to import historical bounce/complaint data so the scorer benefits from it immediately.

**Export from your ESP:**
- **SendGrid:** Suppressions → Bounces → Export. Add complaint/unsubscribe exports too.
- **Postmark:** Servers → Suppressions → Export CSV.
- **Mailgun:** Sending → Suppressions → Export.
- **AWS SES:** SES Console → Suppression list → Export.

**Reshape to our CSV columns:**

```csv
email,outcome_type,occurred_at,source,campaign_id
bob@acme.com,hard_bounce,2026-04-15T08:21:00Z,sendgrid,
alice@example.com,complaint,2026-04-12T14:10:00Z,sendgrid,
```

`source` and `campaign_id` are optional. `occurred_at` must be RFC 3339.

**Upload:**

```bash
curl -X POST https://api.reacher.email/v1/outcomes/upload \
  -H "Authorization: Bearer $REACHER_API_KEY" \
  -F file=@reshaped.csv
# → 202 { "accepted": 12480, "rejected": 3, "suppressed": 8924, "policy_id": 17, "errors": [...] }
```

Rows with malformed `occurred_at`, missing `email`, or unknown `outcome_type` are reported in `errors` but don't block the rest of the batch from succeeding.

---

## Step 5 — (Optional) Tune the policy

Every tenant gets a sensible default policy on first ingest — you don't need to touch this to get value. Skip ahead unless you want different behavior.

The default:

```json
{
  "hard_bounce":  { "action": "suppress", "score_override": "invalid" },
  "complaint":    { "action": "suppress_and_unsubscribe", "score_override": "invalid" },
  "soft_bounce":  { "action": "suppress_after", "threshold_count": 3, "threshold_window_days": 30 },
  "unsubscribe":  { "action": "suppress" },
  "delivered":    { "action": "score_boost", "boost": 5 },
  "open":         { "action": "score_boost", "boost": 3 },
  "click":        { "action": "score_boost", "boost": 8 },
  "outcome_ttl_days": 90
}
```

To override — for example, raise the soft-bounce threshold or extend the TTL:

```bash
curl -X POST https://api.reacher.email/v1/outcome-policies \
  -H "Authorization: Bearer $YOUR_FULL_ACCESS_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "production",
    "is_default": true,
    "rules": {
      "hard_bounce":  { "action": "suppress", "score_override": "invalid" },
      "complaint":    { "action": "suppress_and_unsubscribe", "score_override": "invalid" },
      "soft_bounce":  { "action": "suppress_after", "threshold_count": 5, "threshold_window_days": 60 },
      "unsubscribe":  { "action": "suppress" },
      "delivered":    { "action": "score_boost", "boost": 5 },
      "open":         { "action": "score_boost", "boost": 3 },
      "click":        { "action": "score_boost", "boost": 8 },
      "outcome_ttl_days": 180
    }
  }'
```

`is_default: true` makes this the default for every future ingest. There can only be one default per tenant; setting another to default automatically demotes the previous one. PATCH/DELETE work as expected on the same `/v1/outcome-policies/{id}` URL.

---

## Step 6 — Verify the loop is closed

After your first webhook fires:

```bash
# 1. Confirm the outcome was stored
curl "https://api.reacher.email/v1/outcomes?email=bob@acme.com" \
  -H "Authorization: Bearer $REACHER_API_KEY"
# Expect: { "outcomes": [{...}], "total": 1 }

# 2. If it was a hard bounce or complaint, confirm suppression fired
curl "https://api.reacher.email/v1/suppressions/check?email=bob@acme.com" \
  -H "Authorization: Bearer $REACHER_API_KEY"
# Expect: { "suppressed": true, "reason": "bounce" or "complaint" }

# 3. Verify the same address — outcome should override the score
curl -X POST https://api.reacher.email/v1/check_email \
  -H "Authorization: Bearer $YOUR_VERIFY_KEY" \
  -H "Content-Type: application/json" \
  -d '{"to_email": "bob@acme.com"}'
# Expect: "category": "invalid", "reason_codes" contains "outcome_hard_bounce"
```

Step 3 is the proof that the loop closed: the verifier respects the outcome regardless of what the SMTP probe says.

---

## Troubleshooting

| Symptom | Likely cause | Fix |
|---|---|---|
| `403 API key lacks required scope: outcomes.write` | Key doesn't have the new scope | PATCH the key to add `outcomes.write` (Step 1) |
| `400 max 5000 outcomes per request` | Batch too large | Split into ≤5000-event batches |
| `400 invalid outcome_type 'foo'` | ESP event name not mapped | Update the mapping table; only the 7 documented types are accepted |
| `400 outcomes array is required and must be non-empty` | Empty payload | Skip the API call when you have no outcomes (e.g. all events filtered out) |
| Same outcome counted multiple times | Different `source` strings or different `occurred_at` per retry | Idempotency key is `(tenant, email, type, occurred_at, source)` — keep these stable across retries |
| `accepted` count lower than expected | Some rows had unparseable emails | Inspect the `errors` array in the response |
| Verification of the same email still returns the old score | Outcome stored with a different canonical email | Check what `canonicalize_email` produces (Gmail dots get stripped — `j.smith@gmail.com` → `jsmith@gmail.com`) |
| `GET /v1/outcomes` returns empty even though POST said `accepted` | Listing key lacks `outcomes.read` scope | Add it, or use the same key for both |

---

## Quotas and limits

- **Per request:** 5,000 outcomes, 2 MB body
- **CSV upload:** 50 MB max file size
- **Per-tenant rate limits:** the same as your existing API quota; outcome ingest counts as 1 unit per call regardless of batch size
- **Retention:** outcomes are kept indefinitely by default. The scoring layer only consults outcomes within `outcome_ttl_days` (default 90) so older data sits in storage but doesn't affect new verifications
- **Idempotency window:** there's no time limit; sending an outcome with an `occurred_at` from years ago will dedupe against any prior ingest with the same tuple

---

## Webhook security note

Today, Reacher's `/v1/outcomes` is authenticated via your tenant API key (Bearer token). The translation layer in your stack (the handler that receives the ESP webhook and posts to us) is responsible for verifying the ESP's own signature *before* forwarding. The recipes above show this for Mailgun explicitly; SendGrid and Postmark each have analogous schemes you should implement in your webhook handler.

We're tracking native ESP webhook receivers — endpoints like `https://api.reacher.email/v1/webhooks/sendgrid/{tenant_token}` that handle signature verification on our side so you don't need a translation layer at all — in [RFC 0001](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/blob/master/docs/rfcs/0001-native-provider-webhooks-and-outcome-adapters.md). It's a planned follow-up, not shipped yet.

---

## What's next after the loop is wired

- **Inspect outcomes:** `GET /v1/outcomes` filters on `email`, `source`, `type`, `since`, `limit`, `offset`. Use this for audits and dashboards.
- **Suppression audits:** outcomes that triggered suppression appear in `v1_suppression_entries` with `source = 'outcome_feedback'` and `reason ∈ {bounce, complaint, unsubscribe}`. The existing suppression endpoints work over them.
- **Re-verification:** if you have the [delayed recheck](../README.md#delayed-recheck-for-unknown-results) feature on, addresses you've sent campaigns to will still get periodic reverification on the schedule you configure — but with much higher confidence, since the outcome data backstops the SMTP probe.
- **Dashboards:** a deliverability-trends UI built on this data is on the roadmap as backlog item #19. Until that ships, query `GET /v1/outcomes` directly and chart it in your own tooling.

If you hit anything that doesn't fit the patterns above, open an issue with your ESP name and a sample webhook payload — we'll add a recipe.
