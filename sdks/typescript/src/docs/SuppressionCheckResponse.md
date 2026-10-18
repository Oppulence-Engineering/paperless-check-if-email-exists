# SuppressionCheckResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**canonical_email** | **string** |  | [optional]
**created_at** | **string** |  | [optional]
**created_by** | **string** |  | [optional]
**expires_at** | **string** |  | [optional]
**id** | **number** |  | [optional]
**metadata** | **{ [key: string]: any; }** |  | [optional]
**reason** | [**SuppressionReason**](SuppressionReason.md) |  | [optional]
**reason_code** | **string** |  | [optional]
**reason_detail** | **string** |  | [optional]
**source** | **string** |  | [optional]
**source_ref** | **string** |  | [optional]
**source_type** | **string** |  | [optional]
**status** | **string** |  | [optional]
**suppressed** | **boolean** |  | [required]

## Example

```typescript
import { SuppressionCheckResponse } from '@oppulence/reacher-sdk';

const instance: SuppressionCheckResponse = {
    canonical_email: 'example',
    created_at: 'example',
    created_by: 'example',
    expires_at: 'example',
    id: 0,
    metadata: {} as any,
    reason: {} as any,
    reason_code: 'example',
    reason_detail: 'example',
    source: 'example',
    source_ref: 'example',
    source_type: 'example',
    status: 'example',
    suppressed: true,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
