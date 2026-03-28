# FinderCandidateResult


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | [**EmailCategory**](EmailCategory.md) |  | [required]
**confidence** | [**ConfidenceExplanation**](ConfidenceExplanation.md) |  | [optional]
**email** | **string** |  | [required]
**is_reachable** | [**Reachable**](Reachable.md) |  | [required]
**pattern** | **string** |  | [required]
**result** | [**CheckEmailOutput**](CheckEmailOutput.md) |  | [optional]
**score** | **number** |  | [required]
**sub_reason** | [**SubReason**](SubReason.md) |  | [required]

## Example

```typescript
import { FinderCandidateResult } from '@oppulence/reacher-sdk';

const instance: FinderCandidateResult = {
    category: {} as any,
    confidence: {} as any,
    email: 'example',
    is_reachable: {} as any,
    pattern: 'example',
    result: {} as any,
    score: 0,
    sub_reason: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
