# CreateProviderEndpointInput


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowed_ips** | **Array&lt;string&gt;** |  | [optional]
**label** | **string** |  | [required]
**provider** | **string** |  | [required]
**provider_config** | **any** |  | [optional]
**status** | **string** |  | [optional]

## Example

```typescript
import { CreateProviderEndpointInput } from '@oppulence/reacher-sdk';

const instance: CreateProviderEndpointInput = {
    allowed_ips: [],
    label: 'example',
    provider: 'mailchimp',
    provider_config: {} as any,
    status: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
