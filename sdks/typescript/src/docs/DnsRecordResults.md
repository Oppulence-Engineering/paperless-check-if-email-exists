# DnsRecordResults


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dmarc_policy** | **string** |  | [optional] [default to undefined]
**has_dkim** | **boolean** |  | [default to undefined]
**has_dmarc** | **boolean** |  | [default to undefined]
**has_mx** | **boolean** |  | [default to undefined]
**has_spf** | **boolean** |  | [default to undefined]
**mx_records** | **Array&lt;string&gt;** |  | [default to undefined]
**spf_valid** | **boolean** |  | [default to undefined]

## Example

```typescript
import { DnsRecordResults } from '@oppulence/reacher-sdk';

const instance: DnsRecordResults = {
    dmarc_policy,
    has_dkim,
    has_dmarc,
    has_mx,
    has_spf,
    mx_records,
    spf_valid,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
