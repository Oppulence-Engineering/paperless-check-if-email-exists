# EmailScore


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**age_days** | **number** |  | [optional] [default to undefined]
**catch_all_severity** | **string** | Severity tier for catch-all domains (low&#x3D;free provider, high&#x3D;corporate) | [optional] [default to undefined]
**category** | [**EmailCategory**](EmailCategory.md) |  | [default to undefined]
**domain_suggestion** | **string** | Suggested corrected email when a likely domain typo is detected | [optional] [default to undefined]
**freshness** | [**Freshness**](Freshness.md) |  | [optional] [default to undefined]
**normalized_email** | **string** | Canonical form of the email after alias/plus-address normalization | [optional] [default to undefined]
**reason_codes** | [**Array&lt;ReasonCode&gt;**](ReasonCode.md) |  | [default to undefined]
**safe_to_send** | **boolean** |  | [default to undefined]
**score** | **number** |  | [default to undefined]
**signals** | [**ScoringSignals**](ScoringSignals.md) |  | [default to undefined]
**sub_reason** | [**SubReason**](SubReason.md) |  | [default to undefined]
**verified_at** | **string** |  | [optional] [default to undefined]

## Example

```typescript
import { EmailScore } from '@oppulence/reacher-sdk';

const instance: EmailScore = {
    age_days,
    catch_all_severity,
    category,
    domain_suggestion,
    freshness,
    normalized_email,
    reason_codes,
    safe_to_send,
    score,
    signals,
    sub_reason,
    verified_at,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
