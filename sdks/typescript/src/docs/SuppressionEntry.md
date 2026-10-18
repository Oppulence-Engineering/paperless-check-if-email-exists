# SuppressionEntry


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**canonical_email** | **string** |  | [required]
**created_at** | **string** |  | [required]
**created_by** | **string** |  | [optional]
**email** | **string** |  | [required]
**expires_at** | **string** |  | [optional]
**id** | **number** |  | [required]
**last_seen_at** | **string** |  | [optional]
**metadata** | **{ [key: string]: any; }** |  | [required]
**notes** | **string** |  | [optional]
**reason** | [**SuppressionReason**](SuppressionReason.md) |  | [required]
**reason_code** | **string** |  | [optional]
**reason_detail** | **string** |  | [optional]
**source** | **string** |  | [optional]
**source_ref** | **string** |  | [optional]
**source_type** | **string** |  | [optional]
**status** | **string** |  | [required]
**updated_at** | **string** |  | [required]

## Example

```typescript
import { SuppressionEntry } from '@oppulence/reacher-sdk';

const instance: SuppressionEntry = {
    canonical_email: 'example',
    created_at: 'example',
    created_by: 'example',
    email: 'example',
    expires_at: 'example',
    id: 0,
    last_seen_at: 'example',
    metadata: {} as any,
    notes: 'example',
    reason: {} as any,
    reason_code: 'example',
    reason_detail: 'example',
    source: 'example',
    source_ref: 'example',
    source_type: 'example',
    status: 'example',
    updated_at: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
