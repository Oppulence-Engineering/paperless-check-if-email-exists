# DebugDetails


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backend_name** | **string** | The name of the backend that performed the verification. | [required]
**duration** | [**Duration**](Duration.md) |  | [required]
**end_time** | **string** | The timestamp when the email verification ended. | [required]
**smtp** | [**DebugDetailsSmtp**](DebugDetailsSmtp.md) |  | [required]
**start_time** | **string** | The timestamp when the email verification started. | [required]

## Example

```typescript
import { DebugDetails } from '@oppulence/reacher-sdk';

const instance: DebugDetails = {
    backend_name: 'example',
    duration: {} as any,
    end_time: 'example',
    smtp: {} as any,
    start_time: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
