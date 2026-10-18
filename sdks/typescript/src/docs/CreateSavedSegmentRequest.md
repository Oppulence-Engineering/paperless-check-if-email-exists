# CreateSavedSegmentRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filter** | [**SegmentFilter**](SegmentFilter.md) |  | [optional]
**name** | **string** |  | [required]
**scope** | [**SegmentScope**](SegmentScope.md) |  | [optional]

## Example

```typescript
import { CreateSavedSegmentRequest } from '@oppulence/reacher-sdk';

const instance: CreateSavedSegmentRequest = {
    filter: {} as any,
    name: 'Weekly Cleanup',
    scope: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
