# SuppressionCheckResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **string** |  | [optional]
**reason** | [**SuppressionReason**](SuppressionReason.md) |  | [optional]
**source** | **string** |  | [optional]
**suppressed** | **boolean** |  | [required]

## Example

```typescript
import { SuppressionCheckResponse } from '@oppulence/reacher-sdk';

const instance: SuppressionCheckResponse = {
    created_at: 'example',
    reason: {} as any,
    source: 'example',
    suppressed: true,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
