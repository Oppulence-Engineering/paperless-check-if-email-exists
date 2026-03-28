# CreatePipelineInput


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delivery** | [**PipelineDeliveryConfig**](PipelineDeliveryConfig.md) |  | [optional] [default to undefined]
**name** | **string** |  | [default to undefined]
**policy** | [**PipelinePolicyConfig**](PipelinePolicyConfig.md) |  | [optional] [default to undefined]
**schedule** | [**PipelineSchedule**](PipelineSchedule.md) |  | [default to undefined]
**source** | [**PipelineSource**](PipelineSource.md) |  | [default to undefined]
**status** | [**PipelineStatus**](PipelineStatus.md) |  | [optional] [default to undefined]
**verification** | [**PipelineVerificationSettings**](PipelineVerificationSettings.md) |  | [optional] [default to undefined]

## Example

```typescript
import { CreatePipelineInput } from '@oppulence/reacher-sdk';

const instance: CreatePipelineInput = {
    delivery,
    name,
    policy,
    schedule,
    source,
    status,
    verification,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
