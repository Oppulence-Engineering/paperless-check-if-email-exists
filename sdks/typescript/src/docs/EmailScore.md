# EmailScore


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**age_days** | **number** |  | [optional]
**catch_all** | [**CatchAllScore**](CatchAllScore.md) |  | [optional]
**catch_all_severity** | [**CatchAllSeverity**](CatchAllSeverity.md) |  | [optional]
**category** | [**EmailCategory**](EmailCategory.md) |  | [required]
**confidence** | **number** |  | [optional]
**confidence_factors** | **Array&lt;string&gt;** |  | [optional]
**confidence_level** | [**ConfidenceLevel**](ConfidenceLevel.md) |  | [optional]
**domain_suggestion** | **string** | Suggested corrected email when a likely domain typo is detected | [optional]
**freshness** | [**Freshness**](Freshness.md) |  | [optional]
**normalized_email** | **string** | Canonical form of the email after alias/plus-address normalization | [optional]
**partial_confidence** | [**PartialConfidence**](PartialConfidence.md) |  | [optional]
**reason_codes** | [**Array&lt;ReasonCode&gt;**](ReasonCode.md) |  | [required]
**safe_to_send** | **boolean** |  | [required]
**score** | **number** |  | [required]
**signals** | [**ScoringSignals**](ScoringSignals.md) |  | [required]
**sub_reason** | [**SubReason**](SubReason.md) |  | [required]
**verified_at** | **string** |  | [optional]

## Example

```typescript
import { EmailScore } from '@oppulence/reacher-sdk';

const instance: EmailScore = {
    age_days: 0,
    catch_all: {} as any,
    catch_all_severity: {} as any,
    category: {} as any,
    confidence: 0,
    confidence_factors: [],
    confidence_level: {} as any,
    domain_suggestion: 'example',
    freshness: {} as any,
    normalized_email: 'example',
    partial_confidence: {} as any,
    reason_codes: [],
    safe_to_send: true,
    score: 0,
    signals: {} as any,
    sub_reason: {} as any,
    verified_at: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
