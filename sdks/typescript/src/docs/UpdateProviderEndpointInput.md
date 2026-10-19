# UpdateProviderEndpointInput


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowed_ips** | **Array&lt;string&gt;** |  | [optional]
**label** | **string** |  | [optional]
**provider_config** | **any** |  | [optional]
**rotate_delivery_token** | **boolean** |  | [optional]
**status** | **string** |  | [optional]

## Example

```typescript
import { UpdateProviderEndpointInput } from '@oppulence/reacher-sdk';

const instance: UpdateProviderEndpointInput = {
    allowed_ips: [],
    label: 'example',
    provider_config: {} as any,
    rotate_delivery_token: true,
    status: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
