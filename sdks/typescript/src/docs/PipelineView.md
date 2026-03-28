# PipelineView


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **string** |  | [default to undefined]
**delivery** | [**PipelineDeliveryConfig**](PipelineDeliveryConfig.md) |  | [default to undefined]
**id** | **number** |  | [default to undefined]
**last_run_id** | **number** |  | [optional] [default to undefined]
**last_scheduled_at** | **string** |  | [optional] [default to undefined]
**name** | **string** |  | [default to undefined]
**next_run_at** | **string** |  | [optional] [default to undefined]
**policy** | [**PipelinePolicyConfig**](PipelinePolicyConfig.md) |  | [default to undefined]
**schedule** | [**PipelineSchedule**](PipelineSchedule.md) |  | [default to undefined]
**source** | [**PipelineSource**](PipelineSource.md) |  | [default to undefined]
**status** | [**PipelineStatus**](PipelineStatus.md) |  | [default to undefined]
**tenant_id** | **string** |  | [default to undefined]
**updated_at** | **string** |  | [default to undefined]
**verification** | [**PipelineVerificationSettings**](PipelineVerificationSettings.md) |  | [default to undefined]

## Example

```typescript
import { PipelineView } from '@oppulence/reacher-sdk';

const instance: PipelineView = {
    created_at,
    delivery,
    id,
    last_run_id,
    last_scheduled_at,
    name,
    next_run_at,
    policy,
    schedule,
    source,
    status,
    tenant_id,
    updated_at,
    verification,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
