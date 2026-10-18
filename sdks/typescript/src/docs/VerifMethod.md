# VerifMethod

The method used for email verification.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**host** | **string** | The mail server host the check connected to. Only for Smtp. | [optional]
**type** | **string** | The method used for the email verification. | [required]
**verif_method** | [**VerifMethodVerifMethod**](VerifMethodVerifMethod.md) |  | [optional]

## Example

```typescript
import { VerifMethod } from '@oppulence/reacher-sdk';

const instance: VerifMethod = {
    host: 'example',
    type: 'example',
    verif_method: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
