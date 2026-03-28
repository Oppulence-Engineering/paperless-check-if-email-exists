# FindEmailStatusResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**best_match** | [**FinderBestMatch**](FinderBestMatch.md) |  | [optional]
**bulk_job_id** | **number** |  | [required]
**candidates_checked** | **number** |  | [required]
**domain_has_mx** | **boolean** |  | [required]
**domain_is_catch_all** | **boolean** |  | [required]
**job_id** | **number** |  | [required]
**results** | [**Array&lt;FinderCandidateResult&gt;**](FinderCandidateResult.md) |  | [required]
**status** | **string** |  | [required]

## Example

```typescript
import { FindEmailStatusResponse } from '@oppulence/reacher-sdk';

const instance: FindEmailStatusResponse = {
    best_match: {} as any,
    bulk_job_id: 0,
    candidates_checked: 0,
    domain_has_mx: true,
    domain_is_catch_all: true,
    job_id: 0,
    results: [],
    status: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
