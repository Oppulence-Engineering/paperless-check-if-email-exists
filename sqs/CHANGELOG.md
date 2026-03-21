# Changelog

## [4.1.0](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/compare/v4.0.0...v4.1.0) (2026-03-21)


### Features

* Phase 1 multi-tenant platform with onboarding endpoint ([6a33c56](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/commit/6a33c56d6364ce37097d7f1a928826dc02ea355c))

## [4.0.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/v3.0.2...v4.0.0) (2025-12-12)


### ⚠ BREAKING CHANGES

* 

### Features

* Add AWS SQS support ([#1554](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1554)) ([92be54e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/92be54ebfe4a2d19101141f55e94fc8e9588ff95))
* Add optional timeout on proxy (env var: `RCH__PROXY__TIMEOUT_MS`) ([#1595](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1595)) ([0e51eb6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0e51eb686dad6bd2ec827e785bf9c30ccc88cde1))
* Allow multiple proxies ([#1562](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1562)) ([eed5a15](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/eed5a1536af37877f12eebab6481acaa6efa55c5))


### Bug Fixes

* Add "utilisateur inconnu" in invalid parser ([#1594](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1594)) ([fb91653](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb9165303e2d7be59ed2fa4f0682e8592bc0c5e7))
* Bring back `{yahoo,hotmailb2c}_verif_method` ([#1606](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1606)) ([3fbe520](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3fbe5200a3d8608fbd72c0f2a5917326c1f8ec91))
* Fix version in logs ([fa6be78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6be7867abae981b0d82fde24e0310b9759ab1f))
* Revert back to using lowest-priority MX record ([#1578](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1578)) ([60468b3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/60468b3f533491a0dff6a42e7096f34ece19896c))
* Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579)) ([3388163](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3388163d03b66ba92455be8404441e8555a9d53c))


### Reverts

* "Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579))" ([56e7838](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/56e7838f28067b05b58f1fcd166368a915aafbbc))

## [3.0.1](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/v3.0.0...v3.0.1) (2025-12-04)

## [3.0.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/v2.0.0...v3.0.0) (2025-11-28)


### ⚠ BREAKING CHANGES

* 

### Features

* Add AWS SQS support ([#1554](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1554)) ([92be54e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/92be54ebfe4a2d19101141f55e94fc8e9588ff95))
* Add optional timeout on proxy (env var: `RCH__PROXY__TIMEOUT_MS`) ([#1595](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1595)) ([0e51eb6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0e51eb686dad6bd2ec827e785bf9c30ccc88cde1))
* Allow multiple proxies ([#1562](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1562)) ([eed5a15](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/eed5a1536af37877f12eebab6481acaa6efa55c5))


### Bug Fixes

* Add "utilisateur inconnu" in invalid parser ([#1594](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1594)) ([fb91653](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb9165303e2d7be59ed2fa4f0682e8592bc0c5e7))
* Bring back `{yahoo,hotmailb2c}_verif_method` ([#1606](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1606)) ([3fbe520](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3fbe5200a3d8608fbd72c0f2a5917326c1f8ec91))
* Fix version in logs ([fa6be78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6be7867abae981b0d82fde24e0310b9759ab1f))
* Revert back to using lowest-priority MX record ([#1578](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1578)) ([60468b3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/60468b3f533491a0dff6a42e7096f34ece19896c))
* Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579)) ([3388163](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3388163d03b66ba92455be8404441e8555a9d53c))


### Reverts

* "Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579))" ([56e7838](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/56e7838f28067b05b58f1fcd166368a915aafbbc))

## [2.0.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/sqs-v1.0.0...sqs-v2.0.0) (2025-11-28)


### ⚠ BREAKING CHANGES

* 

### Features

* Add AWS SQS support ([#1554](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1554)) ([92be54e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/92be54ebfe4a2d19101141f55e94fc8e9588ff95))
* Add optional timeout on proxy (env var: `RCH__PROXY__TIMEOUT_MS`) ([#1595](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1595)) ([0e51eb6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0e51eb686dad6bd2ec827e785bf9c30ccc88cde1))
* Allow multiple proxies ([#1562](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1562)) ([eed5a15](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/eed5a1536af37877f12eebab6481acaa6efa55c5))


### Bug Fixes

* Add "utilisateur inconnu" in invalid parser ([#1594](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1594)) ([fb91653](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb9165303e2d7be59ed2fa4f0682e8592bc0c5e7))
* Bring back `{yahoo,hotmailb2c}_verif_method` ([#1606](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1606)) ([3fbe520](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3fbe5200a3d8608fbd72c0f2a5917326c1f8ec91))
* Fix version in logs ([fa6be78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6be7867abae981b0d82fde24e0310b9759ab1f))
* Revert back to using lowest-priority MX record ([#1578](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1578)) ([60468b3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/60468b3f533491a0dff6a42e7096f34ece19896c))
* Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579)) ([3388163](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3388163d03b66ba92455be8404441e8555a9d53c))


### Reverts

* "Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579))" ([56e7838](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/56e7838f28067b05b58f1fcd166368a915aafbbc))

## [1.0.0](https://github.com/Oppulence-Engineering/check-if-email-exists/compare/v0.11.7...v1.0.0) (2025-11-28)


### ⚠ BREAKING CHANGES

* 

### Features

* Add AWS SQS support ([#1554](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1554)) ([92be54e](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/92be54ebfe4a2d19101141f55e94fc8e9588ff95))
* Add optional timeout on proxy (env var: `RCH__PROXY__TIMEOUT_MS`) ([#1595](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1595)) ([0e51eb6](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/0e51eb686dad6bd2ec827e785bf9c30ccc88cde1))
* Allow multiple proxies ([#1562](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1562)) ([eed5a15](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/eed5a1536af37877f12eebab6481acaa6efa55c5))


### Bug Fixes

* Add "utilisateur inconnu" in invalid parser ([#1594](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1594)) ([fb91653](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fb9165303e2d7be59ed2fa4f0682e8592bc0c5e7))
* Bring back `{yahoo,hotmailb2c}_verif_method` ([#1606](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1606)) ([3fbe520](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3fbe5200a3d8608fbd72c0f2a5917326c1f8ec91))
* Fix version in logs ([fa6be78](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/fa6be7867abae981b0d82fde24e0310b9759ab1f))
* Revert back to using lowest-priority MX record ([#1578](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1578)) ([60468b3](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/60468b3f533491a0dff6a42e7096f34ece19896c))
* Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579)) ([3388163](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/3388163d03b66ba92455be8404441e8555a9d53c))


### Reverts

* "Show thread ID in logs ([#1579](https://github.com/Oppulence-Engineering/check-if-email-exists/issues/1579))" ([56e7838](https://github.com/Oppulence-Engineering/check-if-email-exists/commit/56e7838f28067b05b58f1fcd166368a915aafbbc))
