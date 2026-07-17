# ProviderEndpointView


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowed_ips** | **Array&lt;string&gt;** |  | [required]
**created_at** | **string** |  | [required]
**delivery_token** | **string** |  | [optional]
**endpoint_id** | **string** |  | [required]
**label** | **string** |  | [required]
**provider** | **string** |  | [required]
**provider_configured** | **boolean** |  | [required]
**status** | **string** |  | [required]
**updated_at** | **string** |  | [required]
**webhook_path** | **string** |  | [required]

## Example

```typescript
import { ProviderEndpointView } from '@oppulence/reacher-sdk';

const instance: ProviderEndpointView = {
    allowed_ips: [],
    created_at: 'example',
    delivery_token: 'example',
    endpoint_id: 'example',
    label: 'example',
    provider: 'mailchimp',
    provider_configured: true,
    status: 'example',
    updated_at: 'example',
    webhook_path: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
