# PipelineRunView


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**billed_emails** | **number** |  | [required]
**completed_at** | **string** |  | [optional]
**created_at** | **string** |  | [required]
**delivery_attempts** | **number** |  | [required]
**delivery_error** | **string** |  | [optional]
**delivery_status** | [**PipelineDeliveryStatus**](PipelineDeliveryStatus.md) |  | [required]
**error_code** | **string** |  | [optional]
**error_message** | **string** |  | [optional]
**id** | **number** |  | [required]
**job_id** | **number** |  | [optional]
**last_delivery_attempt_at** | **string** |  | [optional]
**list_id** | **number** |  | [optional]
**next_delivery_attempt_at** | **string** |  | [optional]
**pipeline_id** | **number** |  | [required]
**result_location** | **any** |  | [optional]
**scheduled_for** | **string** |  | [optional]
**source_snapshot** | **any** |  | [required]
**started_at** | **string** |  | [optional]
**stats** | **any** |  | [required]
**status** | [**PipelineRunStatus**](PipelineRunStatus.md) |  | [required]
**tenant_id** | **string** |  | [required]
**trigger_type** | **string** |  | [required]
**updated_at** | **string** |  | [required]

## Example

```typescript
import { PipelineRunView } from '@oppulence/reacher-sdk';

const instance: PipelineRunView = {
    billed_emails: 0,
    completed_at: 'example',
    created_at: 'example',
    delivery_attempts: 0,
    delivery_error: 'example',
    delivery_status: {} as any,
    error_code: 'example',
    error_message: 'example',
    id: 0,
    job_id: 0,
    last_delivery_attempt_at: 'example',
    list_id: 0,
    next_delivery_attempt_at: 'example',
    pipeline_id: 0,
    result_location: {} as any,
    scheduled_for: 'example',
    source_snapshot: {} as any,
    started_at: 'example',
    stats: {} as any,
    status: {} as any,
    tenant_id: 'example',
    trigger_type: 'example',
    updated_at: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
