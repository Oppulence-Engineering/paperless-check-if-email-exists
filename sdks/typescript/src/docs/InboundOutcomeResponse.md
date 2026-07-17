# InboundOutcomeResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepted** | **number** |  | [required]
**duplicates** | **number** |  | [required]
**provider** | **string** |  | [required]
**receipt_id** | **string** |  | [required]
**rejected** | **number** |  | [required]
**unmatched** | **number** |  | [required]

## Example

```typescript
import { InboundOutcomeResponse } from '@oppulence/reacher-sdk';

const instance: InboundOutcomeResponse = {
    accepted: 0,
    duplicates: 0,
    provider: 'mailchimp',
    receipt_id: 'example',
    rejected: 0,
    unmatched: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
