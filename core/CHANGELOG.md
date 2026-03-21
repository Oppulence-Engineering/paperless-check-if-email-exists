# Changelog

## [4.1.0](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/compare/v4.0.0...v4.1.0) (2026-03-21)


### Features

* add spam-trap and honeypot domain detection ([6477a20](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/6477a2005b2cecfe0b39a71321d4985789253c57))
* add spam-trap and honeypot domain detection ([e275340](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/e275340e38f9ccf87b46ef26e2b50f2c2338bc97))


### Bug Fixes

* **smtp:** treat permanent 5.1.1 and 5.7.1 responses as invalid recipients ([#1631](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/issues/1631)) ([b3d6b07](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/b3d6b0751ae95c874014d99fb437e4cca4899d14))

## [4.0.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/v3.0.2...v4.0.0) (2025-12-12)


### ⚠ BREAKING CHANGES

* 
* **core:** 
* 
* **core:** 
* 
* Rename all VerifyMethod to VerifMethod
* For Hotmail, Gmail and Yahoo addresses, the `*_use_api` and `*_use_headless` parameters have been removed and replaced with a `*VerifyMethod`, an enum which can take value Api, Headless or Smtp. If using headless, pass a webdriver address to env variable RCH_WEBDRIVER_ADDR.
* `input.hotmail_use_headless` is now a bool instead of a string. Pass the webdriver address as an environment variable `RCH_WEBDRIVER_ADDR` now.
* **core:** `SmtpError::TimeoutError` has been removed in favor of the one async-smtp uses, namely `std::io::Error` with `ErrorKind::TimeoutError`
* 
* 
* Remove HTTP backend from CLI ([#1151](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1151))

### Features

* **#289:** add haveibeenpwned check ([#1253](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1253)) ([166dbd2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/166dbd2cc878e30c51538b919abc1aaea4465c45))
* Add `/v1/{check_email,bulk}` endpoints with throttle&concurrency ([#1537](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1537)) ([08522e4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/08522e4326bbcbc980cf501d5d994d0c17222561))
* Add `misc.is_b2c` field ([#1553](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1553)) ([14a6759](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/14a6759d805d2051a4a1e1d81588279cb9c85336))
* Add `smtp.error.description` field for human-readable description of error ([#1111](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1111)) ([43b47ea](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/43b47ea2b9250f2c6d58c8a0ec4340066169c169))
* Add AWS SQS support ([#1554](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1554)) ([92be54e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/92be54ebfe4a2d19101141f55e94fc8e9588ff95))
* Add debug information about each email verification ([#1391](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1391)) ([3ea6e66](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3ea6e6607735682dfca6ecfa27460650ac6e42d3))
* add email address normalisation ([#1206](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1206)) ([f8ec348](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/f8ec348883cd4f4a20a8acbb38d54b69e798222b))
* add Microsoft 365 HTTP API validation ([#1194](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1194)) ([5d3c49f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5d3c49f41ef1369efe2a9e63b24543e281ae0776))
* Add optional timeout on proxy (env var: `RCH__PROXY__TIMEOUT_MS`) ([#1595](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1595)) ([0e51eb6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0e51eb686dad6bd2ec827e785bf9c30ccc88cde1))
* Add possibility to set SMTP port ([#985](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/985)) ([cdabdf8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/cdabdf80e858908d6c33e1273dfdc1fef0f78d35))
* Add proxy field in SmtpDebug ([2f60a03](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2f60a03f25d56397eb54302b134730ef923d9105))
* Add proxy username/password ([#1057](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1057)) ([d9583c6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d9583c6ae0d3353a5135dd157999cf579b308d6d))
* Add skipped domains ([#1293](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1293)) ([29119fa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/29119fa72027c9830396bbdf3e90f08c0c89d7a7))
* Add SMTP retries to avoid greylisting ([#1041](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1041)) ([b451a1e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b451a1e93a6ccf025c78d56dee7439ad607c8507))
* Add suggestions for syntax errors ([#1192](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1192)) ([2d385f3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2d385f30f7a62ab2706599fbb89fb50275cffb5f))
* additional Gmail validation ([#1193](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1193)) ([49c8f5c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/49c8f5c3b4a3db04533d06d7267b0f15ebda3285))
* Allow multiple proxies ([#1562](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1562)) ([eed5a15](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/eed5a1536af37877f12eebab6481acaa6efa55c5))
* Allow user to define SMTP client security for TLS ([#1043](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1043)) ([bc722ff](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bc722ff1a9b30747308a3b3b5959d73e5e853292))
* **backend:** Add one simple retry on Unknown ([fcffc1a](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fcffc1a28bab990b0596ad8b66163e47a494191b))
* Break SmtpError into `{Helo,Connect,ConnectWithStream,MailFrom,RcptTo,Close}Error` ([#1055](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1055)) ([64e5193](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/64e5193c48a6bf4c080e79daeefd1c98dadffd5d))
* **core:** Add check for antispam MX records ([#1257](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1257)) ([c9771da](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/c9771da66c7869a4d0a255e2e2536f2863e8958c))
* **core:** Add check gravatar image ([#1188](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1188)) ([6a26035](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6a26035327ab681a65a4f4ba284e155f00680e89))
* **core:** Add domain-specific rules as JSON file ([#1347](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1347)) ([cab143c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/cab143c72889c585adbf041e9c248e57d0c4c4ca))
* **core:** Add Hotmail checks via headless password recovery ([#1165](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1165)) ([7517ed9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7517ed98ba966158deebba6a1a4745c931bfed18))
* **core:** Bump to 45s timeout for some domains ([#1348](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1348)) ([fda33a2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fda33a27441e2ccb1c4e97c0fc582abf25b1561f))
* **core:** Default Gmail checks to use API ([4304743](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/4304743fa93b6511857827afcdaa1fb9124bd62b))
* **core:** Fix disabled accts on hanmail.net ([#1339](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1339)) ([90393c8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/90393c8dda39267da7eb5efe6f112c8f25a593f4))
* **core:** Skip catch-all for known domains ([#1336](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1336)) ([c40a46c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/c40a46c4555129346bd9efa444a483bf25b679fe))
* **core:** Update async-smtp to 0.9 ([#1520](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1520)) ([297ce4f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/297ce4f11994b483faa015bebe4abf550eb77e11))
* **core:** Update default MAIL-FROM and HELO ([743a811](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/743a8111b4831ee19e7ac887c39a8da2775acd4c))
* Loop through all MX servers ([#1070](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1070)) ([11e6a06](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/11e6a06a67f5893b729c76d1a33667f83d63c836))
* Revert back to `check_email` input with single email ([#1150](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1150)) ([ce1ba53](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ce1ba5346849b578a0ed30b1d72096f15cfbc09d))
* Set default timeout to 10s ([#1251](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1251)) ([d04f84c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d04f84cc1e7b30e02d3717ab1af9f680cdb2c27f))
* Update parser.rs ([#1345](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1345)) ([8269f22](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8269f22f73214412f154927a908a7769d3f8b00c))
* Use opportunistic STARTTLS by default ([#1079](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1079)) ([54911f0](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/54911f0a8ec51e753f757878021e933609cff868))
* Yahoo account recovery via headless ([#1364](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1364)) ([6f0f12b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6f0f12b8cf528e819f8743f7e3c5f5e141c51559))


### Bug Fixes

* Add "utilisateur inconnu" in invalid parser ([#1594](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1594)) ([fb91653](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb9165303e2d7be59ed2fa4f0682e8592bc0c5e7))
* Add HoneyPot rule ([fb428ef](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb428ef42586641711dfd10190514ff5aa24583d))
* Bring back `{yahoo,hotmailb2c}_verif_method` ([#1606](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1606)) ([3fbe520](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3fbe5200a3d8608fbd72c0f2a5917326c1f8ec91))
* **cli:** Update flags default values ([a4fe57e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/a4fe57e9ab89659e12182719ccb12fb2cdcb5f2e))
* **core:** Add more invalid parsing and improve logging ([#1156](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1156)) ([b5ae9f8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b5ae9f8ad910b77ad6a179ecb5d4b633011ed2f4))
* **core:** Clean up CheckEmailInput ([#1531](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1531)) ([b97b9ff](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b97b9ff9b91bdfbf18e5c0892559e87e7cd5e16c))
* **core:** Default SMTP timeout to 15 ([0d4fa4d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0d4fa4d8f662ecfd3fa2e0359322f324a8ef86db))
* **core:** Don't use headless on Microsoft 465 addresses ([#1196](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1196)) ([0c3c21d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0c3c21daf6ea79875835121fb86ab7c0c86d55eb))
* **core:** Fix default CheckEmailInput ([09215a1](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/09215a13ac3525861e6cd1dea3fc71c13dfffe52))
* **core:** Fix gmail test ([ea80690](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ea80690b4168485ed7e03f4e228a12e276d605b0))
* **core:** Fix hotmail headless option parsing ([6ddc3b9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6ddc3b96da0d01b02711d62873ad0d0df6bf1b33))
* **core:** Fix hotmail headless with authenticator ([51cdb2e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/51cdb2e3c13a433fff92f1d3dcf1bfcb90f6ce7b))
* **core:** Fix hotmail/outlook checks ([5e4bf16](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5e4bf16e75e01ba17dd9022934359c9d03f3b0c8))
* **core:** Fix MX random record selection ([#1263](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1263)) ([9fae593](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9fae593b8590ad5efb3e7d16bbd25cc05c228cb9))
* **core:** Headless check for Microsoft365 too ([#1346](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1346)) ([682cc2d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/682cc2d96b93d73f3fca3ba11f03800477c8fb9e))
* **core:** Improve invalid parser ([#1166](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1166)) ([bb46004](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bb460046bf1cb031fee706d836c8a737157f803c))
* **core:** Improve parser and headless hotmail runner ([#1167](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1167)) ([0de33a5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0de33a5f265105a769c7ca6125df0fd4f88b89e2))
* **core:** Improve parser from Sentry errors ([fbaf588](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fbaf58824a339e546d50c2125a459161769dda6e))
* **core:** Improve parser's `is_invalid` ([#1159](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1159)) ([ec1c4d5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ec1c4d5e5d4c94d75d255a0699402f75eb29f7ab))
* **core:** More robust Hotmail invalid check ([ee741f4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ee741f4570050f559395e687da64c64ff9046afb))
* **core:** No sandbox in headless Hotmail check ([0590438](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0590438310f3c052b2748a8c408e0d8dbfb777b7))
* **core:** Prefer empty MX lookup when Err NoRecordsFound ([#1409](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1409)) ([d4b5ef9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d4b5ef9696a8c3ff0eaad2d3b5321437bd2a4df3))
* **core:** Remove antispam check ([#1337](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1337)) ([06f18ed](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/06f18edf7aee5640b3725feedfa7b7f213da83a8))
* **core:** Use semver in sentry ([03e6c97](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/03e6c97a7f842b115b367ca942119496d8400024))
* **core:** Use Smtp for Gmail by default ([8e79884](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8e79884314f0c1eec5a7964fa686e2c60e7d2209))
* **core:** Use tagged enum representation ([ffde851](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ffde851068798adc3372d843a916a121b5caeccb))
* **core:** Yahoo add back IDENTIFIER_EXISTS ([2b63556](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2b635564efb37b0aa891bbba77244e6cf2d611bb))
* **core:** yahoo api changes: yid is userId now, sessionIndex is required and fo… ([#1314](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1314)) ([0209111](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/02091115026520596fc5b4b2a6757169e91cba15))
* Don't auto-fetch Chrome, install in Docker ([84fcc0d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/84fcc0de40567126ce3a385934086450c3a89ccf))
* Don't show proxy full info in logs ([2668ce1](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2668ce14418076b00f36f18a370070ac1f3754bf))
* Fix `has_full_inbox` check too lenient ([93de444](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/93de444dfa7c6d66061570115be8f53f0647c431))
* Fix duplicate `yahoo_verif_method` field in default() inputs ([#1428](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1428)) ([b7c51d5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b7c51d5caaf21140c174cb419aedaf8fe752f817))
* Fix parsing some invalid emails ([cb65c0f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/cb65c0f4767b2f163f48054652f7652b6d0b6043))
* Fix version in logs ([fa6be78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6be7867abae981b0d82fde24e0310b9759ab1f))
* Improve logging, add retries for Yahoo headless, switch to rustls ([#1549](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1549)) ([b1377db](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b1377db2b32155d766a09a76864fc9b0990833e6))
* Only do headless for non-365 hotmail emails ([1c52bdc](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/1c52bdc75fb201f2e54c62d5f67f50a56c57cb83))
* Put Smtp debug details in Debug struct ([5b71ca5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5b71ca59b6fab18263348aeafc7a895b7f4b8076))
* Remove local_ip retrieval ([ff8e599](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ff8e5998f8b88954b4104f9251d1331542dbb182))
* Revert back to using lowest-priority MX record ([#1578](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1578)) ([60468b3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/60468b3f533491a0dff6a42e7096f34ece19896c))
* Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579)) ([3388163](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3388163d03b66ba92455be8404441e8555a9d53c))
* split Microsoft 365/Hotmail functionality ([#1204](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1204)) ([e987b13](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/e987b13a5ccd98d28fb756f1bf41427c337750c4))
* Switch back to upstream fast-socks ([#1164](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1164)) ([db356f1](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/db356f19374843ca135de8ebd8a6c34bfeb017a8))
* Syntax also check using using `mailchecker` ([8385bec](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8385bec6fedc0912881800442bffda5b33c2f394))
* TLS accept unsafe ([778692b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/778692bce760c0a1e1201dd3e11b41e7ccb7e2e8))
* Use async_std_resolver::resolver_from_system_conf ([#982](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/982)) ([376c3b0](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/376c3b0d4743ccc60a1df2a9fa3e9f2f5cd68178))
* Use chromedriver instead of gecko for parallel requests ([e282e28](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/e282e28aeb7259d800f7faad97173c3a216095a4))
* Use proxy auth if provided for api checks ([#1416](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1416)) ([8340514](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/83405146f04a9c8b718b63635e74cd70decf4931))
* Use std::default for deriving ([#1015](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1015)) ([03720f0](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/03720f027fd68d5ea5ae538aa567a621f4a65fe3))
* Use TLS when available ([#964](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/964)) ([aed11d2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/aed11d2e15b6b7688ecaf856824ca6effbb5d21b))


### Reverts

* "Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579))" ([56e7838](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/56e7838f28067b05b58f1fcd166368a915aafbbc))
* **backend:** Bring back the sqlxmq-based bulk verification ([#1477](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1477)) ([322ad4e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/322ad4e4b53d534a8ae6461f3d3383d67b219b5d))


### Miscellaneous Chores

* Rename all VerifyMethod to VerifMethod ([9f9607d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9f9607d35478a1051dde56812f8914ff75d4c5ac))


### Code Refactoring

* Change RUST_LOG target to `reacher` ([#1152](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1152)) ([7e87be2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7e87be26f1e35a6936bfc967c872cd42b93fd256))
* Remove HTTP backend from CLI ([#1151](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1151)) ([7184372](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/71843720c9b87fa0e43fa482a35ef074435bf562))
* Use config-rs instead of env vars ([#1530](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1530)) ([bcd2dc8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bcd2dc867b7dc2bdaeb70097fd14109c2a40da17))
* Use verify method for known providers ([#1366](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1366)) ([5ca4dfa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5ca4dfa5ec38fba0ec7cfb052106da8d6af4df44))

## [3.0.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/v2.0.0...v3.0.0) (2025-11-28)


### ⚠ BREAKING CHANGES

* 
* **core:** 
* 
* **core:** 
* 
* Rename all VerifyMethod to VerifMethod
* For Hotmail, Gmail and Yahoo addresses, the `*_use_api` and `*_use_headless` parameters have been removed and replaced with a `*VerifyMethod`, an enum which can take value Api, Headless or Smtp. If using headless, pass a webdriver address to env variable RCH_WEBDRIVER_ADDR.
* `input.hotmail_use_headless` is now a bool instead of a string. Pass the webdriver address as an environment variable `RCH_WEBDRIVER_ADDR` now.
* **core:** `SmtpError::TimeoutError` has been removed in favor of the one async-smtp uses, namely `std::io::Error` with `ErrorKind::TimeoutError`
* 
* 
* Remove HTTP backend from CLI ([#1151](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1151))

### Features

* **#289:** add haveibeenpwned check ([#1253](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1253)) ([166dbd2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/166dbd2cc878e30c51538b919abc1aaea4465c45))
* Add `/v1/{check_email,bulk}` endpoints with throttle&concurrency ([#1537](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1537)) ([08522e4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/08522e4326bbcbc980cf501d5d994d0c17222561))
* Add `CheckEmailInput` setter `set_` prefix to differentiate with accessing fields ([#933](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/933)) ([276f656](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/276f6561e7a98af6415dbd4645d84cbe697b738e))
* Add `misc.is_b2c` field ([#1553](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1553)) ([14a6759](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/14a6759d805d2051a4a1e1d81588279cb9c85336))
* Add `smtp.error.description` field for human-readable description of error ([#1111](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1111)) ([43b47ea](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/43b47ea2b9250f2c6d58c8a0ec4340066169c169))
* Add AWS SQS support ([#1554](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1554)) ([92be54e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/92be54ebfe4a2d19101141f55e94fc8e9588ff95))
* Add debug information about each email verification ([#1391](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1391)) ([3ea6e66](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3ea6e6607735682dfca6ecfa27460650ac6e42d3))
* Add deprecated warning when running HTTP server ([#943](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/943)) ([e4b1570](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/e4b1570a8be5573f7394a3139f34ab021452cc3a))
* add email address normalisation ([#1206](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1206)) ([f8ec348](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/f8ec348883cd4f4a20a8acbb38d54b69e798222b))
* add Microsoft 365 HTTP API validation ([#1194](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1194)) ([5d3c49f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5d3c49f41ef1369efe2a9e63b24543e281ae0776))
* Add optional timeout on proxy (env var: `RCH__PROXY__TIMEOUT_MS`) ([#1595](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1595)) ([0e51eb6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0e51eb686dad6bd2ec827e785bf9c30ccc88cde1))
* Add possibility to set SMTP port ([#985](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/985)) ([cdabdf8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/cdabdf80e858908d6c33e1273dfdc1fef0f78d35))
* Add proxy field in SmtpDebug ([2f60a03](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2f60a03f25d56397eb54302b134730ef923d9105))
* Add proxy username/password ([#1057](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1057)) ([d9583c6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d9583c6ae0d3353a5135dd157999cf579b308d6d))
* Add skipped domains ([#1293](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1293)) ([29119fa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/29119fa72027c9830396bbdf3e90f08c0c89d7a7))
* Add SMTP retries to avoid greylisting ([#1041](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1041)) ([b451a1e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b451a1e93a6ccf025c78d56dee7439ad607c8507))
* Add suggestions for syntax errors ([#1192](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1192)) ([2d385f3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2d385f30f7a62ab2706599fbb89fb50275cffb5f))
* additional Gmail validation ([#1193](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1193)) ([49c8f5c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/49c8f5c3b4a3db04533d06d7267b0f15ebda3285))
* Allow multiple proxies ([#1562](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1562)) ([eed5a15](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/eed5a1536af37877f12eebab6481acaa6efa55c5))
* Allow user to define SMTP client security for TLS ([#1043](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1043)) ([bc722ff](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bc722ff1a9b30747308a3b3b5959d73e5e853292))
* **backend:** Add one simple retry on Unknown ([fcffc1a](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fcffc1a28bab990b0596ad8b66163e47a494191b))
* Break SmtpError into `{Helo,Connect,ConnectWithStream,MailFrom,RcptTo,Close}Error` ([#1055](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1055)) ([64e5193](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/64e5193c48a6bf4c080e79daeefd1c98dadffd5d))
* **core:** Add check for antispam MX records ([#1257](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1257)) ([c9771da](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/c9771da66c7869a4d0a255e2e2536f2863e8958c))
* **core:** Add check gravatar image ([#1188](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1188)) ([6a26035](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6a26035327ab681a65a4f4ba284e155f00680e89))
* **core:** Add domain-specific rules as JSON file ([#1347](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1347)) ([cab143c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/cab143c72889c585adbf041e9c248e57d0c4c4ca))
* **core:** Add Hotmail checks via headless password recovery ([#1165](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1165)) ([7517ed9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7517ed98ba966158deebba6a1a4745c931bfed18))
* **core:** Bump to 45s timeout for some domains ([#1348](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1348)) ([fda33a2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fda33a27441e2ccb1c4e97c0fc582abf25b1561f))
* **core:** Default Gmail checks to use API ([4304743](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/4304743fa93b6511857827afcdaa1fb9124bd62b))
* **core:** Fix disabled accts on hanmail.net ([#1339](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1339)) ([90393c8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/90393c8dda39267da7eb5efe6f112c8f25a593f4))
* **core:** Skip catch-all for known domains ([#1336](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1336)) ([c40a46c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/c40a46c4555129346bd9efa444a483bf25b679fe))
* **core:** Update async-smtp to 0.9 ([#1520](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1520)) ([297ce4f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/297ce4f11994b483faa015bebe4abf550eb77e11))
* **core:** Update default MAIL-FROM and HELO ([743a811](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/743a8111b4831ee19e7ac887c39a8da2775acd4c))
* Loop through all MX servers ([#1070](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1070)) ([11e6a06](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/11e6a06a67f5893b729c76d1a33667f83d63c836))
* Revert back to `check_email` input with single email ([#1150](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1150)) ([ce1ba53](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ce1ba5346849b578a0ed30b1d72096f15cfbc09d))
* Set default timeout to 10s ([#1251](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1251)) ([d04f84c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d04f84cc1e7b30e02d3717ab1af9f680cdb2c27f))
* Update parser.rs ([#1345](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1345)) ([8269f22](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8269f22f73214412f154927a908a7769d3f8b00c))
* Use opportunistic STARTTLS by default ([#1079](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1079)) ([54911f0](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/54911f0a8ec51e753f757878021e933609cff868))
* Yahoo account recovery via headless ([#1364](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1364)) ([6f0f12b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6f0f12b8cf528e819f8743f7e3c5f5e141c51559))


### Bug Fixes

* Add "utilisateur inconnu" in invalid parser ([#1594](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1594)) ([fb91653](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb9165303e2d7be59ed2fa4f0682e8592bc0c5e7))
* Add HoneyPot rule ([fb428ef](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb428ef42586641711dfd10190514ff5aa24583d))
* Add serde (De)Serialize to pub structs ([#931](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/931)) ([949475d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/949475dee4a1ed96e873688e7432c702eb30af62))
* Bring back `{yahoo,hotmailb2c}_verif_method` ([#1606](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1606)) ([3fbe520](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3fbe5200a3d8608fbd72c0f2a5917326c1f8ec91))
* **cli:** Update flags default values ([a4fe57e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/a4fe57e9ab89659e12182719ccb12fb2cdcb5f2e))
* **core:** Add more invalid parsing and improve logging ([#1156](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1156)) ([b5ae9f8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b5ae9f8ad910b77ad6a179ecb5d4b633011ed2f4))
* **core:** Clean up CheckEmailInput ([#1531](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1531)) ([b97b9ff](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b97b9ff9b91bdfbf18e5c0892559e87e7cd5e16c))
* **core:** Default SMTP timeout to 15 ([0d4fa4d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0d4fa4d8f662ecfd3fa2e0359322f324a8ef86db))
* **core:** Don't use headless on Microsoft 465 addresses ([#1196](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1196)) ([0c3c21d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0c3c21daf6ea79875835121fb86ab7c0c86d55eb))
* **core:** Fix default CheckEmailInput ([09215a1](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/09215a13ac3525861e6cd1dea3fc71c13dfffe52))
* **core:** Fix gmail test ([ea80690](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ea80690b4168485ed7e03f4e228a12e276d605b0))
* **core:** Fix hotmail headless option parsing ([6ddc3b9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6ddc3b96da0d01b02711d62873ad0d0df6bf1b33))
* **core:** Fix hotmail headless with authenticator ([51cdb2e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/51cdb2e3c13a433fff92f1d3dcf1bfcb90f6ce7b))
* **core:** Fix hotmail/outlook checks ([5e4bf16](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5e4bf16e75e01ba17dd9022934359c9d03f3b0c8))
* **core:** Fix MX random record selection ([#1263](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1263)) ([9fae593](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9fae593b8590ad5efb3e7d16bbd25cc05c228cb9))
* **core:** Headless check for Microsoft365 too ([#1346](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1346)) ([682cc2d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/682cc2d96b93d73f3fca3ba11f03800477c8fb9e))
* **core:** Improve invalid parser ([#1166](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1166)) ([bb46004](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bb460046bf1cb031fee706d836c8a737157f803c))
* **core:** Improve parser and headless hotmail runner ([#1167](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1167)) ([0de33a5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0de33a5f265105a769c7ca6125df0fd4f88b89e2))
* **core:** Improve parser from Sentry errors ([fbaf588](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fbaf58824a339e546d50c2125a459161769dda6e))
* **core:** Improve parser's `is_invalid` ([#1159](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1159)) ([ec1c4d5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ec1c4d5e5d4c94d75d255a0699402f75eb29f7ab))
* **core:** More robust Hotmail invalid check ([ee741f4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ee741f4570050f559395e687da64c64ff9046afb))
* **core:** No sandbox in headless Hotmail check ([0590438](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0590438310f3c052b2748a8c408e0d8dbfb777b7))
* **core:** Prefer empty MX lookup when Err NoRecordsFound ([#1409](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1409)) ([d4b5ef9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d4b5ef9696a8c3ff0eaad2d3b5321437bd2a4df3))
* **core:** Remove antispam check ([#1337](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1337)) ([06f18ed](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/06f18edf7aee5640b3725feedfa7b7f213da83a8))
* **core:** Use semver in sentry ([03e6c97](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/03e6c97a7f842b115b367ca942119496d8400024))
* **core:** Use Smtp for Gmail by default ([8e79884](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8e79884314f0c1eec5a7964fa686e2c60e7d2209))
* **core:** Use tagged enum representation ([ffde851](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ffde851068798adc3372d843a916a121b5caeccb))
* **core:** Yahoo add back IDENTIFIER_EXISTS ([2b63556](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2b635564efb37b0aa891bbba77244e6cf2d611bb))
* **core:** yahoo api changes: yid is userId now, sessionIndex is required and fo… ([#1314](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1314)) ([0209111](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/02091115026520596fc5b4b2a6757169e91cba15))
* Don't auto-fetch Chrome, install in Docker ([84fcc0d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/84fcc0de40567126ce3a385934086450c3a89ccf))
* Don't show proxy full info in logs ([2668ce1](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2668ce14418076b00f36f18a370070ac1f3754bf))
* Fix `has_full_inbox` check too lenient ([93de444](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/93de444dfa7c6d66061570115be8f53f0647c431))
* Fix duplicate `yahoo_verif_method` field in default() inputs ([#1428](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1428)) ([b7c51d5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b7c51d5caaf21140c174cb419aedaf8fe752f817))
* Fix parsing some invalid emails ([cb65c0f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/cb65c0f4767b2f163f48054652f7652b6d0b6043))
* Fix version in logs ([fa6be78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6be7867abae981b0d82fde24e0310b9759ab1f))
* Improve logging, add retries for Yahoo headless, switch to rustls ([#1549](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1549)) ([b1377db](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b1377db2b32155d766a09a76864fc9b0990833e6))
* Only do headless for non-365 hotmail emails ([1c52bdc](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/1c52bdc75fb201f2e54c62d5f67f50a56c57cb83))
* Put Smtp debug details in Debug struct ([5b71ca5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5b71ca59b6fab18263348aeafc7a895b7f4b8076))
* Remove local_ip retrieval ([ff8e599](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ff8e5998f8b88954b4104f9251d1331542dbb182))
* Revert back to using lowest-priority MX record ([#1578](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1578)) ([60468b3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/60468b3f533491a0dff6a42e7096f34ece19896c))
* Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579)) ([3388163](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3388163d03b66ba92455be8404441e8555a9d53c))
* split Microsoft 365/Hotmail functionality ([#1204](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1204)) ([e987b13](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/e987b13a5ccd98d28fb756f1bf41427c337750c4))
* Switch back to upstream fast-socks ([#1164](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1164)) ([db356f1](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/db356f19374843ca135de8ebd8a6c34bfeb017a8))
* Syntax also check using using `mailchecker` ([8385bec](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8385bec6fedc0912881800442bffda5b33c2f394))
* TLS accept unsafe ([778692b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/778692bce760c0a1e1201dd3e11b41e7ccb7e2e8))
* Use async_std_resolver::resolver_from_system_conf ([#982](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/982)) ([376c3b0](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/376c3b0d4743ccc60a1df2a9fa3e9f2f5cd68178))
* Use chromedriver instead of gecko for parallel requests ([e282e28](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/e282e28aeb7259d800f7faad97173c3a216095a4))
* Use proxy auth if provided for api checks ([#1416](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1416)) ([8340514](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/83405146f04a9c8b718b63635e74cd70decf4931))
* Use std::default for deriving ([#1015](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1015)) ([03720f0](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/03720f027fd68d5ea5ae538aa567a621f4a65fe3))
* Use TLS when available ([#964](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/964)) ([aed11d2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/aed11d2e15b6b7688ecaf856824ca6effbb5d21b))


### Reverts

* "Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579))" ([56e7838](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/56e7838f28067b05b58f1fcd166368a915aafbbc))
* **backend:** Bring back the sqlxmq-based bulk verification ([#1477](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1477)) ([322ad4e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/322ad4e4b53d534a8ae6461f3d3383d67b219b5d))


### Miscellaneous Chores

* Rename all VerifyMethod to VerifMethod ([9f9607d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9f9607d35478a1051dde56812f8914ff75d4c5ac))


### Code Refactoring

* Change RUST_LOG target to `reacher` ([#1152](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1152)) ([7e87be2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7e87be26f1e35a6936bfc967c872cd42b93fd256))
* Remove HTTP backend from CLI ([#1151](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1151)) ([7184372](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/71843720c9b87fa0e43fa482a35ef074435bf562))
* Use config-rs instead of env vars ([#1530](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1530)) ([bcd2dc8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bcd2dc867b7dc2bdaeb70097fd14109c2a40da17))
* Use verify method for known providers ([#1366](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1366)) ([5ca4dfa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5ca4dfa5ec38fba0ec7cfb052106da8d6af4df44))

## [2.0.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/core-v1.0.0...core-v2.0.0) (2025-11-28)


### ⚠ BREAKING CHANGES

* 
* **core:** 
* 
* **core:** 
* 
* Rename all VerifyMethod to VerifMethod
* For Hotmail, Gmail and Yahoo addresses, the `*_use_api` and `*_use_headless` parameters have been removed and replaced with a `*VerifyMethod`, an enum which can take value Api, Headless or Smtp. If using headless, pass a webdriver address to env variable RCH_WEBDRIVER_ADDR.
* `input.hotmail_use_headless` is now a bool instead of a string. Pass the webdriver address as an environment variable `RCH_WEBDRIVER_ADDR` now.
* **core:** `SmtpError::TimeoutError` has been removed in favor of the one async-smtp uses, namely `std::io::Error` with `ErrorKind::TimeoutError`
* 
* 
* Remove HTTP backend from CLI ([#1151](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1151))

### Features

* **#289:** add haveibeenpwned check ([#1253](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1253)) ([166dbd2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/166dbd2cc878e30c51538b919abc1aaea4465c45))
* Add `/v1/{check_email,bulk}` endpoints with throttle&concurrency ([#1537](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1537)) ([08522e4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/08522e4326bbcbc980cf501d5d994d0c17222561))
* Add `CheckEmailInput` setter `set_` prefix to differentiate with accessing fields ([#933](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/933)) ([276f656](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/276f6561e7a98af6415dbd4645d84cbe697b738e))
* Add `misc.is_b2c` field ([#1553](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1553)) ([14a6759](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/14a6759d805d2051a4a1e1d81588279cb9c85336))
* Add `smtp.error.description` field for human-readable description of error ([#1111](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1111)) ([43b47ea](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/43b47ea2b9250f2c6d58c8a0ec4340066169c169))
* Add AWS SQS support ([#1554](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1554)) ([92be54e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/92be54ebfe4a2d19101141f55e94fc8e9588ff95))
* Add debug information about each email verification ([#1391](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1391)) ([3ea6e66](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3ea6e6607735682dfca6ecfa27460650ac6e42d3))
* Add deprecated warning when running HTTP server ([#943](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/943)) ([e4b1570](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/e4b1570a8be5573f7394a3139f34ab021452cc3a))
* add email address normalisation ([#1206](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1206)) ([f8ec348](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/f8ec348883cd4f4a20a8acbb38d54b69e798222b))
* add Microsoft 365 HTTP API validation ([#1194](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1194)) ([5d3c49f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5d3c49f41ef1369efe2a9e63b24543e281ae0776))
* Add optional timeout on proxy (env var: `RCH__PROXY__TIMEOUT_MS`) ([#1595](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1595)) ([0e51eb6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0e51eb686dad6bd2ec827e785bf9c30ccc88cde1))
* Add possibility to set SMTP port ([#985](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/985)) ([cdabdf8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/cdabdf80e858908d6c33e1273dfdc1fef0f78d35))
* Add proxy field in SmtpDebug ([2f60a03](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2f60a03f25d56397eb54302b134730ef923d9105))
* Add proxy username/password ([#1057](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1057)) ([d9583c6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d9583c6ae0d3353a5135dd157999cf579b308d6d))
* Add skipped domains ([#1293](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1293)) ([29119fa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/29119fa72027c9830396bbdf3e90f08c0c89d7a7))
* Add SMTP retries to avoid greylisting ([#1041](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1041)) ([b451a1e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b451a1e93a6ccf025c78d56dee7439ad607c8507))
* Add suggestions for syntax errors ([#1192](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1192)) ([2d385f3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2d385f30f7a62ab2706599fbb89fb50275cffb5f))
* additional Gmail validation ([#1193](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1193)) ([49c8f5c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/49c8f5c3b4a3db04533d06d7267b0f15ebda3285))
* Allow multiple proxies ([#1562](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1562)) ([eed5a15](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/eed5a1536af37877f12eebab6481acaa6efa55c5))
* Allow user to define SMTP client security for TLS ([#1043](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1043)) ([bc722ff](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bc722ff1a9b30747308a3b3b5959d73e5e853292))
* **backend:** Add one simple retry on Unknown ([fcffc1a](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fcffc1a28bab990b0596ad8b66163e47a494191b))
* Break SmtpError into `{Helo,Connect,ConnectWithStream,MailFrom,RcptTo,Close}Error` ([#1055](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1055)) ([64e5193](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/64e5193c48a6bf4c080e79daeefd1c98dadffd5d))
* **core:** Add check for antispam MX records ([#1257](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1257)) ([c9771da](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/c9771da66c7869a4d0a255e2e2536f2863e8958c))
* **core:** Add check gravatar image ([#1188](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1188)) ([6a26035](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6a26035327ab681a65a4f4ba284e155f00680e89))
* **core:** Add domain-specific rules as JSON file ([#1347](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1347)) ([cab143c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/cab143c72889c585adbf041e9c248e57d0c4c4ca))
* **core:** Add Hotmail checks via headless password recovery ([#1165](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1165)) ([7517ed9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7517ed98ba966158deebba6a1a4745c931bfed18))
* **core:** Bump to 45s timeout for some domains ([#1348](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1348)) ([fda33a2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fda33a27441e2ccb1c4e97c0fc582abf25b1561f))
* **core:** Default Gmail checks to use API ([4304743](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/4304743fa93b6511857827afcdaa1fb9124bd62b))
* **core:** Fix disabled accts on hanmail.net ([#1339](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1339)) ([90393c8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/90393c8dda39267da7eb5efe6f112c8f25a593f4))
* **core:** Skip catch-all for known domains ([#1336](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1336)) ([c40a46c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/c40a46c4555129346bd9efa444a483bf25b679fe))
* **core:** Update async-smtp to 0.9 ([#1520](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1520)) ([297ce4f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/297ce4f11994b483faa015bebe4abf550eb77e11))
* **core:** Update default MAIL-FROM and HELO ([743a811](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/743a8111b4831ee19e7ac887c39a8da2775acd4c))
* Loop through all MX servers ([#1070](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1070)) ([11e6a06](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/11e6a06a67f5893b729c76d1a33667f83d63c836))
* Revert back to `check_email` input with single email ([#1150](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1150)) ([ce1ba53](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ce1ba5346849b578a0ed30b1d72096f15cfbc09d))
* Set default timeout to 10s ([#1251](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1251)) ([d04f84c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d04f84cc1e7b30e02d3717ab1af9f680cdb2c27f))
* Update parser.rs ([#1345](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1345)) ([8269f22](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8269f22f73214412f154927a908a7769d3f8b00c))
* Use opportunistic STARTTLS by default ([#1079](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1079)) ([54911f0](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/54911f0a8ec51e753f757878021e933609cff868))
* Yahoo account recovery via headless ([#1364](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1364)) ([6f0f12b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6f0f12b8cf528e819f8743f7e3c5f5e141c51559))


### Bug Fixes

* Add "utilisateur inconnu" in invalid parser ([#1594](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1594)) ([fb91653](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb9165303e2d7be59ed2fa4f0682e8592bc0c5e7))
* Add HoneyPot rule ([fb428ef](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb428ef42586641711dfd10190514ff5aa24583d))
* Add serde (De)Serialize to pub structs ([#931](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/931)) ([949475d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/949475dee4a1ed96e873688e7432c702eb30af62))
* Bring back `{yahoo,hotmailb2c}_verif_method` ([#1606](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1606)) ([3fbe520](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3fbe5200a3d8608fbd72c0f2a5917326c1f8ec91))
* **cli:** Update flags default values ([a4fe57e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/a4fe57e9ab89659e12182719ccb12fb2cdcb5f2e))
* **core:** Add more invalid parsing and improve logging ([#1156](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1156)) ([b5ae9f8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b5ae9f8ad910b77ad6a179ecb5d4b633011ed2f4))
* **core:** Clean up CheckEmailInput ([#1531](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1531)) ([b97b9ff](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b97b9ff9b91bdfbf18e5c0892559e87e7cd5e16c))
* **core:** Default SMTP timeout to 15 ([0d4fa4d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0d4fa4d8f662ecfd3fa2e0359322f324a8ef86db))
* **core:** Don't use headless on Microsoft 465 addresses ([#1196](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1196)) ([0c3c21d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0c3c21daf6ea79875835121fb86ab7c0c86d55eb))
* **core:** Fix default CheckEmailInput ([09215a1](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/09215a13ac3525861e6cd1dea3fc71c13dfffe52))
* **core:** Fix gmail test ([ea80690](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ea80690b4168485ed7e03f4e228a12e276d605b0))
* **core:** Fix hotmail headless option parsing ([6ddc3b9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6ddc3b96da0d01b02711d62873ad0d0df6bf1b33))
* **core:** Fix hotmail headless with authenticator ([51cdb2e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/51cdb2e3c13a433fff92f1d3dcf1bfcb90f6ce7b))
* **core:** Fix hotmail/outlook checks ([5e4bf16](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5e4bf16e75e01ba17dd9022934359c9d03f3b0c8))
* **core:** Fix MX random record selection ([#1263](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1263)) ([9fae593](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9fae593b8590ad5efb3e7d16bbd25cc05c228cb9))
* **core:** Headless check for Microsoft365 too ([#1346](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1346)) ([682cc2d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/682cc2d96b93d73f3fca3ba11f03800477c8fb9e))
* **core:** Improve invalid parser ([#1166](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1166)) ([bb46004](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bb460046bf1cb031fee706d836c8a737157f803c))
* **core:** Improve parser and headless hotmail runner ([#1167](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1167)) ([0de33a5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0de33a5f265105a769c7ca6125df0fd4f88b89e2))
* **core:** Improve parser from Sentry errors ([fbaf588](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fbaf58824a339e546d50c2125a459161769dda6e))
* **core:** Improve parser's `is_invalid` ([#1159](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1159)) ([ec1c4d5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ec1c4d5e5d4c94d75d255a0699402f75eb29f7ab))
* **core:** More robust Hotmail invalid check ([ee741f4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ee741f4570050f559395e687da64c64ff9046afb))
* **core:** No sandbox in headless Hotmail check ([0590438](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0590438310f3c052b2748a8c408e0d8dbfb777b7))
* **core:** Prefer empty MX lookup when Err NoRecordsFound ([#1409](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1409)) ([d4b5ef9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d4b5ef9696a8c3ff0eaad2d3b5321437bd2a4df3))
* **core:** Remove antispam check ([#1337](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1337)) ([06f18ed](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/06f18edf7aee5640b3725feedfa7b7f213da83a8))
* **core:** Use semver in sentry ([03e6c97](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/03e6c97a7f842b115b367ca942119496d8400024))
* **core:** Use Smtp for Gmail by default ([8e79884](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8e79884314f0c1eec5a7964fa686e2c60e7d2209))
* **core:** Use tagged enum representation ([ffde851](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ffde851068798adc3372d843a916a121b5caeccb))
* **core:** Yahoo add back IDENTIFIER_EXISTS ([2b63556](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2b635564efb37b0aa891bbba77244e6cf2d611bb))
* **core:** yahoo api changes: yid is userId now, sessionIndex is required and fo… ([#1314](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1314)) ([0209111](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/02091115026520596fc5b4b2a6757169e91cba15))
* Don't auto-fetch Chrome, install in Docker ([84fcc0d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/84fcc0de40567126ce3a385934086450c3a89ccf))
* Don't show proxy full info in logs ([2668ce1](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2668ce14418076b00f36f18a370070ac1f3754bf))
* Fix `has_full_inbox` check too lenient ([93de444](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/93de444dfa7c6d66061570115be8f53f0647c431))
* Fix duplicate `yahoo_verif_method` field in default() inputs ([#1428](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1428)) ([b7c51d5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b7c51d5caaf21140c174cb419aedaf8fe752f817))
* Fix parsing some invalid emails ([cb65c0f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/cb65c0f4767b2f163f48054652f7652b6d0b6043))
* Fix version in logs ([fa6be78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6be7867abae981b0d82fde24e0310b9759ab1f))
* Improve logging, add retries for Yahoo headless, switch to rustls ([#1549](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1549)) ([b1377db](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b1377db2b32155d766a09a76864fc9b0990833e6))
* Only do headless for non-365 hotmail emails ([1c52bdc](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/1c52bdc75fb201f2e54c62d5f67f50a56c57cb83))
* Put Smtp debug details in Debug struct ([5b71ca5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5b71ca59b6fab18263348aeafc7a895b7f4b8076))
* Remove local_ip retrieval ([ff8e599](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ff8e5998f8b88954b4104f9251d1331542dbb182))
* Revert back to using lowest-priority MX record ([#1578](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1578)) ([60468b3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/60468b3f533491a0dff6a42e7096f34ece19896c))
* Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579)) ([3388163](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3388163d03b66ba92455be8404441e8555a9d53c))
* split Microsoft 365/Hotmail functionality ([#1204](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1204)) ([e987b13](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/e987b13a5ccd98d28fb756f1bf41427c337750c4))
* Switch back to upstream fast-socks ([#1164](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1164)) ([db356f1](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/db356f19374843ca135de8ebd8a6c34bfeb017a8))
* Syntax also check using using `mailchecker` ([8385bec](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8385bec6fedc0912881800442bffda5b33c2f394))
* TLS accept unsafe ([778692b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/778692bce760c0a1e1201dd3e11b41e7ccb7e2e8))
* Use async_std_resolver::resolver_from_system_conf ([#982](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/982)) ([376c3b0](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/376c3b0d4743ccc60a1df2a9fa3e9f2f5cd68178))
* Use chromedriver instead of gecko for parallel requests ([e282e28](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/e282e28aeb7259d800f7faad97173c3a216095a4))
* Use proxy auth if provided for api checks ([#1416](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1416)) ([8340514](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/83405146f04a9c8b718b63635e74cd70decf4931))
* Use std::default for deriving ([#1015](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1015)) ([03720f0](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/03720f027fd68d5ea5ae538aa567a621f4a65fe3))
* Use TLS when available ([#964](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/964)) ([aed11d2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/aed11d2e15b6b7688ecaf856824ca6effbb5d21b))


### Reverts

* "Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579))" ([56e7838](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/56e7838f28067b05b58f1fcd166368a915aafbbc))
* **backend:** Bring back the sqlxmq-based bulk verification ([#1477](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1477)) ([322ad4e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/322ad4e4b53d534a8ae6461f3d3383d67b219b5d))


### Miscellaneous Chores

* Rename all VerifyMethod to VerifMethod ([9f9607d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9f9607d35478a1051dde56812f8914ff75d4c5ac))


### Code Refactoring

* Change RUST_LOG target to `reacher` ([#1152](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1152)) ([7e87be2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7e87be26f1e35a6936bfc967c872cd42b93fd256))
* Remove HTTP backend from CLI ([#1151](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1151)) ([7184372](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/71843720c9b87fa0e43fa482a35ef074435bf562))
* Use config-rs instead of env vars ([#1530](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1530)) ([bcd2dc8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bcd2dc867b7dc2bdaeb70097fd14109c2a40da17))
* Use verify method for known providers ([#1366](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1366)) ([5ca4dfa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5ca4dfa5ec38fba0ec7cfb052106da8d6af4df44))

## [1.0.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/v0.11.6...v1.0.0) (2025-11-28)


### ⚠ BREAKING CHANGES

* 
* **core:** 
* 
* **core:** 
* 
* Rename all VerifyMethod to VerifMethod
* For Hotmail, Gmail and Yahoo addresses, the `*_use_api` and `*_use_headless` parameters have been removed and replaced with a `*VerifyMethod`, an enum which can take value Api, Headless or Smtp. If using headless, pass a webdriver address to env variable RCH_WEBDRIVER_ADDR.
* `input.hotmail_use_headless` is now a bool instead of a string. Pass the webdriver address as an environment variable `RCH_WEBDRIVER_ADDR` now.
* **core:** `SmtpError::TimeoutError` has been removed in favor of the one async-smtp uses, namely `std::io::Error` with `ErrorKind::TimeoutError`
* 
* 
* Remove HTTP backend from CLI ([#1151](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1151))

### Features

* **#289:** add haveibeenpwned check ([#1253](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1253)) ([166dbd2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/166dbd2cc878e30c51538b919abc1aaea4465c45))
* Add `/v1/{check_email,bulk}` endpoints with throttle&concurrency ([#1537](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1537)) ([08522e4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/08522e4326bbcbc980cf501d5d994d0c17222561))
* Add `CheckEmailInput` setter `set_` prefix to differentiate with accessing fields ([#933](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/933)) ([276f656](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/276f6561e7a98af6415dbd4645d84cbe697b738e))
* Add `misc.is_b2c` field ([#1553](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1553)) ([14a6759](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/14a6759d805d2051a4a1e1d81588279cb9c85336))
* Add `smtp.error.description` field for human-readable description of error ([#1111](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1111)) ([43b47ea](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/43b47ea2b9250f2c6d58c8a0ec4340066169c169))
* Add AWS SQS support ([#1554](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1554)) ([92be54e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/92be54ebfe4a2d19101141f55e94fc8e9588ff95))
* Add debug information about each email verification ([#1391](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1391)) ([3ea6e66](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3ea6e6607735682dfca6ecfa27460650ac6e42d3))
* Add deprecated warning when running HTTP server ([#943](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/943)) ([e4b1570](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/e4b1570a8be5573f7394a3139f34ab021452cc3a))
* add email address normalisation ([#1206](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1206)) ([f8ec348](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/f8ec348883cd4f4a20a8acbb38d54b69e798222b))
* add Microsoft 365 HTTP API validation ([#1194](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1194)) ([5d3c49f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5d3c49f41ef1369efe2a9e63b24543e281ae0776))
* Add optional timeout on proxy (env var: `RCH__PROXY__TIMEOUT_MS`) ([#1595](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1595)) ([0e51eb6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0e51eb686dad6bd2ec827e785bf9c30ccc88cde1))
* Add possibility to set SMTP port ([#985](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/985)) ([cdabdf8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/cdabdf80e858908d6c33e1273dfdc1fef0f78d35))
* Add proxy field in SmtpDebug ([2f60a03](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2f60a03f25d56397eb54302b134730ef923d9105))
* Add proxy username/password ([#1057](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1057)) ([d9583c6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d9583c6ae0d3353a5135dd157999cf579b308d6d))
* Add skipped domains ([#1293](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1293)) ([29119fa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/29119fa72027c9830396bbdf3e90f08c0c89d7a7))
* Add SMTP retries to avoid greylisting ([#1041](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1041)) ([b451a1e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b451a1e93a6ccf025c78d56dee7439ad607c8507))
* Add suggestions for syntax errors ([#1192](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1192)) ([2d385f3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2d385f30f7a62ab2706599fbb89fb50275cffb5f))
* additional Gmail validation ([#1193](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1193)) ([49c8f5c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/49c8f5c3b4a3db04533d06d7267b0f15ebda3285))
* Allow multiple proxies ([#1562](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1562)) ([eed5a15](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/eed5a1536af37877f12eebab6481acaa6efa55c5))
* Allow user to define SMTP client security for TLS ([#1043](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1043)) ([bc722ff](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bc722ff1a9b30747308a3b3b5959d73e5e853292))
* **backend:** Add one simple retry on Unknown ([fcffc1a](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fcffc1a28bab990b0596ad8b66163e47a494191b))
* Break SmtpError into `{Helo,Connect,ConnectWithStream,MailFrom,RcptTo,Close}Error` ([#1055](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1055)) ([64e5193](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/64e5193c48a6bf4c080e79daeefd1c98dadffd5d))
* **core:** Add check for antispam MX records ([#1257](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1257)) ([c9771da](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/c9771da66c7869a4d0a255e2e2536f2863e8958c))
* **core:** Add check gravatar image ([#1188](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1188)) ([6a26035](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6a26035327ab681a65a4f4ba284e155f00680e89))
* **core:** Add domain-specific rules as JSON file ([#1347](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1347)) ([cab143c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/cab143c72889c585adbf041e9c248e57d0c4c4ca))
* **core:** Add Hotmail checks via headless password recovery ([#1165](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1165)) ([7517ed9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7517ed98ba966158deebba6a1a4745c931bfed18))
* **core:** Bump to 45s timeout for some domains ([#1348](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1348)) ([fda33a2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fda33a27441e2ccb1c4e97c0fc582abf25b1561f))
* **core:** Default Gmail checks to use API ([4304743](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/4304743fa93b6511857827afcdaa1fb9124bd62b))
* **core:** Fix disabled accts on hanmail.net ([#1339](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1339)) ([90393c8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/90393c8dda39267da7eb5efe6f112c8f25a593f4))
* **core:** Skip catch-all for known domains ([#1336](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1336)) ([c40a46c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/c40a46c4555129346bd9efa444a483bf25b679fe))
* **core:** Update async-smtp to 0.9 ([#1520](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1520)) ([297ce4f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/297ce4f11994b483faa015bebe4abf550eb77e11))
* **core:** Update default MAIL-FROM and HELO ([743a811](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/743a8111b4831ee19e7ac887c39a8da2775acd4c))
* Loop through all MX servers ([#1070](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1070)) ([11e6a06](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/11e6a06a67f5893b729c76d1a33667f83d63c836))
* Revert back to `check_email` input with single email ([#1150](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1150)) ([ce1ba53](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ce1ba5346849b578a0ed30b1d72096f15cfbc09d))
* Set default timeout to 10s ([#1251](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1251)) ([d04f84c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d04f84cc1e7b30e02d3717ab1af9f680cdb2c27f))
* Update parser.rs ([#1345](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1345)) ([8269f22](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8269f22f73214412f154927a908a7769d3f8b00c))
* Use opportunistic STARTTLS by default ([#1079](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1079)) ([54911f0](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/54911f0a8ec51e753f757878021e933609cff868))
* Yahoo account recovery via headless ([#1364](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1364)) ([6f0f12b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6f0f12b8cf528e819f8743f7e3c5f5e141c51559))


### Bug Fixes

* Add "utilisateur inconnu" in invalid parser ([#1594](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1594)) ([fb91653](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb9165303e2d7be59ed2fa4f0682e8592bc0c5e7))
* Add HoneyPot rule ([fb428ef](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb428ef42586641711dfd10190514ff5aa24583d))
* Add serde (De)Serialize to pub structs ([#931](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/931)) ([949475d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/949475dee4a1ed96e873688e7432c702eb30af62))
* Bring back `{yahoo,hotmailb2c}_verif_method` ([#1606](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1606)) ([3fbe520](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3fbe5200a3d8608fbd72c0f2a5917326c1f8ec91))
* **cli:** Update flags default values ([a4fe57e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/a4fe57e9ab89659e12182719ccb12fb2cdcb5f2e))
* **core:** Add more invalid parsing and improve logging ([#1156](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1156)) ([b5ae9f8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b5ae9f8ad910b77ad6a179ecb5d4b633011ed2f4))
* **core:** Clean up CheckEmailInput ([#1531](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1531)) ([b97b9ff](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b97b9ff9b91bdfbf18e5c0892559e87e7cd5e16c))
* **core:** Default SMTP timeout to 15 ([0d4fa4d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0d4fa4d8f662ecfd3fa2e0359322f324a8ef86db))
* **core:** Don't use headless on Microsoft 465 addresses ([#1196](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1196)) ([0c3c21d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0c3c21daf6ea79875835121fb86ab7c0c86d55eb))
* **core:** Fix default CheckEmailInput ([09215a1](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/09215a13ac3525861e6cd1dea3fc71c13dfffe52))
* **core:** Fix gmail test ([ea80690](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ea80690b4168485ed7e03f4e228a12e276d605b0))
* **core:** Fix hotmail headless option parsing ([6ddc3b9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6ddc3b96da0d01b02711d62873ad0d0df6bf1b33))
* **core:** Fix hotmail headless with authenticator ([51cdb2e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/51cdb2e3c13a433fff92f1d3dcf1bfcb90f6ce7b))
* **core:** Fix hotmail/outlook checks ([5e4bf16](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5e4bf16e75e01ba17dd9022934359c9d03f3b0c8))
* **core:** Fix MX random record selection ([#1263](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1263)) ([9fae593](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9fae593b8590ad5efb3e7d16bbd25cc05c228cb9))
* **core:** Headless check for Microsoft365 too ([#1346](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1346)) ([682cc2d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/682cc2d96b93d73f3fca3ba11f03800477c8fb9e))
* **core:** Improve invalid parser ([#1166](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1166)) ([bb46004](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bb460046bf1cb031fee706d836c8a737157f803c))
* **core:** Improve parser and headless hotmail runner ([#1167](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1167)) ([0de33a5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0de33a5f265105a769c7ca6125df0fd4f88b89e2))
* **core:** Improve parser from Sentry errors ([fbaf588](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fbaf58824a339e546d50c2125a459161769dda6e))
* **core:** Improve parser's `is_invalid` ([#1159](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1159)) ([ec1c4d5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ec1c4d5e5d4c94d75d255a0699402f75eb29f7ab))
* **core:** More robust Hotmail invalid check ([ee741f4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ee741f4570050f559395e687da64c64ff9046afb))
* **core:** No sandbox in headless Hotmail check ([0590438](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0590438310f3c052b2748a8c408e0d8dbfb777b7))
* **core:** Prefer empty MX lookup when Err NoRecordsFound ([#1409](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1409)) ([d4b5ef9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/d4b5ef9696a8c3ff0eaad2d3b5321437bd2a4df3))
* **core:** Remove antispam check ([#1337](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1337)) ([06f18ed](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/06f18edf7aee5640b3725feedfa7b7f213da83a8))
* **core:** Use semver in sentry ([03e6c97](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/03e6c97a7f842b115b367ca942119496d8400024))
* **core:** Use Smtp for Gmail by default ([8e79884](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8e79884314f0c1eec5a7964fa686e2c60e7d2209))
* **core:** Use tagged enum representation ([ffde851](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ffde851068798adc3372d843a916a121b5caeccb))
* **core:** Yahoo add back IDENTIFIER_EXISTS ([2b63556](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2b635564efb37b0aa891bbba77244e6cf2d611bb))
* **core:** yahoo api changes: yid is userId now, sessionIndex is required and fo… ([#1314](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1314)) ([0209111](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/02091115026520596fc5b4b2a6757169e91cba15))
* Don't auto-fetch Chrome, install in Docker ([84fcc0d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/84fcc0de40567126ce3a385934086450c3a89ccf))
* Don't show proxy full info in logs ([2668ce1](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/2668ce14418076b00f36f18a370070ac1f3754bf))
* Fix `has_full_inbox` check too lenient ([93de444](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/93de444dfa7c6d66061570115be8f53f0647c431))
* Fix duplicate `yahoo_verif_method` field in default() inputs ([#1428](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1428)) ([b7c51d5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b7c51d5caaf21140c174cb419aedaf8fe752f817))
* Fix parsing some invalid emails ([cb65c0f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/cb65c0f4767b2f163f48054652f7652b6d0b6043))
* Fix version in logs ([fa6be78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6be7867abae981b0d82fde24e0310b9759ab1f))
* Improve logging, add retries for Yahoo headless, switch to rustls ([#1549](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1549)) ([b1377db](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b1377db2b32155d766a09a76864fc9b0990833e6))
* Only do headless for non-365 hotmail emails ([1c52bdc](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/1c52bdc75fb201f2e54c62d5f67f50a56c57cb83))
* Put Smtp debug details in Debug struct ([5b71ca5](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5b71ca59b6fab18263348aeafc7a895b7f4b8076))
* Remove local_ip retrieval ([ff8e599](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ff8e5998f8b88954b4104f9251d1331542dbb182))
* Revert back to using lowest-priority MX record ([#1578](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1578)) ([60468b3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/60468b3f533491a0dff6a42e7096f34ece19896c))
* Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579)) ([3388163](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3388163d03b66ba92455be8404441e8555a9d53c))
* split Microsoft 365/Hotmail functionality ([#1204](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1204)) ([e987b13](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/e987b13a5ccd98d28fb756f1bf41427c337750c4))
* Switch back to upstream fast-socks ([#1164](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1164)) ([db356f1](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/db356f19374843ca135de8ebd8a6c34bfeb017a8))
* Syntax also check using using `mailchecker` ([8385bec](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/8385bec6fedc0912881800442bffda5b33c2f394))
* TLS accept unsafe ([778692b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/778692bce760c0a1e1201dd3e11b41e7ccb7e2e8))
* Use async_std_resolver::resolver_from_system_conf ([#982](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/982)) ([376c3b0](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/376c3b0d4743ccc60a1df2a9fa3e9f2f5cd68178))
* Use chromedriver instead of gecko for parallel requests ([e282e28](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/e282e28aeb7259d800f7faad97173c3a216095a4))
* Use proxy auth if provided for api checks ([#1416](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1416)) ([8340514](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/83405146f04a9c8b718b63635e74cd70decf4931))
* Use std::default for deriving ([#1015](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1015)) ([03720f0](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/03720f027fd68d5ea5ae538aa567a621f4a65fe3))
* Use TLS when available ([#964](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/964)) ([aed11d2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/aed11d2e15b6b7688ecaf856824ca6effbb5d21b))


### Reverts

* "Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579))" ([56e7838](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/56e7838f28067b05b58f1fcd166368a915aafbbc))
* **backend:** Bring back the sqlxmq-based bulk verification ([#1477](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1477)) ([322ad4e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/322ad4e4b53d534a8ae6461f3d3383d67b219b5d))


### Miscellaneous Chores

* Rename all VerifyMethod to VerifMethod ([9f9607d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9f9607d35478a1051dde56812f8914ff75d4c5ac))


### Code Refactoring

* Change RUST_LOG target to `reacher` ([#1152](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1152)) ([7e87be2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7e87be26f1e35a6936bfc967c872cd42b93fd256))
* Remove HTTP backend from CLI ([#1151](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1151)) ([7184372](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/71843720c9b87fa0e43fa482a35ef074435bf562))
* Use config-rs instead of env vars ([#1530](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1530)) ([bcd2dc8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bcd2dc867b7dc2bdaeb70097fd14109c2a40da17))
* Use verify method for known providers ([#1366](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1366)) ([5ca4dfa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5ca4dfa5ec38fba0ec7cfb052106da8d6af4df44))
