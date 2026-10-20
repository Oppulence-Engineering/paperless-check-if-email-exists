# DnsRecordResults


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dkim_selectors** | [**Array&lt;DnsRecordResultsDkimSelectorsInner&gt;**](DnsRecordResultsDkimSelectorsInner.md) |  | [required]
**dmarc_policy** | **string** |  | [optional]
**has_dkim** | **boolean** | True if a probed selector has usable key material; null when unknown. Does not verify a message signature or establish absence. | [required]
**has_dmarc** | **boolean** |  | [required]
**has_mx** | **boolean** |  | [required]
**has_spf** | **boolean** |  | [required]
**mx_records** | **Array&lt;string&gt;** |  | [required]
**spf_policy_explicit** | **boolean** | Heuristic: a single SPF record has a terminal all or nonempty redirect. Does not validate syntax or evaluate sender authorization. | [required]

## Example

```typescript
import { DnsRecordResults } from '@oppulence/reacher-sdk';

const instance: DnsRecordResults = {
    dkim_selectors: [],
    dmarc_policy: 'example',
    has_dkim: true,
    has_dmarc: true,
    has_mx: true,
    has_spf: true,
    mx_records: [],
    spf_policy_explicit: true,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
