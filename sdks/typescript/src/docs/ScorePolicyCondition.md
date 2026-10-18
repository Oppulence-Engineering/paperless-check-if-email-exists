# ScorePolicyCondition


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | **Array&lt;string&gt;** |  | [optional]
**reason_codes_any** | **Array&lt;string&gt;** |  | [optional]
**safe_to_send** | **boolean** |  | [optional]
**score_max** | **number** |  | [optional]
**score_min** | **number** |  | [optional]

## Example

```typescript
import { ScorePolicyCondition } from '@oppulence/reacher-sdk';

const instance: ScorePolicyCondition = {
    category: [],
    reason_codes_any: [],
    safe_to_send: true,
    score_max: 0,
    score_min: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
