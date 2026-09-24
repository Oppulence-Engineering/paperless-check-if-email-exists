# EmailScore


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**age_days** | **number** |  | [optional]
**catch_all_severity** | **string** | Severity tier for catch-all domains. low&#x3D;free provider with no other negative signal; high&#x3D;corporate domain, or any catch-all carrying another negative signal. Only the high tier blocks safe_to_send. | [optional]
**category** | [**EmailCategory**](EmailCategory.md) |  | [required]
**domain_suggestion** | **string** | Suggested corrected email when a likely domain typo is detected | [optional]
**freshness** | [**Freshness**](Freshness.md) |  | [optional]
**normalized_email** | **string** | Canonical form of the email after alias/plus-address normalization | [optional]
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
    catch_all_severity: 'example',
    category: {} as any,
    domain_suggestion: 'example',
    freshness: {} as any,
    normalized_email: 'example',
    reason_codes: [],
    safe_to_send: true,
    score: 0,
    signals: {} as any,
    sub_reason: {} as any,
    verified_at: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
