# BlacklistResult


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**answers** | **Array&lt;string&gt;** |  | [required]
**error** | **string** |  | [required]
**listed** | **boolean** | Null for failed or skipped queries; false only for a completed negative lookup. | [required]
**lookup_time_ms** | **number** |  | [required]
**provider** | **string** |  | [required]
**status** | **string** |  | [required]

## Example

```typescript
import { BlacklistResult } from '@oppulence/reacher-sdk';

const instance: BlacklistResult = {
    answers: [],
    error: 'example',
    listed: true,
    lookup_time_ms: 0,
    provider: 'mailchimp',
    status: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
