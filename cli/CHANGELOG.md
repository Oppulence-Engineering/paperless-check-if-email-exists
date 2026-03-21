# Changelog

## [4.0.1](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/compare/v4.0.0...v4.0.1) (2026-03-21)

## [4.0.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/v3.0.2...v4.0.0) (2025-12-12)


### ⚠ BREAKING CHANGES

* 
* 
* **core:** 
* 
* Rename all VerifyMethod to VerifMethod
* For Hotmail, Gmail and Yahoo addresses, the `*_use_api` and `*_use_headless` parameters have been removed and replaced with a `*VerifyMethod`, an enum which can take value Api, Headless or Smtp. If using headless, pass a webdriver address to env variable RCH_WEBDRIVER_ADDR.
* `input.hotmail_use_headless` is now a bool instead of a string. Pass the webdriver address as an environment variable `RCH_WEBDRIVER_ADDR` now.
* 
* Remove HTTP backend from CLI ([#1151](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1151))

### Features

* **#289:** add haveibeenpwned check ([#1253](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1253)) ([166dbd2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/166dbd2cc878e30c51538b919abc1aaea4465c45))
* Add `/v1/{check_email,bulk}` endpoints with throttle&concurrency ([#1537](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1537)) ([08522e4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/08522e4326bbcbc980cf501d5d994d0c17222561))
* Add `misc.is_b2c` field ([#1553](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1553)) ([14a6759](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/14a6759d805d2051a4a1e1d81588279cb9c85336))
* Add AWS SQS support ([#1554](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1554)) ([92be54e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/92be54ebfe4a2d19101141f55e94fc8e9588ff95))
* add Microsoft 365 HTTP API validation ([#1194](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1194)) ([5d3c49f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5d3c49f41ef1369efe2a9e63b24543e281ae0776))
* Add optional timeout on proxy (env var: `RCH__PROXY__TIMEOUT_MS`) ([#1595](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1595)) ([0e51eb6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0e51eb686dad6bd2ec827e785bf9c30ccc88cde1))
* additional Gmail validation ([#1193](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1193)) ([49c8f5c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/49c8f5c3b4a3db04533d06d7267b0f15ebda3285))
* Allow multiple proxies ([#1562](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1562)) ([eed5a15](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/eed5a1536af37877f12eebab6481acaa6efa55c5))
* **core:** Add check gravatar image ([#1188](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1188)) ([6a26035](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6a26035327ab681a65a4f4ba284e155f00680e89))
* **core:** Add Hotmail checks via headless password recovery ([#1165](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1165)) ([7517ed9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7517ed98ba966158deebba6a1a4745c931bfed18))
* Move `backend` code to this repo ([#1138](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1138)) ([0dc6053](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0dc60531d26efb217137347ef2b6aaf678d94238))
* Revert back to `check_email` input with single email ([#1150](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1150)) ([ce1ba53](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ce1ba5346849b578a0ed30b1d72096f15cfbc09d))
* Yahoo account recovery via headless ([#1364](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1364)) ([6f0f12b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6f0f12b8cf528e819f8743f7e3c5f5e141c51559))


### Bug Fixes

* Add "utilisateur inconnu" in invalid parser ([#1594](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1594)) ([fb91653](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb9165303e2d7be59ed2fa4f0682e8592bc0c5e7))
* Bring back `{yahoo,hotmailb2c}_verif_method` ([#1606](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1606)) ([3fbe520](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3fbe5200a3d8608fbd72c0f2a5917326c1f8ec91))
* **cli:** Update flags default values ([a4fe57e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/a4fe57e9ab89659e12182719ccb12fb2cdcb5f2e))
* **core:** Clean up CheckEmailInput ([#1531](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1531)) ([b97b9ff](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b97b9ff9b91bdfbf18e5c0892559e87e7cd5e16c))
* **core:** Fix hotmail headless with authenticator ([51cdb2e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/51cdb2e3c13a433fff92f1d3dcf1bfcb90f6ce7b))
* **core:** Fix MX random record selection ([#1263](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1263)) ([9fae593](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9fae593b8590ad5efb3e7d16bbd25cc05c228cb9))
* Fix version in logs ([fa6be78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6be7867abae981b0d82fde24e0310b9759ab1f))
* Fixed inverted hello-name and from-email in CLI ([#1565](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1565)) ([a53561e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/a53561e087593ccc887b45943f54855b9cc6ae85))
* Improve logging, add retries for Yahoo headless, switch to rustls ([#1549](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1549)) ([b1377db](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b1377db2b32155d766a09a76864fc9b0990833e6))
* Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579)) ([3388163](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3388163d03b66ba92455be8404441e8555a9d53c))


### Reverts

* "Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579))" ([56e7838](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/56e7838f28067b05b58f1fcd166368a915aafbbc))


### Miscellaneous Chores

* Rename all VerifyMethod to VerifMethod ([9f9607d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9f9607d35478a1051dde56812f8914ff75d4c5ac))


### Code Refactoring

* Remove HTTP backend from CLI ([#1151](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1151)) ([7184372](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/71843720c9b87fa0e43fa482a35ef074435bf562))
* Use config-rs instead of env vars ([#1530](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1530)) ([bcd2dc8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bcd2dc867b7dc2bdaeb70097fd14109c2a40da17))
* Use verify method for known providers ([#1366](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1366)) ([5ca4dfa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5ca4dfa5ec38fba0ec7cfb052106da8d6af4df44))

## [3.0.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/v2.0.0...v3.0.0) (2025-11-28)


### ⚠ BREAKING CHANGES

* 
* 
* **core:** 
* 
* Rename all VerifyMethod to VerifMethod
* For Hotmail, Gmail and Yahoo addresses, the `*_use_api` and `*_use_headless` parameters have been removed and replaced with a `*VerifyMethod`, an enum which can take value Api, Headless or Smtp. If using headless, pass a webdriver address to env variable RCH_WEBDRIVER_ADDR.
* `input.hotmail_use_headless` is now a bool instead of a string. Pass the webdriver address as an environment variable `RCH_WEBDRIVER_ADDR` now.
* 
* Remove HTTP backend from CLI ([#1151](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1151))

### Features

* **#289:** add haveibeenpwned check ([#1253](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1253)) ([166dbd2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/166dbd2cc878e30c51538b919abc1aaea4465c45))
* Add `/v1/{check_email,bulk}` endpoints with throttle&concurrency ([#1537](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1537)) ([08522e4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/08522e4326bbcbc980cf501d5d994d0c17222561))
* Add `misc.is_b2c` field ([#1553](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1553)) ([14a6759](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/14a6759d805d2051a4a1e1d81588279cb9c85336))
* Add AWS SQS support ([#1554](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1554)) ([92be54e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/92be54ebfe4a2d19101141f55e94fc8e9588ff95))
* add Microsoft 365 HTTP API validation ([#1194](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1194)) ([5d3c49f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5d3c49f41ef1369efe2a9e63b24543e281ae0776))
* Add optional timeout on proxy (env var: `RCH__PROXY__TIMEOUT_MS`) ([#1595](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1595)) ([0e51eb6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0e51eb686dad6bd2ec827e785bf9c30ccc88cde1))
* additional Gmail validation ([#1193](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1193)) ([49c8f5c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/49c8f5c3b4a3db04533d06d7267b0f15ebda3285))
* Allow multiple proxies ([#1562](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1562)) ([eed5a15](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/eed5a1536af37877f12eebab6481acaa6efa55c5))
* **core:** Add check gravatar image ([#1188](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1188)) ([6a26035](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6a26035327ab681a65a4f4ba284e155f00680e89))
* **core:** Add Hotmail checks via headless password recovery ([#1165](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1165)) ([7517ed9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7517ed98ba966158deebba6a1a4745c931bfed18))
* Move `backend` code to this repo ([#1138](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1138)) ([0dc6053](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0dc60531d26efb217137347ef2b6aaf678d94238))
* Revert back to `check_email` input with single email ([#1150](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1150)) ([ce1ba53](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ce1ba5346849b578a0ed30b1d72096f15cfbc09d))
* Yahoo account recovery via headless ([#1364](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1364)) ([6f0f12b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6f0f12b8cf528e819f8743f7e3c5f5e141c51559))


### Bug Fixes

* Add "utilisateur inconnu" in invalid parser ([#1594](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1594)) ([fb91653](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb9165303e2d7be59ed2fa4f0682e8592bc0c5e7))
* Bring back `{yahoo,hotmailb2c}_verif_method` ([#1606](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1606)) ([3fbe520](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3fbe5200a3d8608fbd72c0f2a5917326c1f8ec91))
* **cli:** Update flags default values ([a4fe57e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/a4fe57e9ab89659e12182719ccb12fb2cdcb5f2e))
* **core:** Clean up CheckEmailInput ([#1531](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1531)) ([b97b9ff](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b97b9ff9b91bdfbf18e5c0892559e87e7cd5e16c))
* **core:** Fix hotmail headless with authenticator ([51cdb2e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/51cdb2e3c13a433fff92f1d3dcf1bfcb90f6ce7b))
* **core:** Fix MX random record selection ([#1263](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1263)) ([9fae593](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9fae593b8590ad5efb3e7d16bbd25cc05c228cb9))
* Fix version in logs ([fa6be78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6be7867abae981b0d82fde24e0310b9759ab1f))
* Fixed inverted hello-name and from-email in CLI ([#1565](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1565)) ([a53561e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/a53561e087593ccc887b45943f54855b9cc6ae85))
* Improve logging, add retries for Yahoo headless, switch to rustls ([#1549](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1549)) ([b1377db](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b1377db2b32155d766a09a76864fc9b0990833e6))
* Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579)) ([3388163](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3388163d03b66ba92455be8404441e8555a9d53c))


### Reverts

* "Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579))" ([56e7838](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/56e7838f28067b05b58f1fcd166368a915aafbbc))


### Miscellaneous Chores

* Rename all VerifyMethod to VerifMethod ([9f9607d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9f9607d35478a1051dde56812f8914ff75d4c5ac))


### Code Refactoring

* Remove HTTP backend from CLI ([#1151](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1151)) ([7184372](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/71843720c9b87fa0e43fa482a35ef074435bf562))
* Use config-rs instead of env vars ([#1530](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1530)) ([bcd2dc8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bcd2dc867b7dc2bdaeb70097fd14109c2a40da17))
* Use verify method for known providers ([#1366](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1366)) ([5ca4dfa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5ca4dfa5ec38fba0ec7cfb052106da8d6af4df44))

## [2.0.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/cli-v1.0.0...cli-v2.0.0) (2025-11-28)


### ⚠ BREAKING CHANGES

* 
* 
* **core:** 
* 
* Rename all VerifyMethod to VerifMethod
* For Hotmail, Gmail and Yahoo addresses, the `*_use_api` and `*_use_headless` parameters have been removed and replaced with a `*VerifyMethod`, an enum which can take value Api, Headless or Smtp. If using headless, pass a webdriver address to env variable RCH_WEBDRIVER_ADDR.
* `input.hotmail_use_headless` is now a bool instead of a string. Pass the webdriver address as an environment variable `RCH_WEBDRIVER_ADDR` now.
* 
* Remove HTTP backend from CLI ([#1151](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1151))

### Features

* **#289:** add haveibeenpwned check ([#1253](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1253)) ([166dbd2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/166dbd2cc878e30c51538b919abc1aaea4465c45))
* Add `/v1/{check_email,bulk}` endpoints with throttle&concurrency ([#1537](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1537)) ([08522e4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/08522e4326bbcbc980cf501d5d994d0c17222561))
* Add `misc.is_b2c` field ([#1553](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1553)) ([14a6759](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/14a6759d805d2051a4a1e1d81588279cb9c85336))
* Add AWS SQS support ([#1554](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1554)) ([92be54e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/92be54ebfe4a2d19101141f55e94fc8e9588ff95))
* add Microsoft 365 HTTP API validation ([#1194](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1194)) ([5d3c49f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5d3c49f41ef1369efe2a9e63b24543e281ae0776))
* Add optional timeout on proxy (env var: `RCH__PROXY__TIMEOUT_MS`) ([#1595](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1595)) ([0e51eb6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0e51eb686dad6bd2ec827e785bf9c30ccc88cde1))
* additional Gmail validation ([#1193](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1193)) ([49c8f5c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/49c8f5c3b4a3db04533d06d7267b0f15ebda3285))
* Allow multiple proxies ([#1562](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1562)) ([eed5a15](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/eed5a1536af37877f12eebab6481acaa6efa55c5))
* **core:** Add check gravatar image ([#1188](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1188)) ([6a26035](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6a26035327ab681a65a4f4ba284e155f00680e89))
* **core:** Add Hotmail checks via headless password recovery ([#1165](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1165)) ([7517ed9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7517ed98ba966158deebba6a1a4745c931bfed18))
* Move `backend` code to this repo ([#1138](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1138)) ([0dc6053](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0dc60531d26efb217137347ef2b6aaf678d94238))
* Revert back to `check_email` input with single email ([#1150](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1150)) ([ce1ba53](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ce1ba5346849b578a0ed30b1d72096f15cfbc09d))
* Yahoo account recovery via headless ([#1364](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1364)) ([6f0f12b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6f0f12b8cf528e819f8743f7e3c5f5e141c51559))


### Bug Fixes

* Add "utilisateur inconnu" in invalid parser ([#1594](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1594)) ([fb91653](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb9165303e2d7be59ed2fa4f0682e8592bc0c5e7))
* Bring back `{yahoo,hotmailb2c}_verif_method` ([#1606](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1606)) ([3fbe520](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3fbe5200a3d8608fbd72c0f2a5917326c1f8ec91))
* **cli:** Update flags default values ([a4fe57e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/a4fe57e9ab89659e12182719ccb12fb2cdcb5f2e))
* **core:** Clean up CheckEmailInput ([#1531](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1531)) ([b97b9ff](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b97b9ff9b91bdfbf18e5c0892559e87e7cd5e16c))
* **core:** Fix hotmail headless with authenticator ([51cdb2e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/51cdb2e3c13a433fff92f1d3dcf1bfcb90f6ce7b))
* **core:** Fix MX random record selection ([#1263](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1263)) ([9fae593](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9fae593b8590ad5efb3e7d16bbd25cc05c228cb9))
* Fix version in logs ([fa6be78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6be7867abae981b0d82fde24e0310b9759ab1f))
* Fixed inverted hello-name and from-email in CLI ([#1565](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1565)) ([a53561e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/a53561e087593ccc887b45943f54855b9cc6ae85))
* Improve logging, add retries for Yahoo headless, switch to rustls ([#1549](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1549)) ([b1377db](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b1377db2b32155d766a09a76864fc9b0990833e6))
* Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579)) ([3388163](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3388163d03b66ba92455be8404441e8555a9d53c))


### Reverts

* "Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579))" ([56e7838](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/56e7838f28067b05b58f1fcd166368a915aafbbc))


### Miscellaneous Chores

* Rename all VerifyMethod to VerifMethod ([9f9607d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9f9607d35478a1051dde56812f8914ff75d4c5ac))


### Code Refactoring

* Remove HTTP backend from CLI ([#1151](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1151)) ([7184372](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/71843720c9b87fa0e43fa482a35ef074435bf562))
* Use config-rs instead of env vars ([#1530](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1530)) ([bcd2dc8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bcd2dc867b7dc2bdaeb70097fd14109c2a40da17))
* Use verify method for known providers ([#1366](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1366)) ([5ca4dfa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5ca4dfa5ec38fba0ec7cfb052106da8d6af4df44))

## [1.0.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/v0.11.7...v1.0.0) (2025-11-28)


### ⚠ BREAKING CHANGES

* 
* 
* **core:** 
* 
* Rename all VerifyMethod to VerifMethod
* For Hotmail, Gmail and Yahoo addresses, the `*_use_api` and `*_use_headless` parameters have been removed and replaced with a `*VerifyMethod`, an enum which can take value Api, Headless or Smtp. If using headless, pass a webdriver address to env variable RCH_WEBDRIVER_ADDR.
* `input.hotmail_use_headless` is now a bool instead of a string. Pass the webdriver address as an environment variable `RCH_WEBDRIVER_ADDR` now.
* 
* Remove HTTP backend from CLI ([#1151](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1151))

### Features

* **#289:** add haveibeenpwned check ([#1253](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1253)) ([166dbd2](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/166dbd2cc878e30c51538b919abc1aaea4465c45))
* Add `/v1/{check_email,bulk}` endpoints with throttle&concurrency ([#1537](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1537)) ([08522e4](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/08522e4326bbcbc980cf501d5d994d0c17222561))
* Add `misc.is_b2c` field ([#1553](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1553)) ([14a6759](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/14a6759d805d2051a4a1e1d81588279cb9c85336))
* Add AWS SQS support ([#1554](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1554)) ([92be54e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/92be54ebfe4a2d19101141f55e94fc8e9588ff95))
* add Microsoft 365 HTTP API validation ([#1194](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1194)) ([5d3c49f](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5d3c49f41ef1369efe2a9e63b24543e281ae0776))
* Add optional timeout on proxy (env var: `RCH__PROXY__TIMEOUT_MS`) ([#1595](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1595)) ([0e51eb6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0e51eb686dad6bd2ec827e785bf9c30ccc88cde1))
* additional Gmail validation ([#1193](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1193)) ([49c8f5c](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/49c8f5c3b4a3db04533d06d7267b0f15ebda3285))
* Allow multiple proxies ([#1562](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1562)) ([eed5a15](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/eed5a1536af37877f12eebab6481acaa6efa55c5))
* **core:** Add check gravatar image ([#1188](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1188)) ([6a26035](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6a26035327ab681a65a4f4ba284e155f00680e89))
* **core:** Add Hotmail checks via headless password recovery ([#1165](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1165)) ([7517ed9](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/7517ed98ba966158deebba6a1a4745c931bfed18))
* Move `backend` code to this repo ([#1138](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1138)) ([0dc6053](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0dc60531d26efb217137347ef2b6aaf678d94238))
* Revert back to `check_email` input with single email ([#1150](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1150)) ([ce1ba53](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/ce1ba5346849b578a0ed30b1d72096f15cfbc09d))
* Yahoo account recovery via headless ([#1364](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1364)) ([6f0f12b](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/6f0f12b8cf528e819f8743f7e3c5f5e141c51559))


### Bug Fixes

* Add "utilisateur inconnu" in invalid parser ([#1594](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1594)) ([fb91653](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb9165303e2d7be59ed2fa4f0682e8592bc0c5e7))
* Bring back `{yahoo,hotmailb2c}_verif_method` ([#1606](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1606)) ([3fbe520](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3fbe5200a3d8608fbd72c0f2a5917326c1f8ec91))
* **cli:** Update flags default values ([a4fe57e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/a4fe57e9ab89659e12182719ccb12fb2cdcb5f2e))
* **core:** Clean up CheckEmailInput ([#1531](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1531)) ([b97b9ff](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b97b9ff9b91bdfbf18e5c0892559e87e7cd5e16c))
* **core:** Fix hotmail headless with authenticator ([51cdb2e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/51cdb2e3c13a433fff92f1d3dcf1bfcb90f6ce7b))
* **core:** Fix MX random record selection ([#1263](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1263)) ([9fae593](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9fae593b8590ad5efb3e7d16bbd25cc05c228cb9))
* Fix version in logs ([fa6be78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6be7867abae981b0d82fde24e0310b9759ab1f))
* Fixed inverted hello-name and from-email in CLI ([#1565](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1565)) ([a53561e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/a53561e087593ccc887b45943f54855b9cc6ae85))
* Improve logging, add retries for Yahoo headless, switch to rustls ([#1549](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1549)) ([b1377db](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/b1377db2b32155d766a09a76864fc9b0990833e6))
* Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579)) ([3388163](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3388163d03b66ba92455be8404441e8555a9d53c))


### Reverts

* "Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579))" ([56e7838](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/56e7838f28067b05b58f1fcd166368a915aafbbc))


### Miscellaneous Chores

* Rename all VerifyMethod to VerifMethod ([9f9607d](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/9f9607d35478a1051dde56812f8914ff75d4c5ac))


### Code Refactoring

* Remove HTTP backend from CLI ([#1151](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1151)) ([7184372](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/71843720c9b87fa0e43fa482a35ef074435bf562))
* Use config-rs instead of env vars ([#1530](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1530)) ([bcd2dc8](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/bcd2dc867b7dc2bdaeb70097fd14109c2a40da17))
* Use verify method for known providers ([#1366](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1366)) ([5ca4dfa](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/5ca4dfa5ec38fba0ec7cfb052106da8d6af4df44))
