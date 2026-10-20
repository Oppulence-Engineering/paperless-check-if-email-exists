# VerificationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BounceRisk** | Pointer to [**NullableBounceRiskAssessment**](BounceRiskAssessment.md) | Additive bounce-risk assessment. May be null when enrichment is disabled or unavailable. | [optional]
**Debug** | Pointer to [**DebugDetails**](DebugDetails.md) |  | [optional]
**ExternalVerification** | Pointer to [**CheckEmailOutputExternalVerification**](CheckEmailOutputExternalVerification.md) |  | [optional]
**Input** | **string** | The email address that was verified. | [optional]
**IsReachable** | [**Reachable**](Reachable.md) |  | [optional]
**Misc** | [**CheckEmailOutputMisc**](CheckEmailOutputMisc.md) |  | [optional]
**Mx** | [**CheckEmailOutputMx**](CheckEmailOutputMx.md) |  | [optional]
**PolicyEvaluation** | Pointer to **interface{}** |  | [optional]
**Provider** | Pointer to [**Provider**](Provider.md) |  | [optional]
**ProviderConfidence** | Pointer to [**ProviderConfidence**](ProviderConfidence.md) |  | [optional]
**ProviderRejectionReason** | Pointer to [**ProviderRejectionReason**](ProviderRejectionReason.md) |  | [optional]
**ProviderRulesApplied** | **bool** |  | [optional]
**Recommendation** | Pointer to **interface{}** |  | [optional]
**Score** | [**EmailScore**](EmailScore.md) |  | [optional]
**Smtp** | [**CheckEmailOutputSmtp**](CheckEmailOutputSmtp.md) |  | [optional]
**Syntax** | [**SyntaxDetails**](SyntaxDetails.md) |  | [optional]
**OperationId** | **string** |  | [optional]
**Status** | **string** |  | [optional]
**StatusUrl** | **string** |  | [optional]

## Methods

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
