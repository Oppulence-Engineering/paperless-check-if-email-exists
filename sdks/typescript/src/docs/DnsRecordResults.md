# DnsRecordResults


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dmarc_policy** | **string** |  | [optional]
**has_dkim** | **boolean** |  | [required]
**has_dmarc** | **boolean** |  | [required]
**has_mx** | **boolean** |  | [required]
**has_spf** | **boolean** |  | [required]
**mx_records** | **Array&lt;string&gt;** |  | [required]
**spf_valid** | **boolean** |  | [required]

## Example

```typescript
import { DnsRecordResults } from '@oppulence/reacher-sdk';

const instance: DnsRecordResults = {
    dmarc_policy: 'example',
    has_dkim: true,
    has_dmarc: true,
    has_mx: true,
    has_spf: true,
    mx_records: [],
    spf_valid: true,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
