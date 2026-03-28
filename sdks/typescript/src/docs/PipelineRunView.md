# PipelineRunView


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**billed_emails** | **number** |  | [default to undefined]
**completed_at** | **string** |  | [optional] [default to undefined]
**created_at** | **string** |  | [default to undefined]
**delivery_attempts** | **number** |  | [default to undefined]
**delivery_error** | **string** |  | [optional] [default to undefined]
**delivery_status** | [**PipelineDeliveryStatus**](PipelineDeliveryStatus.md) |  | [default to undefined]
**error_code** | **string** |  | [optional] [default to undefined]
**error_message** | **string** |  | [optional] [default to undefined]
**id** | **number** |  | [default to undefined]
**job_id** | **number** |  | [optional] [default to undefined]
**last_delivery_attempt_at** | **string** |  | [optional] [default to undefined]
**list_id** | **number** |  | [optional] [default to undefined]
**next_delivery_attempt_at** | **string** |  | [optional] [default to undefined]
**pipeline_id** | **number** |  | [default to undefined]
**result_location** | **any** |  | [optional] [default to undefined]
**scheduled_for** | **string** |  | [optional] [default to undefined]
**source_snapshot** | **any** |  | [default to undefined]
**started_at** | **string** |  | [optional] [default to undefined]
**stats** | **any** |  | [default to undefined]
**status** | [**PipelineRunStatus**](PipelineRunStatus.md) |  | [default to undefined]
**tenant_id** | **string** |  | [default to undefined]
**trigger_type** | **string** |  | [default to undefined]
**updated_at** | **string** |  | [default to undefined]

## Example

```typescript
import { PipelineRunView } from '@oppulence/reacher-sdk';

const instance: PipelineRunView = {
    billed_emails,
    completed_at,
    created_at,
    delivery_attempts,
    delivery_error,
    delivery_status,
    error_code,
    error_message,
    id,
    job_id,
    last_delivery_attempt_at,
    list_id,
    next_delivery_attempt_at,
    pipeline_id,
    result_location,
    scheduled_for,
    source_snapshot,
    started_at,
    stats,
    status,
    tenant_id,
    trigger_type,
    updated_at,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
