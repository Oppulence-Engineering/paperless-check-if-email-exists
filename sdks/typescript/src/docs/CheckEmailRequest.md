# CheckEmailRequest

A request object to perform an email verification. The `to_email` field is required, all other fields are optional.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**check_gravatar** | **boolean** | Whether to check if a Gravatar image exists for the given email. | [optional]
**from_email** | **string** | In the SMTP connection, the FROM email address. | [optional]
**gmail_verif_method** | [**GmailVerifMethod**](GmailVerifMethod.md) |  | [optional]
**hello_name** | **string** | In the SMTP connection, the EHLO hostname. | [optional]
**hotmailb2b_verif_method** | [**HotmailB2BVerifMethod**](HotmailB2BVerifMethod.md) |  | [optional]
**hotmailb2c_verif_method** | [**HotmailB2CVerifMethod**](HotmailB2CVerifMethod.md) |  | [optional]
**proxy** | [**CheckEmailInputProxy**](CheckEmailInputProxy.md) |  | [optional]
**sandbox** | **boolean** | When true, returns deterministic mock results without consuming credits or making real SMTP connections. | [optional]
**smtp_port** | **number** | SMTP port to use for email validation. Defaults to 25, but 465, 587, and 2525 are sometimes also used. | [optional]
**strict_provider_rules** | **boolean** | When false, skips provider-specific syntax validation even if the provider is recognized. | [optional]
**to_email** | **string** | The email address to check. | [required]
**yahoo_verif_method** | [**YahooVerifMethod**](YahooVerifMethod.md) |  | [optional]

## Example

```typescript
import { CheckEmailRequest } from '@oppulence/reacher-sdk';

const instance: CheckEmailRequest = {
    check_gravatar: true,
    from_email: 'example',
    gmail_verif_method: {} as any,
    hello_name: 'example',
    hotmailb2b_verif_method: {} as any,
    hotmailb2c_verif_method: {} as any,
    proxy: {} as any,
    sandbox: true,
    smtp_port: 0,
    strict_provider_rules: true,
    to_email: 'example',
    yahoo_verif_method: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
