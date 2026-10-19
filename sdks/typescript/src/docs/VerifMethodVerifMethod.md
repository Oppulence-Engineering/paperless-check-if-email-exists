# VerifMethodVerifMethod

The SMTP settings the check used. Only for Smtp.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_email** | **string** |  | [optional]
**hello_name** | **string** |  | [optional]
**proxy** | **string** |  | [optional]
**retries** | **number** |  | [optional]
**smtp_port** | **number** |  | [optional]
**smtp_timeout** | [**Duration**](Duration.md) |  | [optional]

## Example

```typescript
import { VerifMethodVerifMethod } from '@oppulence/reacher-sdk';

const instance: VerifMethodVerifMethod = {
    from_email: 'example',
    hello_name: 'example',
    proxy: 'example',
    retries: 0,
    smtp_port: 0,
    smtp_timeout: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
