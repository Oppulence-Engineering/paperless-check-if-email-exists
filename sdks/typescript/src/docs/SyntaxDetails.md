# SyntaxDetails

Validation of the email address syntax.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain** | **string** | The domain part of the email address. | [required]
**is_valid_syntax** | **boolean** | Indicates if the email address syntax is valid. | [required]
**username** | **string** | The username part of the email address. | [required]

## Example

```typescript
import { SyntaxDetails } from '@oppulence/reacher-sdk';

const instance: SyntaxDetails = {
    domain: 'example',
    is_valid_syntax: true,
    username: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
