# ScoringSignals


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**has_domain_suggestion** | **boolean** |  | [required]
**has_mx_records** | **boolean** |  | [required]
**is_disposable** | **boolean** |  | [required]
**is_free_provider** | **boolean** |  | [required]
**is_role_account** | **boolean** |  | [required]
**is_spam_trap_domain** | **boolean** |  | [required]
**reachable** | [**Reachable**](Reachable.md) |  | [required]
**smtp_can_connect** | **boolean** |  | [required]
**smtp_error** | **boolean** |  | [required]
**smtp_has_full_inbox** | **boolean** |  | [required]
**smtp_is_catch_all** | **boolean** |  | [required]
**smtp_is_deliverable** | **boolean** |  | [required]
**smtp_is_disabled** | **boolean** |  | [required]
**valid_syntax** | **boolean** |  | [required]

## Example

```typescript
import { ScoringSignals } from '@oppulence/reacher-sdk';

const instance: ScoringSignals = {
    has_domain_suggestion: true,
    has_mx_records: true,
    is_disposable: true,
    is_free_provider: true,
    is_role_account: true,
    is_spam_trap_domain: true,
    reachable: {} as any,
    smtp_can_connect: true,
    smtp_error: true,
    smtp_has_full_inbox: true,
    smtp_is_catch_all: true,
    smtp_is_deliverable: true,
    smtp_is_disabled: true,
    valid_syntax: true,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
