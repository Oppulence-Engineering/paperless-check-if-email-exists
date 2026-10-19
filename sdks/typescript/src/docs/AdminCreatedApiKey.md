# AdminCreatedApiKey


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **string** |  | [optional]
**expires_at** | **string** |  | [optional]
**id** | **string** |  | [optional]
**key_prefix** | **string** |  | [optional]
**last_used_at** | **string** |  | [optional]
**name** | **string** |  | [optional]
**scopes** | **Array&lt;string&gt;** |  | [optional]
**status** | **string** |  | [optional]
**tenant_id** | **string** |  | [optional]
**key** | **string** | Plaintext key returned only at creation. | [required]

## Example

```typescript
import { AdminCreatedApiKey } from '@oppulence/reacher-sdk';

const instance: AdminCreatedApiKey = {
    created_at: 'example',
    expires_at: 'example',
    id: 'example',
    key_prefix: 'example',
    last_used_at: 'example',
    name: 'Weekly Cleanup',
    scopes: [],
    status: 'example',
    tenant_id: 'example',
    key: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
