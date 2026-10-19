# AuditEvent


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actor** | **string** |  | [optional]
**created_at** | **string** |  | [required]
**event_data** | **{ [key: string]: any; }** |  | [optional]
**event_type** | **string** |  | [required]
**id** | **number** |  | [required]
**job_id** | **number** |  | [required]
**task_id** | **number** |  | [optional]

## Example

```typescript
import { AuditEvent } from '@oppulence/reacher-sdk';

const instance: AuditEvent = {
    actor: 'example',
    created_at: 'example',
    event_data: {} as any,
    event_type: 'example',
    id: 0,
    job_id: 0,
    task_id: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
