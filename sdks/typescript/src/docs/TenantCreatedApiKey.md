# TenantCreatedApiKey


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **string** |  | [required]
**expires_at** | **string** |  | [optional]
**id** | **string** |  | [required]
**key** | **string** | The full key. The API shows it only once. | [required]
**key_prefix** | **string** |  | [required]
**name** | **string** |  | [required]
**scopes** | **Array&lt;string&gt;** |  | [required]
**status** | **string** |  | [required]
**tenant_id** | **string** |  | [required]

## Example

```typescript
import { TenantCreatedApiKey } from '@oppulence/reacher-sdk';

const instance: TenantCreatedApiKey = {
    created_at: 'example',
    expires_at: 'example',
    id: 'example',
    key: 'example',
    key_prefix: 'example',
    name: 'Weekly Cleanup',
    scopes: [],
    status: 'example',
    tenant_id: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
