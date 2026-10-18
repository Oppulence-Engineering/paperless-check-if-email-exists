# PipelineRunStats


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepted_rows** | **number** |  | [optional]
**billed_emails** | **number** |  | [optional]
**changed_only_export** | **boolean** |  | [optional]
**delta_mode** | **boolean** |  | [optional]
**freshness_days** | **number** |  | [optional]
**job_id** | **number** |  | [optional]
**list_id** | **number** |  | [optional]
**selected_unique_emails** | **number** |  | [optional]
**skipped_unchanged** | **number** |  | [optional]
**source_filename** | **string** |  | [optional]
**source_name** | **string** |  | [optional]
**source_row_count** | **number** |  | [optional]
**source_unique_emails** | **number** |  | [optional]
**total_failed** | **number** |  | [optional]
**total_invalid** | **number** |  | [optional]
**total_processed** | **number** |  | [optional]
**total_risky** | **number** |  | [optional]
**total_unknown** | **number** |  | [optional]
**total_valid** | **number** |  | [optional]
**trigger_reason** | **string** |  | [optional]

## Example

```typescript
import { PipelineRunStats } from '@oppulence/reacher-sdk';

const instance: PipelineRunStats = {
    accepted_rows: 0,
    billed_emails: 0,
    changed_only_export: true,
    delta_mode: true,
    freshness_days: 30,
    job_id: 0,
    list_id: 0,
    selected_unique_emails: 0,
    skipped_unchanged: 0,
    source_filename: 'example',
    source_name: 'example',
    source_row_count: 0,
    source_unique_emails: 0,
    total_failed: 0,
    total_invalid: 0,
    total_processed: 0,
    total_risky: 0,
    total_unknown: 0,
    total_valid: 0,
    trigger_reason: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
