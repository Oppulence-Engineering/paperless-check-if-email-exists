# AddSuppressionsResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**added** | **number** |  | [required]
**duplicates** | **number** |  | [required]
**entry_ids** | **Array&lt;number&gt;** |  | [optional]
**rejected** | [**Array&lt;AddSuppressionsResponseRejectedInner&gt;**](AddSuppressionsResponseRejectedInner.md) |  | [optional]
**submitted** | **number** |  | [optional]
**updated** | **number** |  | [optional]

## Example

```typescript
import { AddSuppressionsResponse } from '@oppulence/reacher-sdk';

const instance: AddSuppressionsResponse = {
    added: 0,
    duplicates: 0,
    entry_ids: [],
    rejected: [],
    submitted: 0,
    updated: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
