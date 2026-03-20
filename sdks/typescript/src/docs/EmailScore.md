# EmailScore


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**age_days** | **number** |  | [optional] [default to undefined]
**category** | [**EmailCategory**](EmailCategory.md) |  | [default to undefined]
**freshness** | [**Freshness**](Freshness.md) |  | [optional] [default to undefined]
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
    category,
    freshness,
    reason_codes,
    safe_to_send,
    score,
    signals,
    sub_reason,
    verified_at,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
