# AdminApiKey


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

## Example

```typescript
import { AdminApiKey } from '@oppulence/reacher-sdk';

const instance: AdminApiKey = {
    id: 'example',
    tenant_id: 'example',
    key_prefix: 'example',
    name: 'Weekly Cleanup',
    scopes: [],
    status: 'example',
    last_used_at: 'example',
    expires_at: 'example',
    created_at: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
