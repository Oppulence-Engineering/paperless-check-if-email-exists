# CoreError

The error that one part of the check returned instead of its details.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **string** | The category of an SMTP error, when it is known. | [optional]
**error** | [**CoreErrorError**](CoreErrorError.md) |  | [required]

## Example

```typescript
import { CoreError } from '@oppulence/reacher-sdk';

const instance: CoreError = {
    description: 'example',
    error: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
