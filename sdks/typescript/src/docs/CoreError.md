# CoreError

Details of an error encountered during the verification process.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | **string** | A human-readable description of the error. | [required]
**type** | **string** | The type of error. | [required]

## Example

```typescript
import { CoreError } from '@oppulence/reacher-sdk';

const instance: CoreError = {
    message: 'example',
    type: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
