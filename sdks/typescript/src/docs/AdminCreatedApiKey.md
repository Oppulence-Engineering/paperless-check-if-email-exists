# AdminCreatedApiKey


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **string** |  | [optional]
**tenant_id** | **string** |  | [optional]
**key_prefix** | **string** |  | [optional]
**name** | **string** |  | [optional]
**scopes** | **Array&lt;string&gt;** |  | [optional]
**status** | **string** |  | [optional]
**last_used_at** | **string** |  | [optional]
**expires_at** | **string** |  | [optional]
**created_at** | **string** |  | [optional]
**key** | **string** | Plaintext key returned only at creation. | [required]

## Example

```typescript
import { AdminCreatedApiKey } from '@oppulence/reacher-sdk';

const instance: AdminCreatedApiKey = {
    id: 'example',
    tenant_id: 'example',
    key_prefix: 'example',
    name: 'Weekly Cleanup',
    scopes: [],
    status: 'example',
    last_used_at: 'example',
    expires_at: 'example',
    created_at: 'example',
    key: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
