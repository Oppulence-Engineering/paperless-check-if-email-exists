# OverviewSectionVecOverviewListDataInner


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **number** |  | [required]
**job_id** | **number** |  | [required]
**name** | **string** |  | [required]
**quality_grade** | **string** |  | [optional]
**ready_to_send** | **boolean** |  | [optional]
**safe_to_send_count** | **number** |  | [optional]
**status** | **string** |  | [required]
**total_processed** | **number** |  | [required]
**total_rows** | **number** |  | [required]

## Example

```typescript
import { OverviewSectionVecOverviewListDataInner } from '@oppulence/reacher-sdk';

const instance: OverviewSectionVecOverviewListDataInner = {
    id: 0,
    job_id: 0,
    name: 'Weekly Cleanup',
    quality_grade: 'example',
    ready_to_send: true,
    safe_to_send_count: 0,
    status: 'example',
    total_processed: 0,
    total_rows: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
