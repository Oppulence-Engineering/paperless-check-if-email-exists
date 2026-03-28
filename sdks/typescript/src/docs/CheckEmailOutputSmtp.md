# CheckEmailOutputSmtp

Results from connecting to the mail server via SMTP.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can_connect_smtp** | **boolean** | Indicates if the mail exchanger can be contacted successfully. | [optional]
**has_full_inbox** | **boolean** | Indicates if the mailbox is full. | [optional]
**is_catch_all** | **boolean** | Indicates if the email address is a catch-all address. | [optional]
**is_deliverable** | **boolean** | Indicates if an email sent to this address is deliverable. | [optional]
**is_disabled** | **boolean** | Indicates if the email address has been disabled by the provider. | [optional]
**message** | **string** | A human-readable description of the error. | [optional]
**type** | **string** | The type of error. | [optional]

## Example

```typescript
import { CheckEmailOutputSmtp } from '@oppulence/reacher-sdk';

const instance: CheckEmailOutputSmtp = {
    can_connect_smtp: true,
    has_full_inbox: true,
    is_catch_all: true,
    is_deliverable: true,
    is_disabled: true,
    message: 'example',
    type: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
