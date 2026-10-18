# SegmentFilter


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | [**SegmentFilterCategory**](SegmentFilterCategory.md) |  | [optional]
**change_type** | [**SegmentFilterCategory**](SegmentFilterCategory.md) |  | [optional]
**freshness_days_max** | **number** |  | [optional]
**is_catch_all** | **boolean** |  | [optional]
**is_disposable** | **boolean** |  | [optional]
**is_role_account** | **boolean** |  | [optional]
**policy_decision** | **string** |  | [optional]
**reason_codes_any** | **Array&lt;string&gt;** |  | [optional]
**safe_to_send** | **boolean** |  | [optional]
**score_max** | **number** |  | [optional]
**score_min** | **number** |  | [optional]

## Example

```typescript
import { SegmentFilter } from '@oppulence/reacher-sdk';

const instance: SegmentFilter = {
    category: {} as any,
    change_type: {} as any,
    freshness_days_max: 0,
    is_catch_all: true,
    is_disposable: true,
    is_role_account: true,
    policy_decision: 'example',
    reason_codes_any: [],
    safe_to_send: true,
    score_max: 0,
    score_min: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
