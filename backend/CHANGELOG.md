# Changelog

## [4.1.0](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/compare/v4.0.0...v4.1.0) (2026-03-21)


### Features

* add conditional auto-suppression actions after task completion ([d25e5ef](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/d25e5ef7f4cd2d45e6abdeb5bd88f473ee26e638))
* add finder confidence explanation with pattern quality and domain signals ([ae39bc5](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/ae39bc55bb2ad5f39d74786ece943cc410ab114e))
* add POST /v1/jobs/{job_id}/retry endpoint for partial retry of failed tasks ([b12a7da](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/b12a7dafa9f45f0e3199aead5f19caf693341485))
* add pre-send audience approval checklist endpoint ([58016da](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/58016dad53d090cfb310d95c554e5d27e6c8d8db))
* add reason_codes array for expanded verification findings ([4ffdfe1](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/4ffdfe1c69aac557077e0ed9420ebfc5b1090b3b))
* add reason_codes array to expose all applicable verification findings ([b1ff51e](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/b1ff51e5a6fd4ebaf3f274f5102b3a970eddc812))
* add result freshness metadata (verified_at, age_days, freshness tier) ([4332558](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/433255899dd14f9b4a5ee54551741d82418e2c34))
* add safe_to_send recommendation flag ([433fc00](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/433fc00750a3a99dbe55f83d21924a227fe405b5))
* add safe_to_send recommendation flag to email verification responses ([16bde57](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/16bde57186c490bc53a43543f78ebfd07998564c))
* add sandbox mode for deterministic mock verification results ([217c0f0](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/217c0f053d6411a7274246ead895e37470ab3928))
* add scheduled re-verification for stale email results ([5142c60](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/5142c60e6818394c616f2273356512b1e8300d15))
* add smart deduplication and canonicalization for list uploads ([8302ada](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/8302ada8121d4aaaefe806c0dc638574c9648bb8))
* add spam-trap and honeypot domain detection ([6477a20](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/6477a2005b2cecfe0b39a71321d4985789253c57))
* add spam-trap and honeypot domain detection ([e275340](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/e275340e38f9ccf87b46ef26e2b50f2c2338bc97))
* add waterfall search strategy for finder ([eac3853](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/eac38534e6d9a0dd76352222ebc39eb4c3c30125))
* add workspace suppression list management ([b25eef8](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/b25eef8354e101e354ceb5e6f17052716c441fab))
* add workspace suppression list management ([1da9474](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/1da947414945d4828562ebf1a79f40244f331da1))
* approval checklist, waterfall search, backlog cleanup ([d0b451d](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/d0b451da7426932519eb81e286769d3a415e73b9))
* domain typo suggestions, catch-all severity tiers, alias normalization ([7286d91](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/7286d919f08fab704aebfc83a0c8d55304f6b017))
* domain typo suggestions, catch-all severity tiers, alias normalization ([89ad7b6](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/89ad7b6aed8ae587250d5313552ec6d48bbafc42))
* finder confidence, sandbox mode, conditional auto-suppression ([674abb3](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/674abb3072affcee68231a2343999d5330fe51fc))
* partial retry endpoint + result freshness metadata ([931e542](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/931e54296879499c4b20df7dc3fcd908bbbacc70))
* Phase 1 multi-tenant platform with onboarding endpoint ([6a33c56](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/6a33c56d6364ce37097d7f1a928826dc02ea355c))
* scheduled re-verification for stale email results ([d3f00e7](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/d3f00e70bafafbe67ff21d3590a92805edbd33d7))
* scheduled re-verification for stale email results ([#19](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/issues/19)) ([d3f00e7](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/d3f00e70bafafbe67ff21d3590a92805edbd33d7))
* smart deduplication and canonicalization for list uploads ([e8a128c](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/e8a128cbf04ca2e8dd00d909d43e598c3c844893))
* smart deduplication and canonicalization for list uploads ([#20](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/issues/20)) ([e8a128c](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/e8a128cbf04ca2e8dd00d909d43e598c3c844893))


### Bug Fixes

* address review feedback on deduplication PR ([236f407](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/236f407be0532a27875ef775e61f6f744b1f34a0))
* address review feedback on reverification PR ([d8f736a](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/d8f736ae309ac845a38728499bbeadf34bd70000))
* atomic schedule claim, pool-only status endpoint, RFC3339 timestamps ([c2e6e05](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/c2e6e050a6b6e7b41267412abaf6daf7a7604dab))
* atomic schedule claiming, correct stale query, failed publish cleanup ([75fb30a](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/75fb30a4c2ffcf4d003e589c19f8e1605220796c))
* auto-suppress in direct check path, guard blank emails, fix md headings ([29c4ab9](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/29c4ab944225db3e29b98037cfb209179e79ce38))
* clear job completed_at on retry, keep quota check before publish ([c7cf38b](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/c7cf38b98cd450852b825c57a9c64a3c2b22a081))
* compute safe_to_send_pct from effective_safe to match recommendation ([819481d](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/819481d72b12a0a3cfed747bbee77f3f62762ea3))
* correct bind order in auto-suppression INSERT (tenant_id, email, reason) ([84d321b](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/84d321bc7dff59b571ebe2dc4e6ad173740dfd3b))
* count cancelled rows as incomplete, use unrounded ratio for thresholds ([f3dcd93](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/f3dcd9309140aaa5d5be5165ea7d115b2522d4f6))
* dynamic suppress reason, deterministic sandbox timestamps, remove _sandbox field ([1d946e3](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/1d946e318ab0626f039e0f2a7e4c9b4021d758fb))
* factor suppression into readiness, distinguish cancelled jobs, fix join ([3ffdb3e](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/3ffdb3eadb39030579083a0140cb01845ea9dc73))
* guard pagination u64→i64 casts to prevent negative SQL bounds ([8db2aa0](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/8db2aa0126637a064340161dd9403551310934dd))
* hardcode sandbox freshness fields for fully deterministic responses ([d36a6c6](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/d36a6c670156a6e0c250a5b5739f6c82abfc9a15))
* inject freshness into bulk JSON results path ([1207023](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/12070233f6990ff33e8cd33975b4825ac7487bfc))
* keep duplicates non-terminal until propagation, count cancelled tasks ([f36303c](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/f36303cae8b2ffc4a47d4bf105d7eb728c7ca69d))
* lock retryable rows with FOR UPDATE, use exact count for quota, reset by ID ([f6c7c2e](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/f6c7c2ef97bf70ef45082062d55cc1818f06e8c1))
* log settings query errors, redact PII from logs, add sandbox marker ([cbd67ac](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/cbd67ac2655d616169b6200b90ef3503ca0681e6))
* normalize email in auto-suppress, validate sandbox input, update backlog ([ae2988b](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/ae2988b1d6e35ae8bd8ae0e2fba5cb2ac75ad667))
* only subtract safe-to-send suppressed rows from readiness ratio ([3fe2301](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/3fe230198b7ed5634db0267858fea35b37cda81b))
* publish to RabbitMQ before committing DB state, clamp negative age_days ([e5ba97c](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/e5ba97cb111226859719be959bf7e450cd45beb6))
* replace sqlx query_scalar! macro with runtime version in tests ([e96b8b5](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/e96b8b5f69b95c7d1c10a93dfc4860d39e29fcfd))
* resolve CI failures — formatting and service containers ([f6b4090](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/f6b4090aa06a9d59adf9f6d1954e1128c3e09b3f))
* resolve test failures across multiple test suites ([2704bdc](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/2704bdc27dada9302af5c98be0c3a423a2458737))
* stop stripping +subaddress for non-Gmail providers ([836ced4](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/836ced427b2ee28af98f6f5726840dc3a1425c16))
* treat canonicalization failures as invalid, not billable ([ba63dd0](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/ba63dd0bd8360107ae975551cd57db11c0e589f4))
* typed approval schema, reject invalid strategy, propagate DB errors ([f5b4638](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/f5b4638b3ead3a1eed88a67eb7f33062df47a78e))
* update list CSV assertions for reason_codes column ([8b47a23](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/8b47a23ebdabeace6d8b04c7c3a1765910d7fa6b))
* validate strategy before DB work, sort candidates by priority in waterfall ([5d27a6c](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/5d27a6c914a99c5f34ddf4ea85b3f1725e51c78b))

## [4.0.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/v3.0.2...v4.0.0) (2025-12-12)


### ⚠ BREAKING CHANGES

* 
* **core:** 
* 
* **core:** 
* 
* **backend:** 
* Rename all VerifyMethod to VerifMethod
* For Hotmail, Gmail and Yahoo addresses, the `*_use_api` and `*_use_headless` parameters have been removed and replaced with a `*VerifyMethod`, an enum which can take value Api, Headless or Smtp. If using headless, pass a webdriver address to env variable RCH_WEBDRIVER_ADDR.
* `input.hotmail_use_headless` is now a bool instead of a string. Pass the webdriver address as an environment variable `RCH_WEBDRIVER_ADDR` now.
* 
* 

### Features

* **#289:** add haveibeenpwned check ([#1253](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1253)) ([166dbd2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/166dbd2cc878e30c51538b919abc1aaea4465c45))
* Add `/v1/{check_email,bulk}` endpoints with throttle&concurrency ([#1537](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1537)) ([08522e4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/08522e4326bbcbc980cf501d5d994d0c17222561))
* Add `misc.is_b2c` field ([#1553](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1553)) ([14a6759](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/14a6759d805d2051a4a1e1d81588279cb9c85336))
* Add AWS SQS support ([#1554](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1554)) ([92be54e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/92be54ebfe4a2d19101141f55e94fc8e9588ff95))
* Add back RabbitMQ-based worker ([#1513](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1513)) ([de75ece](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/de75eceef32c6ea512e0a301ec62d393bb59ff0f))
* Add debug information about each email verification ([#1391](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1391)) ([3ea6e66](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3ea6e6607735682dfca6ecfa27460650ac6e42d3))
* add email address normalisation ([#1206](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1206)) ([f8ec348](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/f8ec348883cd4f4a20a8acbb38d54b69e798222b))
* Add optional timeout on proxy (env var: `RCH__PROXY__TIMEOUT_MS`) ([#1595](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1595)) ([0e51eb6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0e51eb686dad6bd2ec827e785bf9c30ccc88cde1))
* Add RabbitMQ worker ([#1395](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1395)) ([ecef8c9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ecef8c98deb744390c7017a4e98d4f3c7e737fcb))
* Add suggestions for syntax errors ([#1192](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1192)) ([2d385f3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2d385f30f7a62ab2706599fbb89fb50275cffb5f))
* Allow /v1/check_email without worker mode ([9ca9f39](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9ca9f39ee487dc1b7d9b4cdc9a0b2c0669b10bc0))
* Allow multiple proxies ([#1562](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1562)) ([eed5a15](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/eed5a1536af37877f12eebab6481acaa6efa55c5))
* **backend:** Add header secret to protect against public requests ([#1158](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1158)) ([fa6a56b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6a56b62f4b3aeeec704cfe4882755998d40833))
* **backend:** Add one simple retry on Unknown ([fcffc1a](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fcffc1a28bab990b0596ad8b66163e47a494191b))
* **backend:** Add POST /v1/bulk ([#1413](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1413)) ([d9302d4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d9302d4c1cec6a5a1788afe2a3718df8986f118f))
* **backend:** Add reply-to queue ([aaea59f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/aaea59f251634db7c35f029b09ef6e5f8c77cfbc))
* **backend:** Add worker webhook ([db90cfa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/db90cfa27b85916685268a3599bdfdb2c46de07a))
* **backend:** Customize SMTP defaults ([8f152b8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8f152b83c70b94618b71308552a6999f4b27aa2f))
* **backend:** Prune bulk email verification database ([#1377](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1377)) ([f905735](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/f90573566abf40133ebfb28ebc8f18ad8278a9b3))
* **backend:** Reject a request with to_email field empty or missing ([#1353](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1353)) ([1d9c29f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/1d9c29f5a48655a11f985b7df91c8bcbdf102487))
* **backend:** Remove /v0/bulk endpoints ([#1421](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1421)) ([522f324](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/522f32448416cd75a70ddb51038e50d06c3130b4))
* **backend:** Support RCH_SMTP_TIMEOUT ([#1407](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1407)) ([b9bda40](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b9bda4049540372811a86d8dd7ba873c9875e54d))
* **core:** Add check gravatar image ([#1188](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1188)) ([6a26035](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6a26035327ab681a65a4f4ba284e155f00680e89))
* **core:** Add Hotmail checks via headless password recovery ([#1165](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1165)) ([7517ed9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7517ed98ba966158deebba6a1a4745c931bfed18))
* **core:** Update async-smtp to 0.9 ([#1520](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1520)) ([297ce4f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/297ce4f11994b483faa015bebe4abf550eb77e11))
* Increase content length limit for bulk validation endpoint ([#1525](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1525)) ([bbdab31](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bbdab31e0dde54d21f4eeb5880ae28e60de7dced))
* Move `backend` code to this repo ([#1138](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1138)) ([0dc6053](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0dc60531d26efb217137347ef2b6aaf678d94238))
* Revert back to `check_email` input with single email ([#1150](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1150)) ([ce1ba53](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ce1ba5346849b578a0ed30b1d72096f15cfbc09d))
* Set default timeout to 10s ([#1251](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1251)) ([d04f84c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d04f84cc1e7b30e02d3717ab1af9f680cdb2c27f))
* Yahoo account recovery via headless ([#1364](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1364)) ([6f0f12b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6f0f12b8cf528e819f8743f7e3c5f5e141c51559))


### Bug Fixes

* Add "utilisateur inconnu" in invalid parser ([#1594](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1594)) ([fb91653](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb9165303e2d7be59ed2fa4f0682e8592bc0c5e7))
* Add backend_name in /v0/check_email ([a738fae](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/a738faec99942d20b817298f7850e84ab3e74835))
* **backend:** CSV download retrieves all results ([#1362](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1362)) ([b3670fc](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b3670fcaebce05a0aab09bcc3253134cb3c643c1))
* **backend:** Fix docker CTRL+C ([3a7245f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3a7245f9a47e8332d682d437d9492559e5adf66f))
* **backend:** Fix dockerfile ([f0ed49f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/f0ed49f50238c1c71a130f3db19ec047af00b8df))
* **backend:** Fix env var for multiple queues ([ed19166](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ed191662b18c62f397b4fed6b95249b5aa76c423))
* **backend:** Improve sentry error messages ([#1155](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1155)) ([d90d998](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d90d998d1cb189fed3f888659aa08fd4fabf6e93))
* **backend:** Redact email in sentry bug tracking ([2c2d1d8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2c2d1d88c0086196bc09359e32c96638124d9539))
* **backend:** Update sqlx to 0.7 ([#1390](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1390)) ([7198f87](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7198f87de92ab403cdc1e7c68667cdef9db96085))
* Bring back `{yahoo,hotmailb2c}_verif_method` ([#1606](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1606)) ([3fbe520](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3fbe5200a3d8608fbd72c0f2a5917326c1f8ec91))
* **core:** Clean up CheckEmailInput ([#1531](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1531)) ([b97b9ff](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b97b9ff9b91bdfbf18e5c0892559e87e7cd5e16c))
* **core:** Fix default CheckEmailInput ([09215a1](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/09215a13ac3525861e6cd1dea3fc71c13dfffe52))
* **core:** Fix MX random record selection ([#1263](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1263)) ([9fae593](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9fae593b8590ad5efb3e7d16bbd25cc05c228cb9))
* **core:** Headless check for Microsoft365 too ([#1346](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1346)) ([682cc2d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/682cc2d96b93d73f3fca3ba11f03800477c8fb9e))
* **core:** Improve invalid parser ([#1166](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1166)) ([bb46004](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bb460046bf1cb031fee706d836c8a737157f803c))
* **core:** Use Smtp for Gmail by default ([8e79884](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8e79884314f0c1eec5a7964fa686e2c60e7d2209))
* **core:** Use tagged enum representation ([ffde851](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ffde851068798adc3372d843a916a121b5caeccb))
* **docker:** Fix dockerfile entrypoint ([d1d3326](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d1d3326af88a85b2192796d8d2c92ff854b5644d))
* Fix dockerfile ([ce5067e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ce5067e4050e0cf3fa6c022bc7e25e5f15261c2a))
* Fix rabbitmq docker compose ([7c3856e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7c3856ebec6089b37b3dd30e3c4f13df9fb4e73a))
* Fix version in logs ([fa6be78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6be7867abae981b0d82fde24e0310b9759ab1f))
* Improve logging, add retries for Yahoo headless, switch to rustls ([#1549](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1549)) ([b1377db](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b1377db2b32155d766a09a76864fc9b0990833e6))
* initialize crypto provider at startup for TLS connections ([78e0461](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/78e04614673ffb52582ac7c70b4fc3fbc508b3ef))
* Make new config backwards-compatible ([#1567](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1567)) ([b824e2c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b824e2c988ee4eef021b97fc65ebcfa36a166d7f))
* Put Smtp debug details in Debug struct ([5b71ca5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5b71ca59b6fab18263348aeafc7a895b7f4b8076))
* Reinstate proxy in JSON request ([#1569](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1569)) ([c36e6e0](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/c36e6e09c9079de210d288b84d79b984e2ea77f0))
* Remove local_ip retrieval ([ff8e599](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ff8e5998f8b88954b4104f9251d1331542dbb182))
* Remove max requests per minute/day ([07a6d96](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/07a6d96416f52ac0824e7e7ac665fd2169ddc7ec))
* Revert back to using lowest-priority MX record ([#1578](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1578)) ([60468b3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/60468b3f533491a0dff6a42e7096f34ece19896c))
* Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579)) ([3388163](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3388163d03b66ba92455be8404441e8555a9d53c))
* Support queues in env var ([39655d5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/39655d51afe5f65d62cd5dc3485586e16bcdec31))
* Typo in expect of RCH_VERIF_METHOD ([#1405](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1405)) ([c50d8eb](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/c50d8ebdfc470fe1ec6290e07668c70095298799))
* Use chromedriver instead of gecko for parallel requests ([e282e28](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/e282e28aeb7259d800f7faad97173c3a216095a4))
* use schema-qualified public.uuid_nil() to fix PostgreSQL function inlining issue ([385ec52](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/385ec52a6e1eac0eb1c360986905c478fe2cb725))


### Reverts

* "Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579))" ([56e7838](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/56e7838f28067b05b58f1fcd166368a915aafbbc))
* **backend:** Bring back the sqlxmq-based bulk verification ([#1477](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1477)) ([322ad4e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/322ad4e4b53d534a8ae6461f3d3383d67b219b5d))


### Miscellaneous Chores

* Rename all VerifyMethod to VerifMethod ([9f9607d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9f9607d35478a1051dde56812f8914ff75d4c5ac))


### Code Refactoring

* Change RUST_LOG target to `reacher` ([#1152](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1152)) ([7e87be2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7e87be26f1e35a6936bfc967c872cd42b93fd256))
* Use config-rs instead of env vars ([#1530](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1530)) ([bcd2dc8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bcd2dc867b7dc2bdaeb70097fd14109c2a40da17))
* Use verify method for known providers ([#1366](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1366)) ([5ca4dfa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5ca4dfa5ec38fba0ec7cfb052106da8d6af4df44))

## [3.0.1](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/v3.0.0...v3.0.1) (2025-12-04)


### Bug Fixes

* initialize crypto provider at startup for TLS connections ([78e0461](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/78e04614673ffb52582ac7c70b4fc3fbc508b3ef))
* use schema-qualified public.uuid_nil() to fix PostgreSQL function inlining issue ([385ec52](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/385ec52a6e1eac0eb1c360986905c478fe2cb725))

## [3.0.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/v2.0.0...v3.0.0) (2025-11-28)


### ⚠ BREAKING CHANGES

* 
* **core:** 
* 
* **core:** 
* 
* **backend:** 
* Rename all VerifyMethod to VerifMethod
* For Hotmail, Gmail and Yahoo addresses, the `*_use_api` and `*_use_headless` parameters have been removed and replaced with a `*VerifyMethod`, an enum which can take value Api, Headless or Smtp. If using headless, pass a webdriver address to env variable RCH_WEBDRIVER_ADDR.
* `input.hotmail_use_headless` is now a bool instead of a string. Pass the webdriver address as an environment variable `RCH_WEBDRIVER_ADDR` now.
* 
* 

### Features

* **#289:** add haveibeenpwned check ([#1253](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1253)) ([166dbd2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/166dbd2cc878e30c51538b919abc1aaea4465c45))
* Add `/v1/{check_email,bulk}` endpoints with throttle&concurrency ([#1537](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1537)) ([08522e4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/08522e4326bbcbc980cf501d5d994d0c17222561))
* Add `misc.is_b2c` field ([#1553](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1553)) ([14a6759](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/14a6759d805d2051a4a1e1d81588279cb9c85336))
* Add AWS SQS support ([#1554](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1554)) ([92be54e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/92be54ebfe4a2d19101141f55e94fc8e9588ff95))
* Add back RabbitMQ-based worker ([#1513](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1513)) ([de75ece](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/de75eceef32c6ea512e0a301ec62d393bb59ff0f))
* Add debug information about each email verification ([#1391](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1391)) ([3ea6e66](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3ea6e6607735682dfca6ecfa27460650ac6e42d3))
* add email address normalisation ([#1206](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1206)) ([f8ec348](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/f8ec348883cd4f4a20a8acbb38d54b69e798222b))
* Add optional timeout on proxy (env var: `RCH__PROXY__TIMEOUT_MS`) ([#1595](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1595)) ([0e51eb6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0e51eb686dad6bd2ec827e785bf9c30ccc88cde1))
* Add RabbitMQ worker ([#1395](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1395)) ([ecef8c9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ecef8c98deb744390c7017a4e98d4f3c7e737fcb))
* Add suggestions for syntax errors ([#1192](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1192)) ([2d385f3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2d385f30f7a62ab2706599fbb89fb50275cffb5f))
* Allow /v1/check_email without worker mode ([9ca9f39](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9ca9f39ee487dc1b7d9b4cdc9a0b2c0669b10bc0))
* Allow multiple proxies ([#1562](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1562)) ([eed5a15](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/eed5a1536af37877f12eebab6481acaa6efa55c5))
* **backend:** Add header secret to protect against public requests ([#1158](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1158)) ([fa6a56b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6a56b62f4b3aeeec704cfe4882755998d40833))
* **backend:** Add one simple retry on Unknown ([fcffc1a](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fcffc1a28bab990b0596ad8b66163e47a494191b))
* **backend:** Add POST /v1/bulk ([#1413](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1413)) ([d9302d4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d9302d4c1cec6a5a1788afe2a3718df8986f118f))
* **backend:** Add reply-to queue ([aaea59f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/aaea59f251634db7c35f029b09ef6e5f8c77cfbc))
* **backend:** Add worker webhook ([db90cfa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/db90cfa27b85916685268a3599bdfdb2c46de07a))
* **backend:** Customize SMTP defaults ([8f152b8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8f152b83c70b94618b71308552a6999f4b27aa2f))
* **backend:** Prune bulk email verification database ([#1377](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1377)) ([f905735](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/f90573566abf40133ebfb28ebc8f18ad8278a9b3))
* **backend:** Reject a request with to_email field empty or missing ([#1353](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1353)) ([1d9c29f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/1d9c29f5a48655a11f985b7df91c8bcbdf102487))
* **backend:** Remove /v0/bulk endpoints ([#1421](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1421)) ([522f324](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/522f32448416cd75a70ddb51038e50d06c3130b4))
* **backend:** Support RCH_SMTP_TIMEOUT ([#1407](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1407)) ([b9bda40](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b9bda4049540372811a86d8dd7ba873c9875e54d))
* **core:** Add check gravatar image ([#1188](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1188)) ([6a26035](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6a26035327ab681a65a4f4ba284e155f00680e89))
* **core:** Add Hotmail checks via headless password recovery ([#1165](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1165)) ([7517ed9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7517ed98ba966158deebba6a1a4745c931bfed18))
* **core:** Update async-smtp to 0.9 ([#1520](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1520)) ([297ce4f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/297ce4f11994b483faa015bebe4abf550eb77e11))
* Increase content length limit for bulk validation endpoint ([#1525](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1525)) ([bbdab31](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bbdab31e0dde54d21f4eeb5880ae28e60de7dced))
* Move `backend` code to this repo ([#1138](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1138)) ([0dc6053](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0dc60531d26efb217137347ef2b6aaf678d94238))
* Revert back to `check_email` input with single email ([#1150](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1150)) ([ce1ba53](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ce1ba5346849b578a0ed30b1d72096f15cfbc09d))
* Set default timeout to 10s ([#1251](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1251)) ([d04f84c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d04f84cc1e7b30e02d3717ab1af9f680cdb2c27f))
* Yahoo account recovery via headless ([#1364](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1364)) ([6f0f12b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6f0f12b8cf528e819f8743f7e3c5f5e141c51559))


### Bug Fixes

* Add "utilisateur inconnu" in invalid parser ([#1594](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1594)) ([fb91653](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb9165303e2d7be59ed2fa4f0682e8592bc0c5e7))
* Add backend_name in /v0/check_email ([a738fae](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/a738faec99942d20b817298f7850e84ab3e74835))
* **backend:** CSV download retrieves all results ([#1362](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1362)) ([b3670fc](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b3670fcaebce05a0aab09bcc3253134cb3c643c1))
* **backend:** Fix docker CTRL+C ([3a7245f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3a7245f9a47e8332d682d437d9492559e5adf66f))
* **backend:** Fix dockerfile ([f0ed49f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/f0ed49f50238c1c71a130f3db19ec047af00b8df))
* **backend:** Fix env var for multiple queues ([ed19166](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ed191662b18c62f397b4fed6b95249b5aa76c423))
* **backend:** Improve sentry error messages ([#1155](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1155)) ([d90d998](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d90d998d1cb189fed3f888659aa08fd4fabf6e93))
* **backend:** Redact email in sentry bug tracking ([2c2d1d8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2c2d1d88c0086196bc09359e32c96638124d9539))
* **backend:** Update sqlx to 0.7 ([#1390](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1390)) ([7198f87](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7198f87de92ab403cdc1e7c68667cdef9db96085))
* Bring back `{yahoo,hotmailb2c}_verif_method` ([#1606](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1606)) ([3fbe520](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3fbe5200a3d8608fbd72c0f2a5917326c1f8ec91))
* **core:** Clean up CheckEmailInput ([#1531](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1531)) ([b97b9ff](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b97b9ff9b91bdfbf18e5c0892559e87e7cd5e16c))
* **core:** Fix default CheckEmailInput ([09215a1](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/09215a13ac3525861e6cd1dea3fc71c13dfffe52))
* **core:** Fix MX random record selection ([#1263](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1263)) ([9fae593](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9fae593b8590ad5efb3e7d16bbd25cc05c228cb9))
* **core:** Headless check for Microsoft365 too ([#1346](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1346)) ([682cc2d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/682cc2d96b93d73f3fca3ba11f03800477c8fb9e))
* **core:** Improve invalid parser ([#1166](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1166)) ([bb46004](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bb460046bf1cb031fee706d836c8a737157f803c))
* **core:** Use Smtp for Gmail by default ([8e79884](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8e79884314f0c1eec5a7964fa686e2c60e7d2209))
* **core:** Use tagged enum representation ([ffde851](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ffde851068798adc3372d843a916a121b5caeccb))
* **docker:** Fix dockerfile entrypoint ([d1d3326](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d1d3326af88a85b2192796d8d2c92ff854b5644d))
* Fix dockerfile ([ce5067e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ce5067e4050e0cf3fa6c022bc7e25e5f15261c2a))
* Fix rabbitmq docker compose ([7c3856e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7c3856ebec6089b37b3dd30e3c4f13df9fb4e73a))
* Fix version in logs ([fa6be78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6be7867abae981b0d82fde24e0310b9759ab1f))
* Improve logging, add retries for Yahoo headless, switch to rustls ([#1549](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1549)) ([b1377db](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b1377db2b32155d766a09a76864fc9b0990833e6))
* Make new config backwards-compatible ([#1567](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1567)) ([b824e2c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b824e2c988ee4eef021b97fc65ebcfa36a166d7f))
* Put Smtp debug details in Debug struct ([5b71ca5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5b71ca59b6fab18263348aeafc7a895b7f4b8076))
* Reinstate proxy in JSON request ([#1569](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1569)) ([c36e6e0](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/c36e6e09c9079de210d288b84d79b984e2ea77f0))
* Remove local_ip retrieval ([ff8e599](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ff8e5998f8b88954b4104f9251d1331542dbb182))
* Remove max requests per minute/day ([07a6d96](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/07a6d96416f52ac0824e7e7ac665fd2169ddc7ec))
* Revert back to using lowest-priority MX record ([#1578](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1578)) ([60468b3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/60468b3f533491a0dff6a42e7096f34ece19896c))
* Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579)) ([3388163](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3388163d03b66ba92455be8404441e8555a9d53c))
* Support queues in env var ([39655d5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/39655d51afe5f65d62cd5dc3485586e16bcdec31))
* Typo in expect of RCH_VERIF_METHOD ([#1405](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1405)) ([c50d8eb](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/c50d8ebdfc470fe1ec6290e07668c70095298799))
* Use chromedriver instead of gecko for parallel requests ([e282e28](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/e282e28aeb7259d800f7faad97173c3a216095a4))


### Reverts

* "Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579))" ([56e7838](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/56e7838f28067b05b58f1fcd166368a915aafbbc))
* **backend:** Bring back the sqlxmq-based bulk verification ([#1477](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1477)) ([322ad4e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/322ad4e4b53d534a8ae6461f3d3383d67b219b5d))


### Miscellaneous Chores

* Rename all VerifyMethod to VerifMethod ([9f9607d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9f9607d35478a1051dde56812f8914ff75d4c5ac))


### Code Refactoring

* Change RUST_LOG target to `reacher` ([#1152](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1152)) ([7e87be2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7e87be26f1e35a6936bfc967c872cd42b93fd256))
* Use config-rs instead of env vars ([#1530](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1530)) ([bcd2dc8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bcd2dc867b7dc2bdaeb70097fd14109c2a40da17))
* Use verify method for known providers ([#1366](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1366)) ([5ca4dfa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5ca4dfa5ec38fba0ec7cfb052106da8d6af4df44))

## [2.0.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/backend-v1.0.0...backend-v2.0.0) (2025-11-28)


### ⚠ BREAKING CHANGES

* 
* **core:** 
* 
* **core:** 
* 
* **backend:** 
* Rename all VerifyMethod to VerifMethod
* For Hotmail, Gmail and Yahoo addresses, the `*_use_api` and `*_use_headless` parameters have been removed and replaced with a `*VerifyMethod`, an enum which can take value Api, Headless or Smtp. If using headless, pass a webdriver address to env variable RCH_WEBDRIVER_ADDR.
* `input.hotmail_use_headless` is now a bool instead of a string. Pass the webdriver address as an environment variable `RCH_WEBDRIVER_ADDR` now.
* 
* 

### Features

* **#289:** add haveibeenpwned check ([#1253](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1253)) ([166dbd2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/166dbd2cc878e30c51538b919abc1aaea4465c45))
* Add `/v1/{check_email,bulk}` endpoints with throttle&concurrency ([#1537](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1537)) ([08522e4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/08522e4326bbcbc980cf501d5d994d0c17222561))
* Add `misc.is_b2c` field ([#1553](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1553)) ([14a6759](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/14a6759d805d2051a4a1e1d81588279cb9c85336))
* Add AWS SQS support ([#1554](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1554)) ([92be54e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/92be54ebfe4a2d19101141f55e94fc8e9588ff95))
* Add back RabbitMQ-based worker ([#1513](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1513)) ([de75ece](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/de75eceef32c6ea512e0a301ec62d393bb59ff0f))
* Add debug information about each email verification ([#1391](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1391)) ([3ea6e66](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3ea6e6607735682dfca6ecfa27460650ac6e42d3))
* add email address normalisation ([#1206](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1206)) ([f8ec348](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/f8ec348883cd4f4a20a8acbb38d54b69e798222b))
* Add optional timeout on proxy (env var: `RCH__PROXY__TIMEOUT_MS`) ([#1595](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1595)) ([0e51eb6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0e51eb686dad6bd2ec827e785bf9c30ccc88cde1))
* Add RabbitMQ worker ([#1395](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1395)) ([ecef8c9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ecef8c98deb744390c7017a4e98d4f3c7e737fcb))
* Add suggestions for syntax errors ([#1192](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1192)) ([2d385f3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2d385f30f7a62ab2706599fbb89fb50275cffb5f))
* Allow /v1/check_email without worker mode ([9ca9f39](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9ca9f39ee487dc1b7d9b4cdc9a0b2c0669b10bc0))
* Allow multiple proxies ([#1562](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1562)) ([eed5a15](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/eed5a1536af37877f12eebab6481acaa6efa55c5))
* **backend:** Add header secret to protect against public requests ([#1158](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1158)) ([fa6a56b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6a56b62f4b3aeeec704cfe4882755998d40833))
* **backend:** Add one simple retry on Unknown ([fcffc1a](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fcffc1a28bab990b0596ad8b66163e47a494191b))
* **backend:** Add POST /v1/bulk ([#1413](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1413)) ([d9302d4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d9302d4c1cec6a5a1788afe2a3718df8986f118f))
* **backend:** Add reply-to queue ([aaea59f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/aaea59f251634db7c35f029b09ef6e5f8c77cfbc))
* **backend:** Add worker webhook ([db90cfa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/db90cfa27b85916685268a3599bdfdb2c46de07a))
* **backend:** Customize SMTP defaults ([8f152b8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8f152b83c70b94618b71308552a6999f4b27aa2f))
* **backend:** Prune bulk email verification database ([#1377](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1377)) ([f905735](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/f90573566abf40133ebfb28ebc8f18ad8278a9b3))
* **backend:** Reject a request with to_email field empty or missing ([#1353](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1353)) ([1d9c29f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/1d9c29f5a48655a11f985b7df91c8bcbdf102487))
* **backend:** Remove /v0/bulk endpoints ([#1421](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1421)) ([522f324](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/522f32448416cd75a70ddb51038e50d06c3130b4))
* **backend:** Support RCH_SMTP_TIMEOUT ([#1407](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1407)) ([b9bda40](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b9bda4049540372811a86d8dd7ba873c9875e54d))
* **core:** Add check gravatar image ([#1188](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1188)) ([6a26035](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6a26035327ab681a65a4f4ba284e155f00680e89))
* **core:** Add Hotmail checks via headless password recovery ([#1165](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1165)) ([7517ed9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7517ed98ba966158deebba6a1a4745c931bfed18))
* **core:** Update async-smtp to 0.9 ([#1520](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1520)) ([297ce4f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/297ce4f11994b483faa015bebe4abf550eb77e11))
* Increase content length limit for bulk validation endpoint ([#1525](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1525)) ([bbdab31](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bbdab31e0dde54d21f4eeb5880ae28e60de7dced))
* Move `backend` code to this repo ([#1138](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1138)) ([0dc6053](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0dc60531d26efb217137347ef2b6aaf678d94238))
* Revert back to `check_email` input with single email ([#1150](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1150)) ([ce1ba53](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ce1ba5346849b578a0ed30b1d72096f15cfbc09d))
* Set default timeout to 10s ([#1251](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1251)) ([d04f84c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d04f84cc1e7b30e02d3717ab1af9f680cdb2c27f))
* Yahoo account recovery via headless ([#1364](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1364)) ([6f0f12b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6f0f12b8cf528e819f8743f7e3c5f5e141c51559))


### Bug Fixes

* Add "utilisateur inconnu" in invalid parser ([#1594](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1594)) ([fb91653](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb9165303e2d7be59ed2fa4f0682e8592bc0c5e7))
* Add backend_name in /v0/check_email ([a738fae](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/a738faec99942d20b817298f7850e84ab3e74835))
* **backend:** CSV download retrieves all results ([#1362](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1362)) ([b3670fc](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b3670fcaebce05a0aab09bcc3253134cb3c643c1))
* **backend:** Fix docker CTRL+C ([3a7245f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3a7245f9a47e8332d682d437d9492559e5adf66f))
* **backend:** Fix dockerfile ([f0ed49f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/f0ed49f50238c1c71a130f3db19ec047af00b8df))
* **backend:** Fix env var for multiple queues ([ed19166](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ed191662b18c62f397b4fed6b95249b5aa76c423))
* **backend:** Improve sentry error messages ([#1155](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1155)) ([d90d998](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d90d998d1cb189fed3f888659aa08fd4fabf6e93))
* **backend:** Redact email in sentry bug tracking ([2c2d1d8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2c2d1d88c0086196bc09359e32c96638124d9539))
* **backend:** Update sqlx to 0.7 ([#1390](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1390)) ([7198f87](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7198f87de92ab403cdc1e7c68667cdef9db96085))
* Bring back `{yahoo,hotmailb2c}_verif_method` ([#1606](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1606)) ([3fbe520](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3fbe5200a3d8608fbd72c0f2a5917326c1f8ec91))
* **core:** Clean up CheckEmailInput ([#1531](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1531)) ([b97b9ff](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b97b9ff9b91bdfbf18e5c0892559e87e7cd5e16c))
* **core:** Fix default CheckEmailInput ([09215a1](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/09215a13ac3525861e6cd1dea3fc71c13dfffe52))
* **core:** Fix MX random record selection ([#1263](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1263)) ([9fae593](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9fae593b8590ad5efb3e7d16bbd25cc05c228cb9))
* **core:** Headless check for Microsoft365 too ([#1346](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1346)) ([682cc2d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/682cc2d96b93d73f3fca3ba11f03800477c8fb9e))
* **core:** Improve invalid parser ([#1166](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1166)) ([bb46004](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bb460046bf1cb031fee706d836c8a737157f803c))
* **core:** Use Smtp for Gmail by default ([8e79884](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8e79884314f0c1eec5a7964fa686e2c60e7d2209))
* **core:** Use tagged enum representation ([ffde851](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ffde851068798adc3372d843a916a121b5caeccb))
* **docker:** Fix dockerfile entrypoint ([d1d3326](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d1d3326af88a85b2192796d8d2c92ff854b5644d))
* Fix dockerfile ([ce5067e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ce5067e4050e0cf3fa6c022bc7e25e5f15261c2a))
* Fix rabbitmq docker compose ([7c3856e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7c3856ebec6089b37b3dd30e3c4f13df9fb4e73a))
* Fix version in logs ([fa6be78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6be7867abae981b0d82fde24e0310b9759ab1f))
* Improve logging, add retries for Yahoo headless, switch to rustls ([#1549](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1549)) ([b1377db](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b1377db2b32155d766a09a76864fc9b0990833e6))
* Make new config backwards-compatible ([#1567](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1567)) ([b824e2c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b824e2c988ee4eef021b97fc65ebcfa36a166d7f))
* Put Smtp debug details in Debug struct ([5b71ca5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5b71ca59b6fab18263348aeafc7a895b7f4b8076))
* Reinstate proxy in JSON request ([#1569](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1569)) ([c36e6e0](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/c36e6e09c9079de210d288b84d79b984e2ea77f0))
* Remove local_ip retrieval ([ff8e599](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ff8e5998f8b88954b4104f9251d1331542dbb182))
* Remove max requests per minute/day ([07a6d96](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/07a6d96416f52ac0824e7e7ac665fd2169ddc7ec))
* Revert back to using lowest-priority MX record ([#1578](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1578)) ([60468b3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/60468b3f533491a0dff6a42e7096f34ece19896c))
* Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579)) ([3388163](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3388163d03b66ba92455be8404441e8555a9d53c))
* Support queues in env var ([39655d5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/39655d51afe5f65d62cd5dc3485586e16bcdec31))
* Typo in expect of RCH_VERIF_METHOD ([#1405](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1405)) ([c50d8eb](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/c50d8ebdfc470fe1ec6290e07668c70095298799))
* Use chromedriver instead of gecko for parallel requests ([e282e28](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/e282e28aeb7259d800f7faad97173c3a216095a4))


### Reverts

* "Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579))" ([56e7838](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/56e7838f28067b05b58f1fcd166368a915aafbbc))
* **backend:** Bring back the sqlxmq-based bulk verification ([#1477](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1477)) ([322ad4e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/322ad4e4b53d534a8ae6461f3d3383d67b219b5d))


### Miscellaneous Chores

* Rename all VerifyMethod to VerifMethod ([9f9607d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9f9607d35478a1051dde56812f8914ff75d4c5ac))


### Code Refactoring

* Change RUST_LOG target to `reacher` ([#1152](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1152)) ([7e87be2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7e87be26f1e35a6936bfc967c872cd42b93fd256))
* Use config-rs instead of env vars ([#1530](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1530)) ([bcd2dc8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bcd2dc867b7dc2bdaeb70097fd14109c2a40da17))
* Use verify method for known providers ([#1366](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1366)) ([5ca4dfa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5ca4dfa5ec38fba0ec7cfb052106da8d6af4df44))

## [1.0.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/v0.11.7...v1.0.0) (2025-11-28)


### ⚠ BREAKING CHANGES

* 
* **core:** 
* 
* **core:** 
* 
* **backend:** 
* Rename all VerifyMethod to VerifMethod
* For Hotmail, Gmail and Yahoo addresses, the `*_use_api` and `*_use_headless` parameters have been removed and replaced with a `*VerifyMethod`, an enum which can take value Api, Headless or Smtp. If using headless, pass a webdriver address to env variable RCH_WEBDRIVER_ADDR.
* `input.hotmail_use_headless` is now a bool instead of a string. Pass the webdriver address as an environment variable `RCH_WEBDRIVER_ADDR` now.
* 
* 

### Features

* **#289:** add haveibeenpwned check ([#1253](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1253)) ([166dbd2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/166dbd2cc878e30c51538b919abc1aaea4465c45))
* Add `/v1/{check_email,bulk}` endpoints with throttle&concurrency ([#1537](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1537)) ([08522e4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/08522e4326bbcbc980cf501d5d994d0c17222561))
* Add `misc.is_b2c` field ([#1553](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1553)) ([14a6759](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/14a6759d805d2051a4a1e1d81588279cb9c85336))
* Add AWS SQS support ([#1554](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1554)) ([92be54e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/92be54ebfe4a2d19101141f55e94fc8e9588ff95))
* Add back RabbitMQ-based worker ([#1513](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1513)) ([de75ece](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/de75eceef32c6ea512e0a301ec62d393bb59ff0f))
* Add debug information about each email verification ([#1391](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1391)) ([3ea6e66](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3ea6e6607735682dfca6ecfa27460650ac6e42d3))
* add email address normalisation ([#1206](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1206)) ([f8ec348](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/f8ec348883cd4f4a20a8acbb38d54b69e798222b))
* Add optional timeout on proxy (env var: `RCH__PROXY__TIMEOUT_MS`) ([#1595](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1595)) ([0e51eb6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0e51eb686dad6bd2ec827e785bf9c30ccc88cde1))
* Add RabbitMQ worker ([#1395](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1395)) ([ecef8c9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ecef8c98deb744390c7017a4e98d4f3c7e737fcb))
* Add suggestions for syntax errors ([#1192](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1192)) ([2d385f3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2d385f30f7a62ab2706599fbb89fb50275cffb5f))
* Allow /v1/check_email without worker mode ([9ca9f39](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9ca9f39ee487dc1b7d9b4cdc9a0b2c0669b10bc0))
* Allow multiple proxies ([#1562](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1562)) ([eed5a15](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/eed5a1536af37877f12eebab6481acaa6efa55c5))
* **backend:** Add header secret to protect against public requests ([#1158](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1158)) ([fa6a56b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6a56b62f4b3aeeec704cfe4882755998d40833))
* **backend:** Add one simple retry on Unknown ([fcffc1a](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fcffc1a28bab990b0596ad8b66163e47a494191b))
* **backend:** Add POST /v1/bulk ([#1413](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1413)) ([d9302d4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d9302d4c1cec6a5a1788afe2a3718df8986f118f))
* **backend:** Add reply-to queue ([aaea59f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/aaea59f251634db7c35f029b09ef6e5f8c77cfbc))
* **backend:** Add worker webhook ([db90cfa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/db90cfa27b85916685268a3599bdfdb2c46de07a))
* **backend:** Customize SMTP defaults ([8f152b8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8f152b83c70b94618b71308552a6999f4b27aa2f))
* **backend:** Prune bulk email verification database ([#1377](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1377)) ([f905735](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/f90573566abf40133ebfb28ebc8f18ad8278a9b3))
* **backend:** Reject a request with to_email field empty or missing ([#1353](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1353)) ([1d9c29f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/1d9c29f5a48655a11f985b7df91c8bcbdf102487))
* **backend:** Remove /v0/bulk endpoints ([#1421](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1421)) ([522f324](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/522f32448416cd75a70ddb51038e50d06c3130b4))
* **backend:** Support RCH_SMTP_TIMEOUT ([#1407](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1407)) ([b9bda40](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b9bda4049540372811a86d8dd7ba873c9875e54d))
* **core:** Add check gravatar image ([#1188](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1188)) ([6a26035](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6a26035327ab681a65a4f4ba284e155f00680e89))
* **core:** Add Hotmail checks via headless password recovery ([#1165](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1165)) ([7517ed9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7517ed98ba966158deebba6a1a4745c931bfed18))
* **core:** Update async-smtp to 0.9 ([#1520](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1520)) ([297ce4f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/297ce4f11994b483faa015bebe4abf550eb77e11))
* Increase content length limit for bulk validation endpoint ([#1525](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1525)) ([bbdab31](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bbdab31e0dde54d21f4eeb5880ae28e60de7dced))
* Move `backend` code to this repo ([#1138](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1138)) ([0dc6053](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0dc60531d26efb217137347ef2b6aaf678d94238))
* Revert back to `check_email` input with single email ([#1150](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1150)) ([ce1ba53](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ce1ba5346849b578a0ed30b1d72096f15cfbc09d))
* Set default timeout to 10s ([#1251](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1251)) ([d04f84c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d04f84cc1e7b30e02d3717ab1af9f680cdb2c27f))
* Yahoo account recovery via headless ([#1364](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1364)) ([6f0f12b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6f0f12b8cf528e819f8743f7e3c5f5e141c51559))


### Bug Fixes

* Add "utilisateur inconnu" in invalid parser ([#1594](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1594)) ([fb91653](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb9165303e2d7be59ed2fa4f0682e8592bc0c5e7))
* Add backend_name in /v0/check_email ([a738fae](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/a738faec99942d20b817298f7850e84ab3e74835))
* **backend:** CSV download retrieves all results ([#1362](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1362)) ([b3670fc](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b3670fcaebce05a0aab09bcc3253134cb3c643c1))
* **backend:** Fix docker CTRL+C ([3a7245f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3a7245f9a47e8332d682d437d9492559e5adf66f))
* **backend:** Fix dockerfile ([f0ed49f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/f0ed49f50238c1c71a130f3db19ec047af00b8df))
* **backend:** Fix env var for multiple queues ([ed19166](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ed191662b18c62f397b4fed6b95249b5aa76c423))
* **backend:** Improve sentry error messages ([#1155](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1155)) ([d90d998](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d90d998d1cb189fed3f888659aa08fd4fabf6e93))
* **backend:** Redact email in sentry bug tracking ([2c2d1d8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2c2d1d88c0086196bc09359e32c96638124d9539))
* **backend:** Update sqlx to 0.7 ([#1390](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1390)) ([7198f87](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7198f87de92ab403cdc1e7c68667cdef9db96085))
* Bring back `{yahoo,hotmailb2c}_verif_method` ([#1606](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1606)) ([3fbe520](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3fbe5200a3d8608fbd72c0f2a5917326c1f8ec91))
* **core:** Clean up CheckEmailInput ([#1531](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1531)) ([b97b9ff](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b97b9ff9b91bdfbf18e5c0892559e87e7cd5e16c))
* **core:** Fix default CheckEmailInput ([09215a1](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/09215a13ac3525861e6cd1dea3fc71c13dfffe52))
* **core:** Fix MX random record selection ([#1263](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1263)) ([9fae593](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9fae593b8590ad5efb3e7d16bbd25cc05c228cb9))
* **core:** Headless check for Microsoft365 too ([#1346](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1346)) ([682cc2d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/682cc2d96b93d73f3fca3ba11f03800477c8fb9e))
* **core:** Improve invalid parser ([#1166](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1166)) ([bb46004](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bb460046bf1cb031fee706d836c8a737157f803c))
* **core:** Use Smtp for Gmail by default ([8e79884](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8e79884314f0c1eec5a7964fa686e2c60e7d2209))
* **core:** Use tagged enum representation ([ffde851](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ffde851068798adc3372d843a916a121b5caeccb))
* **docker:** Fix dockerfile entrypoint ([d1d3326](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d1d3326af88a85b2192796d8d2c92ff854b5644d))
* Fix dockerfile ([ce5067e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ce5067e4050e0cf3fa6c022bc7e25e5f15261c2a))
* Fix rabbitmq docker compose ([7c3856e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7c3856ebec6089b37b3dd30e3c4f13df9fb4e73a))
* Fix version in logs ([fa6be78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6be7867abae981b0d82fde24e0310b9759ab1f))
* Improve logging, add retries for Yahoo headless, switch to rustls ([#1549](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1549)) ([b1377db](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b1377db2b32155d766a09a76864fc9b0990833e6))
* Make new config backwards-compatible ([#1567](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1567)) ([b824e2c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b824e2c988ee4eef021b97fc65ebcfa36a166d7f))
* Put Smtp debug details in Debug struct ([5b71ca5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5b71ca59b6fab18263348aeafc7a895b7f4b8076))
* Reinstate proxy in JSON request ([#1569](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1569)) ([c36e6e0](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/c36e6e09c9079de210d288b84d79b984e2ea77f0))
* Remove local_ip retrieval ([ff8e599](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ff8e5998f8b88954b4104f9251d1331542dbb182))
* Remove max requests per minute/day ([07a6d96](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/07a6d96416f52ac0824e7e7ac665fd2169ddc7ec))
* Revert back to using lowest-priority MX record ([#1578](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1578)) ([60468b3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/60468b3f533491a0dff6a42e7096f34ece19896c))
* Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579)) ([3388163](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3388163d03b66ba92455be8404441e8555a9d53c))
* Support queues in env var ([39655d5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/39655d51afe5f65d62cd5dc3485586e16bcdec31))
* Typo in expect of RCH_VERIF_METHOD ([#1405](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1405)) ([c50d8eb](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/c50d8ebdfc470fe1ec6290e07668c70095298799))
* Use chromedriver instead of gecko for parallel requests ([e282e28](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/e282e28aeb7259d800f7faad97173c3a216095a4))


### Reverts

* "Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579))" ([56e7838](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/56e7838f28067b05b58f1fcd166368a915aafbbc))
* **backend:** Bring back the sqlxmq-based bulk verification ([#1477](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1477)) ([322ad4e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/322ad4e4b53d534a8ae6461f3d3383d67b219b5d))


### Miscellaneous Chores

* Rename all VerifyMethod to VerifMethod ([9f9607d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9f9607d35478a1051dde56812f8914ff75d4c5ac))


### Code Refactoring

* Change RUST_LOG target to `reacher` ([#1152](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1152)) ([7e87be2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7e87be26f1e35a6936bfc967c872cd42b93fd256))
* Use config-rs instead of env vars ([#1530](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1530)) ([bcd2dc8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bcd2dc867b7dc2bdaeb70097fd14109c2a40da17))
* Use verify method for known providers ([#1366](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1366)) ([5ca4dfa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5ca4dfa5ec38fba0ec7cfb052106da8d6af4df44))
