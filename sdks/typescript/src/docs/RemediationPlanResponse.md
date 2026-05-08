# RemediationPlanResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completed_at** | **string** |  | [optional]
**created_at** | **string** |  | [required]
**effective_job_id** | **number** |  | [optional]
**list_id** | **number** |  | [required]
**_options** | [**RemediationOptions**](RemediationOptions.md) |  | [optional]
**plan_id** | **number** |  | [required]
**result_state_digest** | **string** |  | [required]
**rule_version** | **string** |  | [required]
**status** | **string** |  | [required]
**summary_counts** | [**RemediationSummaryCounts**](RemediationSummaryCounts.md) |  | [required]

## Example

```typescript
import { RemediationPlanResponse } from '@oppulence/reacher-sdk';

const instance: RemediationPlanResponse = {
    completed_at: 'example',
    created_at: 'example',
    effective_job_id: 0,
    list_id: 0,
    _options: {} as any,
    plan_id: 0,
    result_state_digest: 'example',
    rule_version: 'example',
    status: 'example',
    summary_counts: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
