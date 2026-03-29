# BounceRiskAssessment


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | [**RecommendedAction**](RecommendedAction.md) |  | [required]
**category** | [**BounceRiskCategory**](BounceRiskCategory.md) |  | [required]
**confidence** | **number** |  | [required]
**model_version** | **string** |  | [required]
**risk_factors** | [**Array&lt;RiskFactor&gt;**](RiskFactor.md) |  | [required]
**score** | **number** |  | [required]
**scored_at** | **string** |  | [required]

## Example

```typescript
import { BounceRiskAssessment } from '@oppulence/reacher-sdk';

const instance: BounceRiskAssessment = {
    action: {} as any,
    category: {} as any,
    confidence: 0,
    model_version: 'example',
    risk_factors: [],
    score: 0,
    scored_at: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
