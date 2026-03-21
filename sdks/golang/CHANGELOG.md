# Changelog

## [4.1.0](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/compare/v4.0.0...v4.1.0) (2026-03-21)


### Features

* add finder confidence explanation with pattern quality and domain signals ([ae39bc5](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/ae39bc55bb2ad5f39d74786ece943cc410ab114e))
* add POST /v1/jobs/{job_id}/retry endpoint for partial retry of failed tasks ([b12a7da](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/b12a7dafa9f45f0e3199aead5f19caf693341485))
* add pre-send audience approval checklist endpoint ([58016da](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/58016dad53d090cfb310d95c554e5d27e6c8d8db))
* add reason_codes array for expanded verification findings ([4ffdfe1](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/4ffdfe1c69aac557077e0ed9420ebfc5b1090b3b))
* add reason_codes array to expose all applicable verification findings ([b1ff51e](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/b1ff51e5a6fd4ebaf3f274f5102b3a970eddc812))
* add safe_to_send recommendation flag ([433fc00](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/433fc00750a3a99dbe55f83d21924a227fe405b5))
* add safe_to_send recommendation flag to email verification responses ([16bde57](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/16bde57186c490bc53a43543f78ebfd07998564c))
* add sandbox mode for deterministic mock verification results ([217c0f0](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/217c0f053d6411a7274246ead895e37470ab3928))
* add scheduled re-verification for stale email results ([5142c60](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/5142c60e6818394c616f2273356512b1e8300d15))
* add smart deduplication and canonicalization for list uploads ([8302ada](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/8302ada8121d4aaaefe806c0dc638574c9648bb8))
* add spam-trap and honeypot domain detection ([6477a20](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/6477a2005b2cecfe0b39a71321d4985789253c57))
* add spam-trap and honeypot domain detection ([e275340](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/e275340e38f9ccf87b46ef26e2b50f2c2338bc97))
* add workspace suppression list management ([b25eef8](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/b25eef8354e101e354ceb5e6f17052716c441fab))
* add workspace suppression list management ([1da9474](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/1da947414945d4828562ebf1a79f40244f331da1))
* approval checklist, waterfall search, backlog cleanup ([d0b451d](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/d0b451da7426932519eb81e286769d3a415e73b9))
* domain typo suggestions, catch-all severity tiers, alias normalization ([7286d91](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/7286d919f08fab704aebfc83a0c8d55304f6b017))
* domain typo suggestions, catch-all severity tiers, alias normalization ([89ad7b6](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/89ad7b6aed8ae587250d5313552ec6d48bbafc42))
* finder confidence, sandbox mode, conditional auto-suppression ([674abb3](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/674abb3072affcee68231a2343999d5330fe51fc))
* partial retry endpoint + result freshness metadata ([931e542](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/931e54296879499c4b20df7dc3fcd908bbbacc70))
* scheduled re-verification for stale email results ([d3f00e7](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/d3f00e70bafafbe67ff21d3590a92805edbd33d7))
* scheduled re-verification for stale email results ([#19](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/issues/19)) ([d3f00e7](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/d3f00e70bafafbe67ff21d3590a92805edbd33d7))
* smart deduplication and canonicalization for list uploads ([e8a128c](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/e8a128cbf04ca2e8dd00d909d43e598c3c844893))
* smart deduplication and canonicalization for list uploads ([#20](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/issues/20)) ([e8a128c](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/e8a128cbf04ca2e8dd00d909d43e598c3c844893))


### Bug Fixes

* typed approval schema, reject invalid strategy, propagate DB errors ([f5b4638](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/f5b4638b3ead3a1eed88a67eb7f33062df47a78e))

## [3.1.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/v3.0.2...v3.1.0) (2025-12-12)


### Features

* add TypeScript and Go SDK generation from OpenAPI spec ([745efbb](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/745efbbb731d27b85ef048539f716e67ce819cd4))
* add TypeScript and Go SDK generation from OpenAPI spec ([0f11b78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0f11b78fc318569f97c8f755a5ec0c7170745149))

## [0.2.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/v0.1.0...v0.2.0) (2025-12-12)


### Features

* add TypeScript and Go SDK generation from OpenAPI spec ([745efbb](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/745efbbb731d27b85ef048539f716e67ce819cd4))
* add TypeScript and Go SDK generation from OpenAPI spec ([0f11b78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0f11b78fc318569f97c8f755a5ec0c7170745149))
