# TenantApiKey


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **string** |  | [required]
**expires_at** | **string** |  | [optional]
**id** | **string** |  | [required]
**key_prefix** | **string** |  | [required]
**last_used_at** | **string** |  | [optional]
**name** | **string** |  | [required]
**scopes** | **Array&lt;string&gt;** |  | [required]
**status** | **string** |  | [required]
**tenant_id** | **string** |  | [required]

## Example

```typescript
import { TenantApiKey } from '@oppulence/reacher-sdk';

const instance: TenantApiKey = {
    created_at: 'example',
    expires_at: 'example',
    id: 'example',
    key_prefix: 'example',
    last_used_at: 'example',
    name: 'Weekly Cleanup',
    scopes: [],
    status: 'example',
    tenant_id: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
