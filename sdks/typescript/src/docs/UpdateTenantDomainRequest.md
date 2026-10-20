# UpdateTenantDomainRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain** | **string** |  | [optional]
**is_active** | **boolean** |  | [optional]
**is_verified** | **boolean** | Only read to refuse it: verification is not the tenant\&#39;s to declare. | [optional]
**notes** | **string** |  | [optional]

## Example

```typescript
import { UpdateTenantDomainRequest } from '@oppulence/reacher-sdk';

const instance: UpdateTenantDomainRequest = {
    domain: 'example',
    is_active: true,
    is_verified: true,
    notes: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
