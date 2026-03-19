# Webhook

Configuration for a webhook to receive email verification results. The method will be POST, and the body will contain the email verification response.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**extra** | **object** |  | [optional] [default to undefined]
**url** | **string** | The URL to send the email verification results to. | [default to undefined]

## Example

```typescript
import { Webhook } from '@oppulence/reacher-sdk';

const instance: Webhook = {
    extra,
    url,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
