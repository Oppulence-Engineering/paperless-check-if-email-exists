# PushPipelineResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepted_rows** | **number** |  | [required]
**batch_id** | **number** |  | [required]
**replayed** | **boolean** |  | [required]
**run_id** | **number** |  | [required]
**status** | [**PipelineRunStatus**](PipelineRunStatus.md) |  | [required]

## Example

```typescript
import { PushPipelineResponse } from '@oppulence/reacher-sdk';

const instance: PushPipelineResponse = {
    accepted_rows: 0,
    batch_id: 0,
    replayed: true,
    run_id: 0,
    status: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
