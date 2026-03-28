# PipelineSourceOneOf1


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audience_id** | **string** |  | [required]
**connection_id** | **string** |  | [required]
**field_mapping** | **any** |  | [optional]
**provider** | **string** |  | [required]
**type** | **string** |  | [required]

## Example

```typescript
import { PipelineSourceOneOf1 } from '@oppulence/reacher-sdk';

const instance: PipelineSourceOneOf1 = {
    audience_id: 'aud_123',
    connection_id: 'conn_123',
    field_mapping: {} as any,
    provider: 'mailchimp',
    type: 'integration',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
