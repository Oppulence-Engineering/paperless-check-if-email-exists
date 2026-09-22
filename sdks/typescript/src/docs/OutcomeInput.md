# OutcomeInput


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**campaign_id** | **string** |  | [optional]
**email** | **string** |  | [required]
**event_type** | **string** |  | [required]
**metadata** | **any** |  | [optional]
**occurred_at** | **string** |  | [optional]
**provider_event_id** | **string** |  | [optional]
**provider_message_id** | **string** |  | [optional]
**source_key** | **string** |  | [optional]

## Example

```typescript
import { OutcomeInput } from '@oppulence/reacher-sdk';

const instance: OutcomeInput = {
    campaign_id: 'example',
    email: 'example',
    event_type: 'example',
    metadata: {} as any,
    occurred_at: 'example',
    provider_event_id: 'example',
    provider_message_id: 'example',
    source_key: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
