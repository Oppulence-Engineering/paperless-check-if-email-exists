# CreatePipelineInput


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delivery** | [**PipelineDeliveryConfig**](PipelineDeliveryConfig.md) |  | [optional]
**name** | **string** |  | [required]
**policy** | [**PipelinePolicyConfig**](PipelinePolicyConfig.md) |  | [optional]
**schedule** | [**PipelineSchedule**](PipelineSchedule.md) |  | [required]
**source** | [**PipelineSource**](PipelineSource.md) |  | [required]
**status** | [**PipelineStatus**](PipelineStatus.md) |  | [optional]
**verification** | [**PipelineVerificationSettings**](PipelineVerificationSettings.md) |  | [optional]

## Example

```typescript
import { CreatePipelineInput } from '@oppulence/reacher-sdk';

const instance: CreatePipelineInput = {
    name: 'Weekly Cleanup',
    schedule: { cron: '0 9 * * 1', timezone: 'UTC' },
    source: { type: 'list_snapshot', list_id: 123 },
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
