# CheckEmailOutputSmtp

Results from connecting to the mail server via SMTP.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can_connect_smtp** | **boolean** | Indicates if the mail exchanger can be contacted successfully. | [default to undefined]
**has_full_inbox** | **boolean** | Indicates if the mailbox is full. | [default to undefined]
**is_catch_all** | **boolean** | Indicates if the email address is a catch-all address. | [default to undefined]
**is_deliverable** | **boolean** | Indicates if an email sent to this address is deliverable. | [default to undefined]
**is_disabled** | **boolean** | Indicates if the email address has been disabled by the provider. | [default to undefined]
**message** | **string** | A human-readable description of the error. | [default to undefined]
**type** | **string** | The type of error. | [default to undefined]

## Example

```typescript
import { CheckEmailOutputSmtp } from '@oppulence/reacher-sdk';

const instance: CheckEmailOutputSmtp = {
    can_connect_smtp,
    has_full_inbox,
    is_catch_all,
    is_deliverable,
    is_disabled,
    message,
    type,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
