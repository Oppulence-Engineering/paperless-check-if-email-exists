# AdminApiKeyWriteRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **string** |  | [optional]
**scopes** | **Array&lt;string&gt;** |  | [optional]
**expires_at** | **string** |  | [optional]

## Example

```typescript
import { AdminApiKeyWriteRequest } from '@oppulence/reacher-sdk';

const instance: AdminApiKeyWriteRequest = {
    name: 'Weekly Cleanup',
    scopes: [],
    expires_at: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
