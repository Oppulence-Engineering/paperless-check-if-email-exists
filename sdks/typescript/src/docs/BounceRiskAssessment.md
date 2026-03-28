# BounceRiskAssessment


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | [**RecommendedAction**](RecommendedAction.md) |  | [default to undefined]
**category** | [**BounceRiskCategory**](BounceRiskCategory.md) |  | [default to undefined]
**confidence** | **number** |  | [default to undefined]
**model_version** | **string** |  | [default to undefined]
**risk_factors** | [**Array&lt;RiskFactor&gt;**](RiskFactor.md) |  | [default to undefined]
**score** | **number** |  | [default to undefined]
**scored_at** | **string** |  | [default to undefined]

## Example

```typescript
import { BounceRiskAssessment } from '@oppulence/reacher-sdk';

const instance: BounceRiskAssessment = {
    action,
    category,
    confidence,
    model_version,
    risk_factors,
    score,
    scored_at,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
