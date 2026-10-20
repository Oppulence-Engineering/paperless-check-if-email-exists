# CheckEmailOutputExternalVerification

External verification evidence. When present, SMTP was not performed locally.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**result** | **string** |  | [required]
**result_code** | **number** |  | [required]
**service** | **string** |  | [required]
**subresult** | **string** |  | [required]

## Example

```typescript
import { CheckEmailOutputExternalVerification } from '@oppulence/reacher-sdk';

const instance: CheckEmailOutputExternalVerification = {
    result: 'example',
    result_code: 0,
    service: 'example',
    subresult: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
