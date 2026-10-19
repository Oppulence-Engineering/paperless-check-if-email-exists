# DiffRow


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**base_category** | **string** |  | [optional]
**base_row_index** | **number** |  | [optional]
**base_score** | **number** |  | [optional]
**base_task_id** | **number** |  | [optional]
**canonical_email** | **string** |  | [required]
**change_type** | **string** |  | [required]
**compare_category** | **string** |  | [optional]
**compare_row_index** | **number** |  | [optional]
**compare_score** | **number** |  | [optional]
**compare_task_id** | **number** |  | [optional]

## Example

```typescript
import { DiffRow } from '@oppulence/reacher-sdk';

const instance: DiffRow = {
    base_category: 'example',
    base_row_index: 0,
    base_score: 0,
    base_task_id: 0,
    canonical_email: 'example',
    change_type: 'example',
    compare_category: 'example',
    compare_row_index: 0,
    compare_score: 0,
    compare_task_id: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
