# CheckEmailOutput

The result of the email verification process.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bounce_risk** | [**BounceRiskAssessment**](BounceRiskAssessment.md) | Additive bounce-risk assessment. May be null when enrichment is disabled or unavailable. | [optional]
**debug** | [**DebugDetails**](DebugDetails.md) |  | [optional]
**input** | **string** | The email address that was verified. | [required]
**is_reachable** | [**Reachable**](Reachable.md) |  | [required]
**misc** | [**CheckEmailOutputMisc**](CheckEmailOutputMisc.md) |  | [required]
**mx** | [**CheckEmailOutputMx**](CheckEmailOutputMx.md) |  | [required]
**provider** | [**Provider**](Provider.md) |  | [optional]
**provider_confidence** | [**ProviderConfidence**](ProviderConfidence.md) |  | [optional]
**provider_rejection_reason** | [**ProviderRejectionReason**](ProviderRejectionReason.md) |  | [optional]
**provider_rules_applied** | **boolean** |  | [required]
**score** | [**EmailScore**](EmailScore.md) |  | [required]
**smtp** | [**CheckEmailOutputSmtp**](CheckEmailOutputSmtp.md) |  | [required]
**syntax** | [**SyntaxDetails**](SyntaxDetails.md) |  | [required]

## Example

```typescript
import { CheckEmailOutput } from '@oppulence/reacher-sdk';

const instance: CheckEmailOutput = {
    bounce_risk: {} as any,
    debug: {} as any,
    input: 'example',
    is_reachable: {} as any,
    misc: {} as any,
    mx: {} as any,
    provider: {} as any,
    provider_confidence: {} as any,
    provider_rejection_reason: {} as any,
    provider_rules_applied: true,
    score: {} as any,
    smtp: {} as any,
    syntax: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
