# ListDetailResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deduplicated_count** | **number** |  | [optional]
**email_column** | **string** |  | [required]
**id** | **number** |  | [required]
**job_id** | **number** |  | [required]
**name** | **string** |  | [required]
**status** | **string** |  | [required]
**summary** | [**ListSummary**](ListSummary.md) |  | [required]
**total_rows** | **number** |  | [required]
**unique_emails** | **number** |  | [optional]

## Example

```typescript
import { ListDetailResponse } from '@oppulence/reacher-sdk';

const instance: ListDetailResponse = {
    deduplicated_count: 0,
    email_column: 'example',
    id: 0,
    job_id: 0,
    name: 'Weekly Cleanup',
    status: 'example',
    summary: {} as any,
    total_rows: 0,
    unique_emails: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
