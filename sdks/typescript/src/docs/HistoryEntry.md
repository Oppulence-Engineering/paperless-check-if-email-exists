# HistoryEntry


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | **string** |  | [optional]
**change_type** | **string** |  | [optional]
**completed_at** | **string** |  | [optional]
**is_reachable** | **string** |  | [optional]
**job_id** | **number** |  | [optional]
**list_id** | **number** |  | [optional]
**pipeline_run_id** | **number** |  | [optional]
**policy_decision** | **string** |  | [optional]
**policy_evaluation** | **any** |  | [optional]
**policy_mode** | **string** |  | [optional]
**previous_category** | **string** |  | [optional]
**previous_list_id** | **number** |  | [optional]
**previous_pipeline_run_id** | **number** |  | [optional]
**previous_reason_codes** | **Array&lt;string&gt;** |  | [optional]
**previous_safe_to_send** | **boolean** |  | [optional]
**previous_score** | **number** |  | [optional]
**previous_task_id** | **number** |  | [optional]
**reason_codes** | **Array&lt;string&gt;** |  | [optional]
**recommendation** | **any** |  | [optional]
**recommendation_action** | **string** |  | [optional]
**safe_to_send** | **boolean** |  | [optional]
**score** | **number** |  | [optional]
**sub_reason** | **string** |  | [optional]
**task_id** | **number** |  | [required]

## Example

```typescript
import { HistoryEntry } from '@oppulence/reacher-sdk';

const instance: HistoryEntry = {
    category: 'example',
    change_type: 'example',
    completed_at: 'example',
    is_reachable: 'example',
    job_id: 0,
    list_id: 0,
    pipeline_run_id: 0,
    policy_decision: 'example',
    policy_evaluation: {} as any,
    policy_mode: 'example',
    previous_category: 'example',
    previous_list_id: 0,
    previous_pipeline_run_id: 0,
    previous_reason_codes: [],
    previous_safe_to_send: true,
    previous_score: 0,
    previous_task_id: 0,
    reason_codes: [],
    recommendation: {} as any,
    recommendation_action: 'example',
    safe_to_send: true,
    score: 0,
    sub_reason: 'example',
    task_id: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
