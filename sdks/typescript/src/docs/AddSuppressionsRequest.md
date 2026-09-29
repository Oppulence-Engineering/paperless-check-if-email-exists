# AddSuppressionsRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**emails** | **Array&lt;string&gt;** |  | [required]
**expires_at** | **string** |  | [optional]
**metadata** | **{ [key: string]: any; }** |  | [optional]
**notes** | **string** |  | [optional]
**reason** | [**SuppressionReason**](SuppressionReason.md) |  | [optional]
**reason_detail** | **string** |  | [optional]
**source** | **string** |  | [optional]
**source_ref** | **string** |  | [optional]
**source_type** | **string** |  | [optional]

## Example

```typescript
import { AddSuppressionsRequest } from '@oppulence/reacher-sdk';

const instance: AddSuppressionsRequest = {
    emails: [],
    expires_at: 'example',
    metadata: {} as any,
    notes: 'example',
    reason: {} as any,
    reason_detail: 'example',
    source: 'example',
    source_ref: 'example',
    source_type: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
