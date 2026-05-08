# CreateScorePolicyRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_default** | **boolean** |  | [optional]
**name** | **string** |  | [required]
**rules** | **any** |  | [optional]

## Example

```typescript
import { CreateScorePolicyRequest } from '@oppulence/reacher-sdk';

const instance: CreateScorePolicyRequest = {
    is_default: true,
    name: 'Weekly Cleanup',
    rules: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
