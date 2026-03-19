# EmailScore


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | [**EmailCategory**](EmailCategory.md) |  | [default to undefined]
**safe_to_send** | **boolean** |  | [default to undefined]
**score** | **number** |  | [default to undefined]
**signals** | [**ScoringSignals**](ScoringSignals.md) |  | [default to undefined]
**sub_reason** | [**SubReason**](SubReason.md) |  | [default to undefined]

## Example

```typescript
import { EmailScore } from '@oppulence/reacher-sdk';

const instance: EmailScore = {
    category,
    safe_to_send,
    score,
    signals,
    sub_reason,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
