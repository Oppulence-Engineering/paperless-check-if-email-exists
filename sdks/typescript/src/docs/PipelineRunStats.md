# PipelineRunStats


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**billed_emails** | **number** |  | [optional]
**changed_only_export** | **boolean** |  | [optional]
**completed_tasks** | **number** |  | [optional]
**delta_mode** | **boolean** |  | [optional]
**freshness_days** | **number** |  | [optional]
**invalid** | **number** |  | [optional]
**published_tasks** | **number** |  | [optional]
**queued_emails** | **number** |  | [optional]
**risky** | **number** |  | [optional]
**selected_unique_emails** | **number** |  | [optional]
**skipped_unchanged** | **number** |  | [optional]
**source_filename** | **string** |  | [optional]
**source_name** | **string** |  | [optional]
**source_row_count** | **number** |  | [optional]
**source_unique_emails** | **number** |  | [optional]
**trigger_reason** | **string** |  | [optional]
**unknown** | **number** |  | [optional]
**valid** | **number** |  | [optional]

## Example

```typescript
import { PipelineRunStats } from '@oppulence/reacher-sdk';

const instance: PipelineRunStats = {
    billed_emails: 0,
    changed_only_export: true,
    completed_tasks: 0,
    delta_mode: true,
    freshness_days: 30,
    invalid: 0,
    published_tasks: 0,
    queued_emails: 0,
    risky: 0,
    selected_unique_emails: 0,
    skipped_unchanged: 0,
    source_filename: 'example',
    source_name: 'example',
    source_row_count: 0,
    source_unique_emails: 0,
    trigger_reason: 'example',
    unknown: 0,
    valid: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
