# JobTaskResult


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error** | **string** |  | [optional]
**id** | **number** |  | [required]
**result** | [**CheckEmailOutput**](CheckEmailOutput.md) |  | [optional]
**retry_count** | **number** |  | [required]
**task_state** | **string** |  | [required]

## Example

```typescript
import { JobTaskResult } from '@oppulence/reacher-sdk';

const instance: JobTaskResult = {
    error: 'example',
    id: 0,
    result: {} as any,
    retry_count: 0,
    task_state: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
