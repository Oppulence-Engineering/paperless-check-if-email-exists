# FindEmailStatusResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**best_match** | [**FinderBestMatch**](FinderBestMatch.md) |  | [optional] [default to undefined]
**bulk_job_id** | **number** |  | [default to undefined]
**candidates_checked** | **number** |  | [default to undefined]
**domain_has_mx** | **boolean** |  | [default to undefined]
**domain_is_catch_all** | **boolean** |  | [default to undefined]
**job_id** | **number** |  | [default to undefined]
**results** | [**Array&lt;FinderCandidateResult&gt;**](FinderCandidateResult.md) |  | [default to undefined]
**status** | **string** |  | [default to undefined]

## Example

```typescript
import { FindEmailStatusResponse } from '@oppulence/reacher-sdk';

const instance: FindEmailStatusResponse = {
    best_match,
    bulk_job_id,
    candidates_checked,
    domain_has_mx,
    domain_is_catch_all,
    job_id,
    results,
    status,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
