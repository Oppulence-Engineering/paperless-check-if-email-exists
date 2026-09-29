# ListDiffResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**added** | [**DiffGroup**](DiffGroup.md) |  | [required]
**base_list_id** | **number** |  | [required]
**compare_list_id** | **number** |  | [required]
**degraded** | [**DiffGroup**](DiffGroup.md) |  | [required]
**improved** | [**DiffGroup**](DiffGroup.md) |  | [required]
**newly_invalid** | [**DiffGroup**](DiffGroup.md) |  | [required]
**newly_risky** | [**DiffGroup**](DiffGroup.md) |  | [required]
**newly_safe** | [**DiffGroup**](DiffGroup.md) |  | [required]
**removed** | [**DiffGroup**](DiffGroup.md) |  | [required]
**unchanged** | [**DiffGroup**](DiffGroup.md) |  | [required]

## Example

```typescript
import { ListDiffResponse } from '@oppulence/reacher-sdk';

const instance: ListDiffResponse = {
    added: {} as any,
    base_list_id: 0,
    compare_list_id: 0,
    degraded: {} as any,
    improved: {} as any,
    newly_invalid: {} as any,
    newly_risky: {} as any,
    newly_safe: {} as any,
    removed: {} as any,
    unchanged: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
