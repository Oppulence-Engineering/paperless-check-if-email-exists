# V1JobApprovalChecklist200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**categories** | **object** |  | [optional]
**job_id** | **number** |  | [optional]
**ready_to_send** | **boolean** |  | [optional]
**recommendation** | **string** |  | [optional]
**risk_flags** | **object** |  | [optional]
**safe_to_send_count** | **number** |  | [optional]
**safe_to_send_pct** | **number** |  | [optional]
**total_records** | **number** |  | [optional]

## Example

```typescript
import { V1JobApprovalChecklist200Response } from '@oppulence/reacher-sdk';

const instance: V1JobApprovalChecklist200Response = {
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
