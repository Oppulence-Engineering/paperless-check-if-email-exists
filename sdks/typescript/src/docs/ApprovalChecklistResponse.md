# ApprovalChecklistResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**categories** | [**ApprovalCategoryBreakdown**](ApprovalCategoryBreakdown.md) |  | [required]
**job_id** | **number** |  | [required]
**ready_to_send** | **boolean** |  | [required]
**recommendation** | **string** |  | [required]
**risk_flags** | [**ApprovalRiskFlags**](ApprovalRiskFlags.md) |  | [required]
**safe_to_send_count** | **number** |  | [required]
**safe_to_send_pct** | **number** |  | [required]
**total_records** | **number** |  | [required]

## Example

```typescript
import { ApprovalChecklistResponse } from '@oppulence/reacher-sdk';

const instance: ApprovalChecklistResponse = {
    categories: {} as any,
    job_id: 0,
    ready_to_send: true,
    recommendation: 'example',
    risk_flags: {} as any,
    safe_to_send_count: 0,
    safe_to_send_pct: 0,
    total_records: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
