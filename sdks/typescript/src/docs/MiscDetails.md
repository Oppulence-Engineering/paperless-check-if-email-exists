# MiscDetails

Additional information about the email account.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gravatar_url** | **string** | URL to the Gravatar profile picture associated with the email, if available and requested. | [required]
**haveibeenpwned** | **boolean** |  | [required]
**is_b2c** | **boolean** | Is this a B2C email address? | [required]
**is_disposable** | **boolean** | Indicates if the email address is from a known disposable email provider. | [required]
**is_role_account** | **boolean** | Indicates if the email address is a role-based account. | [required]
**is_spam_trap_domain** | **boolean** |  | [required]

## Example

```typescript
import { MiscDetails } from '@oppulence/reacher-sdk';

const instance: MiscDetails = {
    gravatar_url: 'example',
    haveibeenpwned: true,
    is_b2c: true,
    is_disposable: true,
    is_role_account: true,
    is_spam_trap_domain: true,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
