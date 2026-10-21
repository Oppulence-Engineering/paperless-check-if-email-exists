# ListUploadResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**duplicates** | **number** |  | [optional]
**email_column** | **string** |  | [required]
**job_id** | **number** |  | [required]
**list_id** | **number** |  | [required]
**rejected** | [**Array&lt;RejectedRow&gt;**](RejectedRow.md) |  | [optional]
**source_key** | **string** |  | [optional]
**total_rows** | **number** |  | [required]
**unique_emails** | **number** |  | [optional]

## Example

```typescript
import { ListUploadResponse } from '@oppulence/reacher-sdk';

const instance: ListUploadResponse = {
    duplicates: 0,
    email_column: 'example',
    job_id: 0,
    list_id: 0,
    rejected: [],
    source_key: 'example',
    total_rows: 0,
    unique_emails: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
