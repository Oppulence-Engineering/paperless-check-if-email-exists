# SuppressionEvent


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actor_id** | **string** |  | [optional]
**actor_type** | **string** |  | [required]
**canonical_email** | **string** |  | [required]
**created_at** | **string** |  | [required]
**entry_id** | **number** |  | [optional]
**event_type** | **string** |  | [required]
**from_status** | **string** |  | [optional]
**id** | **number** |  | [required]
**metadata** | **{ [key: string]: any; }** |  | [required]
**reason_code** | **string** |  | [optional]
**reason_detail** | **string** |  | [optional]
**source_ref** | **string** |  | [optional]
**source_type** | **string** |  | [optional]
**to_status** | **string** |  | [optional]

## Example

```typescript
import { SuppressionEvent } from '@oppulence/reacher-sdk';

const instance: SuppressionEvent = {
    actor_id: 'example',
    actor_type: 'example',
    canonical_email: 'example',
    created_at: 'example',
    entry_id: 0,
    event_type: 'example',
    from_status: 'example',
    id: 0,
    metadata: {} as any,
    reason_code: 'example',
    reason_detail: 'example',
    source_ref: 'example',
    source_type: 'example',
    to_status: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
