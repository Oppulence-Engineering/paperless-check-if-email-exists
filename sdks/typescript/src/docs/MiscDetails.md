# MiscDetails

Additional information about the email account.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gravatar_url** | **string** | URL to the Gravatar profile picture associated with the email, if available and requested. | [optional] [default to undefined]
**is_b2c** | **boolean** | Is this a B2C email address? | [default to undefined]
**is_disposable** | **boolean** | Indicates if the email address is from a known disposable email provider. | [default to undefined]
**is_role_account** | **boolean** | Indicates if the email address is a role-based account. | [default to undefined]

## Example

```typescript
import { MiscDetails } from '@oppulence/reacher-sdk';

const instance: MiscDetails = {
    gravatar_url,
    is_b2c,
    is_disposable,
    is_role_account,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
