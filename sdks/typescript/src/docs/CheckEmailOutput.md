# CheckEmailOutput

The result of the email verification process.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**debug** | [**DebugDetails**](DebugDetails.md) |  | [optional] [default to undefined]
**input** | **string** | The email address that was verified. | [default to undefined]
**is_reachable** | [**Reachable**](Reachable.md) |  | [default to undefined]
**misc** | [**CheckEmailOutputMisc**](CheckEmailOutputMisc.md) |  | [default to undefined]
**mx** | [**CheckEmailOutputMx**](CheckEmailOutputMx.md) |  | [default to undefined]
**score** | [**EmailScore**](EmailScore.md) |  | [default to undefined]
**smtp** | [**CheckEmailOutputSmtp**](CheckEmailOutputSmtp.md) |  | [default to undefined]
**syntax** | [**SyntaxDetails**](SyntaxDetails.md) |  | [default to undefined]

## Example

```typescript
import { CheckEmailOutput } from '@oppulence/reacher-sdk';

const instance: CheckEmailOutput = {
    debug,
    input,
    is_reachable,
    misc,
    mx,
    score,
    smtp,
    syntax,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
