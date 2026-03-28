# PipelineView


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **string** |  | [required]
**delivery** | [**PipelineDeliveryConfig**](PipelineDeliveryConfig.md) |  | [required]
**id** | **number** |  | [required]
**last_run_id** | **number** |  | [optional]
**last_scheduled_at** | **string** |  | [optional]
**name** | **string** |  | [required]
**next_run_at** | **string** |  | [optional]
**policy** | [**PipelinePolicyConfig**](PipelinePolicyConfig.md) |  | [required]
**schedule** | [**PipelineSchedule**](PipelineSchedule.md) |  | [required]
**source** | [**PipelineSource**](PipelineSource.md) |  | [required]
**status** | [**PipelineStatus**](PipelineStatus.md) |  | [required]
**tenant_id** | **string** |  | [required]
**updated_at** | **string** |  | [required]
**verification** | [**PipelineVerificationSettings**](PipelineVerificationSettings.md) |  | [required]

## Example

```typescript
import { PipelineView } from '@oppulence/reacher-sdk';

const instance: PipelineView = {
    created_at: 'example',
    delivery: {} as any,
    id: 0,
    last_run_id: 0,
    last_scheduled_at: 'example',
    name: 'Weekly Cleanup',
    next_run_at: 'example',
    policy: {} as any,
    schedule: {} as any,
    source: {} as any,
    status: {} as any,
    tenant_id: 'example',
    updated_at: 'example',
    verification: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
