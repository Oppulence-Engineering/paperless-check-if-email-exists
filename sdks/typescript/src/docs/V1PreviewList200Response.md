# V1PreviewList200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**duplicates** | **number** |  | [required]
**email_column** | **string** |  | [required]
**estimated_credits** | **number** |  | [required]
**rejected** | [**Array&lt;RejectedRow&gt;**](RejectedRow.md) |  | [required]
**total_rows** | **number** |  | [required]
**unique_emails** | **number** |  | [required]

## Example

```typescript
import { V1PreviewList200Response } from '@oppulence/reacher-sdk';

const instance: V1PreviewList200Response = {
    duplicates: 0,
    email_column: 'example',
    estimated_credits: 0,
    rejected: [],
    total_rows: 0,
    unique_emails: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
