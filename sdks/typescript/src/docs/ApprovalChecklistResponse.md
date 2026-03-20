# ApprovalChecklistResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**categories** | [**ApprovalCategoryBreakdown**](ApprovalCategoryBreakdown.md) |  | [default to undefined]
**job_id** | **number** |  | [default to undefined]
**ready_to_send** | **boolean** |  | [default to undefined]
**recommendation** | **string** |  | [default to undefined]
**risk_flags** | [**ApprovalRiskFlags**](ApprovalRiskFlags.md) |  | [default to undefined]
**safe_to_send_count** | **number** |  | [default to undefined]
**safe_to_send_pct** | **number** |  | [default to undefined]
**total_records** | **number** |  | [default to undefined]

## Example

```typescript
import { ApprovalChecklistResponse } from '@oppulence/reacher-sdk';

const instance: ApprovalChecklistResponse = {
    categories,
    job_id,
    ready_to_send,
    recommendation,
    risk_flags,
    safe_to_send_count,
    safe_to_send_pct,
    total_records,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
