# GetV1Bulk200ResponseSummary

A summary of the processed emails.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_safe** | **number** | The number of emails where &#x60;is_reachable&#x60; is \&quot;safe\&quot;. | [required]
**total_invalid** | **number** | The number of emails where &#x60;is_reachable&#x60; is \&quot;invalid\&quot;. | [required]
**total_risky** | **number** | The number of emails where &#x60;is_reachable&#x60; is \&quot;risky\&quot;. | [required]
**total_unknown** | **number** | The number of emails where &#x60;is_reachable&#x60; is \&quot;unknown\&quot;. | [required]

## Example

```typescript
import { GetV1Bulk200ResponseSummary } from '@oppulence/reacher-sdk';

const instance: GetV1Bulk200ResponseSummary = {
    total_safe: 0,
    total_invalid: 0,
    total_risky: 0,
    total_unknown: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
