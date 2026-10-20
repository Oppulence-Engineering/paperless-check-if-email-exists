# VerificationResponse

SDK result for either a completed verification (HTTP 200) or its pending operation (HTTP 202).

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bounce_risk** | [**BounceRiskAssessment**](BounceRiskAssessment.md) | Additive bounce-risk assessment. May be null when enrichment is disabled or unavailable. | [optional]
**debug** | [**DebugDetails**](DebugDetails.md) |  | [optional]
**input** | **string** | The email address that was verified. | [optional]
**is_reachable** | [**Reachable**](Reachable.md) |  | [optional]
**misc** | [**CheckEmailOutputMisc**](CheckEmailOutputMisc.md) |  | [optional]
**mx** | [**CheckEmailOutputMx**](CheckEmailOutputMx.md) |  | [optional]
**policy_evaluation** | **any** |  | [optional]
**provider** | [**Provider**](Provider.md) |  | [optional]
**provider_confidence** | [**ProviderConfidence**](ProviderConfidence.md) |  | [optional]
**provider_rejection_reason** | [**ProviderRejectionReason**](ProviderRejectionReason.md) |  | [optional]
**provider_rules_applied** | **boolean** |  | [optional]
**recommendation** | **any** |  | [optional]
**score** | [**EmailScore**](EmailScore.md) |  | [optional]
**smtp** | [**CheckEmailOutputSmtp**](CheckEmailOutputSmtp.md) |  | [optional]
**syntax** | [**SyntaxDetails**](SyntaxDetails.md) |  | [optional]
**operation_id** | **string** |  | [optional]
**status** | **string** |  | [optional]
**status_url** | **string** |  | [optional]

## Example

```typescript
import { VerificationResponse } from '@oppulence/reacher-sdk';

const instance: VerificationResponse = {
    bounce_risk: {} as any,
    debug: {} as any,
    input: 'example',
    is_reachable: {} as any,
    misc: {} as any,
    mx: {} as any,
    policy_evaluation: {} as any,
    provider: {} as any,
    provider_confidence: {} as any,
    provider_rejection_reason: {} as any,
    provider_rules_applied: true,
    recommendation: {} as any,
    score: {} as any,
    smtp: {} as any,
    syntax: {} as any,
    operation_id: 'example',
    status: 'example',
    status_url: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
