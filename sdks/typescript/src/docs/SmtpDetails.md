# SmtpDetails

Results from SMTP connection attempts to the mail server.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can_connect_smtp** | **boolean** | Indicates if the mail exchanger can be contacted successfully. | [required]
**has_full_inbox** | **boolean** | Indicates if the mailbox is full. | [required]
**is_catch_all** | **boolean** | Indicates if the email address is a catch-all address. | [required]
**is_deliverable** | **boolean** | Indicates if an email sent to this address is deliverable. | [required]
**is_disabled** | **boolean** | Indicates if the email address has been disabled by the provider. | [required]

## Example

```typescript
import { SmtpDetails } from '@oppulence/reacher-sdk';

const instance: SmtpDetails = {
    can_connect_smtp: true,
    has_full_inbox: true,
    is_catch_all: true,
    is_deliverable: true,
    is_disabled: true,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
