# OutcomeIngestRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**outcomes** | [**Array&lt;OutcomeInput&gt;**](OutcomeInput.md) |  | [required]
**provider** | **string** |  | [required]
**source_key** | **string** |  | [optional]

## Example

```typescript
import { OutcomeIngestRequest } from '@oppulence/reacher-sdk';

const instance: OutcomeIngestRequest = {
    outcomes: [],
    provider: 'mailchimp',
    source_key: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
