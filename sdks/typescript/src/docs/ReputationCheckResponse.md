# ReputationCheckResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blacklist_results** | [**Array&lt;BlacklistResult&gt;**](BlacklistResult.md) |  | [required]
**cached** | **boolean** |  | [required]
**dns_records** | [**DnsRecordResults**](DnsRecordResults.md) |  | [required]
**domain** | **string** |  | [required]
**domain_info** | [**DomainInfo**](DomainInfo.md) |  | [required]
**risk_level** | **string** |  | [required]
**score** | **number** |  | [required]

## Example

```typescript
import { ReputationCheckResponse } from '@oppulence/reacher-sdk';

const instance: ReputationCheckResponse = {
    blacklist_results: {} as any,
    cached: true,
    dns_records: {} as any,
    domain: 'example',
    domain_info: {} as any,
    risk_level: 'example',
    score: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
