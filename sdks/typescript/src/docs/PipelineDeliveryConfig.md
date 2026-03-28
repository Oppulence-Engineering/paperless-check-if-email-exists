# PipelineDeliveryConfig


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dashboard** | **boolean** |  | [optional]
**max_attempts** | **number** |  | [optional]
**retry_backoff_seconds** | **number** |  | [optional]
**webhook** | [**PipelineDeliveryWebhook**](PipelineDeliveryWebhook.md) |  | [optional]

## Example

```typescript
import { PipelineDeliveryConfig } from '@oppulence/reacher-sdk';

const instance: PipelineDeliveryConfig = {
    dashboard: true,
    max_attempts: 5,
    retry_backoff_seconds: 300,
    webhook: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
