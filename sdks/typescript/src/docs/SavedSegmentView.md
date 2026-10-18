# SavedSegmentView


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **string** |  | [required]
**filter** | [**SegmentFilter**](SegmentFilter.md) |  | [required]
**id** | **number** |  | [required]
**name** | **string** |  | [required]
**scope** | [**SegmentScope**](SegmentScope.md) |  | [required]
**updated_at** | **string** |  | [required]

## Example

```typescript
import { SavedSegmentView } from '@oppulence/reacher-sdk';

const instance: SavedSegmentView = {
    created_at: 'example',
    filter: {} as any,
    id: 0,
    name: 'Weekly Cleanup',
    scope: {} as any,
    updated_at: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
