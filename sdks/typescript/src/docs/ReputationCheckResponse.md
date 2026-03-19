# ReputationCheckResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blacklist_results** | [**Array&lt;BlacklistResult&gt;**](BlacklistResult.md) |  | [default to undefined]
**cached** | **boolean** |  | [default to undefined]
**dns_records** | [**DnsRecordResults**](DnsRecordResults.md) |  | [default to undefined]
**domain** | **string** |  | [default to undefined]
**domain_info** | [**DomainInfo**](DomainInfo.md) |  | [default to undefined]
**risk_level** | **string** |  | [default to undefined]
**score** | **number** |  | [default to undefined]

## Example

```typescript
import { ReputationCheckResponse } from '@oppulence/reacher-sdk';

const instance: ReputationCheckResponse = {
    blacklist_results,
    cached,
    dns_records,
    domain,
    domain_info,
    risk_level,
    score,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
