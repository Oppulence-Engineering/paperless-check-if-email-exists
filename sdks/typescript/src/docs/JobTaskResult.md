# JobTaskResult


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**canonical_task_id** | **number** |  | [optional]
**error** | **string** |  | [optional]
**id** | **number** |  | [required]
**is_duplicate** | **boolean** |  | [optional]
**original_email** | **string** |  | [optional]
**result** | [**CheckEmailOutput**](CheckEmailOutput.md) |  | [optional]
**retry_count** | **number** |  | [required]
**row_number** | **number** |  | [optional]
**task_state** | **string** |  | [required]

## Example

```typescript
import { JobTaskResult } from '@oppulence/reacher-sdk';

const instance: JobTaskResult = {
    canonical_task_id: 0,
    error: 'example',
    id: 0,
    is_duplicate: true,
    original_email: 'example',
    result: {} as any,
    retry_count: 0,
    row_number: 0,
    task_state: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
