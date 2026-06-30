# BulkCreateRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**input** | **Array&lt;string&gt;** |  | [required]
**source** | **string** | Alias for source_key. | [optional]
**source_key** | **string** | Optional source key used for source quality analytics. | [optional]
**webhook** | **{ [key: string]: any; }** |  | [optional]

## Example

```typescript
import { BulkCreateRequest } from '@oppulence/reacher-sdk';

const instance: BulkCreateRequest = {
    input: [],
    source: 'example',
    source_key: 'example',
    webhook: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
