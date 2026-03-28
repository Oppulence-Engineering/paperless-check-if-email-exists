# Webhook

Configuration for a webhook to receive email verification results. The method will be POST, and the body will contain the email verification response.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**extra** | **object** |  | [optional]
**url** | **string** | The URL to send the email verification results to. | [required]

## Example

```typescript
import { Webhook } from '@oppulence/reacher-sdk';

const instance: Webhook = {
    extra: {} as any,
    url: 'https://example.com/webhook',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
