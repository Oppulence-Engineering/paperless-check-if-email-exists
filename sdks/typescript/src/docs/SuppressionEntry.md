# SuppressionEntry


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **string** |  | [required]
**email** | **string** |  | [required]
**id** | **number** |  | [required]
**notes** | **string** |  | [optional]
**reason** | [**SuppressionReason**](SuppressionReason.md) |  | [required]
**source** | **string** |  | [optional]

## Example

```typescript
import { SuppressionEntry } from '@oppulence/reacher-sdk';

const instance: SuppressionEntry = {
    created_at: 'example',
    email: 'example',
    id: 0,
    notes: 'example',
    reason: {} as any,
    source: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
