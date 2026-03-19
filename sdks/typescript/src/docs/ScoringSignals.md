# ScoringSignals


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**has_domain_suggestion** | **boolean** |  | [default to undefined]
**has_mx_records** | **boolean** |  | [default to undefined]
**is_disposable** | **boolean** |  | [default to undefined]
**is_free_provider** | **boolean** |  | [default to undefined]
**is_role_account** | **boolean** |  | [default to undefined]
**reachable** | [**Reachable**](Reachable.md) |  | [default to undefined]
**smtp_can_connect** | **boolean** |  | [default to undefined]
**smtp_error** | **boolean** |  | [default to undefined]
**smtp_has_full_inbox** | **boolean** |  | [default to undefined]
**smtp_is_catch_all** | **boolean** |  | [default to undefined]
**smtp_is_deliverable** | **boolean** |  | [default to undefined]
**smtp_is_disabled** | **boolean** |  | [default to undefined]
**valid_syntax** | **boolean** |  | [default to undefined]

## Example

```typescript
import { ScoringSignals } from '@oppulence/reacher-sdk';

const instance: ScoringSignals = {
    has_domain_suggestion,
    has_mx_records,
    is_disposable,
    is_free_provider,
    is_role_account,
    reachable,
    smtp_can_connect,
    smtp_error,
    smtp_has_full_inbox,
    smtp_is_catch_all,
    smtp_is_deliverable,
    smtp_is_disabled,
    valid_syntax,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
