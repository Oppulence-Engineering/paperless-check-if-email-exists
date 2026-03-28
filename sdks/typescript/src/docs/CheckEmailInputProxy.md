# CheckEmailInputProxy

Proxy configuration for email verification.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**host** | **string** | The proxy host address. | [required]
**password** | **string** | Password for proxy authentication. | [optional]
**port** | **number** | The proxy port number. | [required]
**username** | **string** | Username for proxy authentication. | [optional]

## Example

```typescript
import { CheckEmailInputProxy } from '@oppulence/reacher-sdk';

const instance: CheckEmailInputProxy = {
    host: 'example',
    password: 'example',
    port: 0,
    username: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
