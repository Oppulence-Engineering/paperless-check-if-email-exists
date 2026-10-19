# ListItem


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completed_at** | **string** |  | [required]
**created_at** | **string** |  | [required]
**email_column** | **string** |  | [required]
**id** | **number** |  | [required]
**name** | **string** |  | [required]
**original_filename** | **string** |  | [required]
**source_key** | **string** |  | [optional]
**status** | **string** |  | [required]
**total_rows** | **number** |  | [required]

## Example

```typescript
import { ListItem } from '@oppulence/reacher-sdk';

const instance: ListItem = {
    completed_at: 'example',
    created_at: 'example',
    email_column: 'example',
    id: 0,
    name: 'Weekly Cleanup',
    original_filename: 'example',
    source_key: 'example',
    status: 'example',
    total_rows: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
