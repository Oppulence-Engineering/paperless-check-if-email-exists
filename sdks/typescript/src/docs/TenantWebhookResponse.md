# TenantWebhookResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_webhook_url** | **string** |  | [optional]
**tenant_id** | **string** |  | [required]
**tenant_name** | **string** |  | [required]
**webhook_signing_secret_configured** | **boolean** |  | [required]

## Example

```typescript
import { TenantWebhookResponse } from '@oppulence/reacher-sdk';

const instance: TenantWebhookResponse = {
    default_webhook_url: 'example',
    tenant_id: 'example',
    tenant_name: 'example',
    webhook_signing_secret_configured: true,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
