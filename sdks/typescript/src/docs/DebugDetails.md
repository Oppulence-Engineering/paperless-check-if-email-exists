# DebugDetails


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**duration** | [**Duration**](Duration.md) |  | [default to undefined]
**end_time** | **string** | The timestamp when the email verification ended. | [default to undefined]
**server_name** | **string** | The name of the server that performed the verification. | [default to undefined]
**smtp** | [**DebugDetailsSmtp**](DebugDetailsSmtp.md) |  | [default to undefined]
**start_time** | **string** | The timestamp when the email verification started. | [default to undefined]

## Example

```typescript
import { DebugDetails } from '@oppulence/reacher-sdk';

const instance: DebugDetails = {
    duration,
    end_time,
    server_name,
    smtp,
    start_time,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
