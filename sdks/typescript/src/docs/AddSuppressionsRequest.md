# AddSuppressionsRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**emails** | **Array&lt;string&gt;** |  | [required]
**notes** | **string** |  | [optional]
**reason** | [**SuppressionReason**](SuppressionReason.md) |  | [optional]
**source** | **string** |  | [optional]

## Example

```typescript
import { AddSuppressionsRequest } from '@oppulence/reacher-sdk';

const instance: AddSuppressionsRequest = {
    emails: [],
    notes: 'example',
    reason: {} as any,
    source: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
