# FindEmailRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain** | **string** |  | [default to undefined]
**first_name** | **string** |  | [default to undefined]
**last_name** | **string** |  | [default to undefined]
**strategy** | **string** | Search strategy: parallel (all at once) or waterfall (high-quality patterns first) | [optional] [default to StrategyEnum_Parallel]

## Example

```typescript
import { FindEmailRequest } from '@oppulence/reacher-sdk';

const instance: FindEmailRequest = {
    domain,
    first_name,
    last_name,
    strategy,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
