# PipelineDeliveryConfig


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dashboard** | **boolean** |  | [optional] [default to undefined]
**max_attempts** | **number** |  | [optional] [default to undefined]
**retry_backoff_seconds** | **number** |  | [optional] [default to undefined]
**webhook** | [**PipelineDeliveryWebhook**](PipelineDeliveryWebhook.md) |  | [optional] [default to undefined]

## Example

```typescript
import { PipelineDeliveryConfig } from '@oppulence/reacher-sdk';

const instance: PipelineDeliveryConfig = {
    dashboard,
    max_attempts,
    retry_backoff_seconds,
    webhook,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
