# ReverificationStatusResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**batch_size** | **number** |  | [optional] [default to undefined]
**emails_requeued** | **number** |  | [optional] [default to undefined]
**enabled** | **boolean** |  | [default to undefined]
**last_job_id** | **number** |  | [optional] [default to undefined]
**last_run_at** | **string** |  | [optional] [default to undefined]
**next_run_at** | **string** |  | [optional] [default to undefined]
**staleness_days** | **number** |  | [optional] [default to undefined]

## Example

```typescript
import { ReverificationStatusResponse } from '@oppulence/reacher-sdk';

const instance: ReverificationStatusResponse = {
    batch_size,
    emails_requeued,
    enabled,
    last_job_id,
    last_run_at,
    next_run_at,
    staleness_days,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
