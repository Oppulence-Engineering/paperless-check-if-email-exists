# ResearchDetail


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_candidates** | **number** |  | [required]
**max_verifications** | **number** |  | [required]
**rows** | [**Array&lt;ResearchRow&gt;**](ResearchRow.md) |  | [required]
**session** | [**ResearchSummary**](ResearchSummary.md) |  | [required]
**status** | **string** |  | [required]
**target_role** | **string** |  | [required]
**verification_requests** | **number** |  | [required]

## Example

```typescript
import { ResearchDetail } from '@oppulence/reacher-sdk';

const instance: ResearchDetail = {
    max_candidates: 0,
    max_verifications: 0,
    rows: [],
    session: {} as any,
    status: 'example',
    target_role: 'example',
    verification_requests: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
