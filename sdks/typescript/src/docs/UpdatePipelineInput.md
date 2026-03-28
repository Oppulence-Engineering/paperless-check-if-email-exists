# UpdatePipelineInput


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delivery** | [**PipelineDeliveryConfig**](PipelineDeliveryConfig.md) |  | [optional]
**name** | **string** |  | [optional]
**policy** | [**PipelinePolicyConfig**](PipelinePolicyConfig.md) |  | [optional]
**schedule** | [**PipelineSchedule**](PipelineSchedule.md) |  | [optional]
**source** | [**PipelineSource**](PipelineSource.md) |  | [optional]
**status** | [**PipelineStatus**](PipelineStatus.md) |  | [optional]
**verification** | [**PipelineVerificationSettings**](PipelineVerificationSettings.md) |  | [optional]

## Example

```typescript
import { UpdatePipelineInput } from '@oppulence/reacher-sdk';

const instance: UpdatePipelineInput = {
    delivery: {} as any,
    name: 'Weekly Cleanup',
    policy: {} as any,
    schedule: {} as any,
    source: {} as any,
    status: {} as any,
    verification: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
