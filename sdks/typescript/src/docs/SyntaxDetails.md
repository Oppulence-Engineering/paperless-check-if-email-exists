# SyntaxDetails

Validation of the email address syntax.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **string** |  | [required]
**domain** | **string** | The domain part of the email address. | [required]
**is_valid_syntax** | **boolean** | Indicates if the email address syntax is valid. | [required]
**normalized_email** | **string** |  | [required]
**suggestion** | **string** |  | [required]
**username** | **string** | The username part of the email address. | [required]

## Example

```typescript
import { SyntaxDetails } from '@oppulence/reacher-sdk';

const instance: SyntaxDetails = {
    address: 'example',
    domain: 'example',
    is_valid_syntax: true,
    normalized_email: 'example',
    suggestion: 'example',
    username: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
