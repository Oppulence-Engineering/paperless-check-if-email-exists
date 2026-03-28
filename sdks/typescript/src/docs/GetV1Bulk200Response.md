# GetV1Bulk200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**job_id** | **number** |  | [required]
**created_at** | **string** | The date and time when the bulk verification job was created. | [required]
**finished_at** | **string** | If the bulk verification job is completed, the date and time when it was finished. | [optional]
**total_records** | **number** | The number of emails to verify in the bulk verification job. | [required]
**total_processed** | **number** | The number of emails that have been verified at the time of the query. | [required]
**summary** | [**GetV1Bulk200ResponseSummary**](GetV1Bulk200ResponseSummary.md) |  | [required]
**job_status** | **string** | The status of the job, either \&quot;Running\&quot; or \&quot;Completed\&quot;. | [required]

## Example

```typescript
import { GetV1Bulk200Response } from '@oppulence/reacher-sdk';

const instance: GetV1Bulk200Response = {
    job_id: 0,
    created_at: 'example',
    finished_at: 'example',
    total_records: 0,
    total_processed: 0,
    summary: {} as any,
    job_status: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
