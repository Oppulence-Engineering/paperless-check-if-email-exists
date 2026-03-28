# JobResultPageResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**has_more** | **boolean** |  | [required]
**next_cursor** | **number** |  | [optional]
**results** | [**Array&lt;JobTaskResult&gt;**](JobTaskResult.md) |  | [required]

## Example

```typescript
import { JobResultPageResponse } from '@oppulence/reacher-sdk';

const instance: JobResultPageResponse = {
    has_more: true,
    next_cursor: 0,
    results: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
