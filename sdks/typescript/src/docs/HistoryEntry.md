# HistoryEntry


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | **string** |  | [optional]
**completed_at** | **string** |  | [optional]
**is_reachable** | **string** |  | [optional]
**job_id** | **number** |  | [optional]
**policy_decision** | **string** |  | [optional]
**policy_evaluation** | **any** |  | [optional]
**policy_mode** | **string** |  | [optional]
**reason_codes** | **Array&lt;string&gt;** |  | [optional]
**recommendation** | **any** |  | [optional]
**recommendation_action** | **string** |  | [optional]
**safe_to_send** | **boolean** |  | [optional]
**score** | **number** |  | [optional]
**sub_reason** | **string** |  | [optional]

## Example

```typescript
import { HistoryEntry } from '@oppulence/reacher-sdk';

const instance: HistoryEntry = {
    category: 'example',
    completed_at: 'example',
    is_reachable: 'example',
    job_id: 0,
    policy_decision: 'example',
    policy_evaluation: {} as any,
    policy_mode: 'example',
    reason_codes: [],
    recommendation: {} as any,
    recommendation_action: 'example',
    safe_to_send: true,
    score: 0,
    sub_reason: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
