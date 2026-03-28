# MiscDetails

Additional information about the email account.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gravatar_url** | **string** | URL to the Gravatar profile picture associated with the email, if available and requested. | [optional]
**is_b2c** | **boolean** | Is this a B2C email address? | [required]
**is_disposable** | **boolean** | Indicates if the email address is from a known disposable email provider. | [required]
**is_role_account** | **boolean** | Indicates if the email address is a role-based account. | [required]

## Example

```typescript
import { MiscDetails } from '@oppulence/reacher-sdk';

const instance: MiscDetails = {
    gravatar_url: 'example',
    is_b2c: true,
    is_disposable: true,
    is_role_account: true,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
