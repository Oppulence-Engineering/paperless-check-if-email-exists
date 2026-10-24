# OverviewResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alerts** | [**OverviewSectionAlertListResponse**](OverviewSectionAlertListResponse.md) |  | [required]
**domains** | [**OverviewSectionOverviewDomains**](OverviewSectionOverviewDomains.md) |  | [required]
**lists** | [**OverviewSectionVecOverviewList**](OverviewSectionVecOverviewList.md) |  | [required]
**outcomes** | [**OverviewSectionOverviewOutcomes**](OverviewSectionOverviewOutcomes.md) |  | [required]
**pipelines** | [**OverviewSectionVecOverviewPipeline**](OverviewSectionVecOverviewPipeline.md) |  | [required]
**reverification** | [**OverviewSectionOverviewReverification**](OverviewSectionOverviewReverification.md) |  | [required]
**suppressions** | [**OverviewSectionI64**](OverviewSectionI64.md) |  | [required]
**usage** | [**TenantUsageResponse**](TenantUsageResponse.md) |  | [required]

## Example

```typescript
import { OverviewResponse } from '@oppulence/reacher-sdk';

const instance: OverviewResponse = {
    alerts: {} as any,
    domains: {} as any,
    lists: {} as any,
    outcomes: {} as any,
    pipelines: {} as any,
    reverification: {} as any,
    suppressions: {} as any,
    usage: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
