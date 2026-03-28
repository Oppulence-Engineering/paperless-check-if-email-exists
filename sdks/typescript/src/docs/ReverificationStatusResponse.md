# ReverificationStatusResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**batch_size** | **number** |  | [optional]
**emails_requeued** | **number** |  | [optional]
**enabled** | **boolean** |  | [required]
**last_job_id** | **number** |  | [optional]
**last_run_at** | **string** |  | [optional]
**next_run_at** | **string** |  | [optional]
**staleness_days** | **number** |  | [optional]

## Example

```typescript
import { ReverificationStatusResponse } from '@oppulence/reacher-sdk';

const instance: ReverificationStatusResponse = {
    batch_size: 0,
    emails_requeued: 0,
    enabled: true,
    last_job_id: 0,
    last_run_at: 'example',
    next_run_at: 'example',
    staleness_days: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
