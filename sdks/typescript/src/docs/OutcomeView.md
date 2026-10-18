# OutcomeView


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**campaign_id** | **string** |  | [optional]
**canonical_email** | **string** |  | [required]
**correlation_status** | **string** |  | [required]
**created_at** | **string** |  | [required]
**email** | **string** |  | [required]
**endpoint_id** | **string** |  | [optional]
**event_family** | **string** |  | [optional]
**event_type** | **string** |  | [required]
**id** | **number** |  | [required]
**metadata** | **any** |  | [required]
**occurred_at** | **string** |  | [required]
**provider** | **string** |  | [required]
**provider_event_id** | **string** |  | [optional]
**provider_message_id** | **string** |  | [optional]
**receipt_id** | **string** |  | [optional]
**source_key** | **string** |  | [optional]

## Example

```typescript
import { OutcomeView } from '@oppulence/reacher-sdk';

const instance: OutcomeView = {
    campaign_id: 'example',
    canonical_email: 'example',
    correlation_status: 'example',
    created_at: 'example',
    email: 'example',
    endpoint_id: 'example',
    event_family: 'example',
    event_type: 'example',
    id: 0,
    metadata: {} as any,
    occurred_at: 'example',
    provider: 'mailchimp',
    provider_event_id: 'example',
    provider_message_id: 'example',
    receipt_id: 'example',
    source_key: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
