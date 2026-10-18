# QueryResultRow


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | **string** |  | [optional]
**completed_at** | **string** |  | [optional]
**created_at** | **string** |  | [required]
**email** | **string** |  | [optional]
**id** | **number** |  | [required]
**job_id** | **number** |  | [optional]
**policy_decision** | **string** |  | [optional]
**policy_evaluation** | **{ [key: string]: any; }** |  | [optional]
**policy_mode** | **string** |  | [optional]
**reason_codes** | **Array&lt;string&gt;** |  | [optional]
**recommendation** | **{ [key: string]: any; }** |  | [optional]
**recommendation_action** | **string** |  | [optional]
**safe_to_send** | **boolean** |  | [optional]
**score** | **number** |  | [optional]
**sub_reason** | **string** |  | [optional]
**task_state** | **string** |  | [required]

## Example

```typescript
import { QueryResultRow } from '@oppulence/reacher-sdk';

const instance: QueryResultRow = {
    category: 'example',
    completed_at: 'example',
    created_at: 'example',
    email: 'example',
    id: 0,
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
    task_state: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
