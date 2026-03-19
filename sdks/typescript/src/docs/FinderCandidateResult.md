# FinderCandidateResult


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | [**EmailCategory**](EmailCategory.md) |  | [default to undefined]
**email** | **string** |  | [default to undefined]
**is_reachable** | [**Reachable**](Reachable.md) |  | [default to undefined]
**pattern** | **string** |  | [default to undefined]
**result** | [**CheckEmailOutput**](CheckEmailOutput.md) |  | [optional] [default to undefined]
**score** | **number** |  | [default to undefined]
**sub_reason** | [**SubReason**](SubReason.md) |  | [default to undefined]

## Example

```typescript
import { FinderCandidateResult } from '@oppulence/reacher-sdk';

const instance: FinderCandidateResult = {
    category,
    email,
    is_reachable,
    pattern,
    result,
    score,
    sub_reason,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
