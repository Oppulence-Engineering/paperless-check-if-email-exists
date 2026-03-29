# RiskFactor


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contribution** | **number** |  | [required]
**description** | **string** |  | [required]
**direction** | [**RiskDirection**](RiskDirection.md) |  | [required]
**signal** | **string** |  | [required]

## Example

```typescript
import { RiskFactor } from '@oppulence/reacher-sdk';

const instance: RiskFactor = {
    contribution: 0,
    description: 'example',
    direction: {} as any,
    signal: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
