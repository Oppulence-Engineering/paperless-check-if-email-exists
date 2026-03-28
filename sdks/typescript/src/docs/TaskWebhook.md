# TaskWebhook

Optional webhook configuration for sending email verification results during bulk verification.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**on_each_email** | [**Webhook**](Webhook.md) |  | [optional]

## Example

```typescript
import { TaskWebhook } from '@oppulence/reacher-sdk';

const instance: TaskWebhook = {
    on_each_email: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
