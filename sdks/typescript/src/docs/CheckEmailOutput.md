# CheckEmailOutput

The result of the email verification process.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**debug** | [**DebugDetails**](DebugDetails.md) |  | [optional]
**input** | **string** | The email address that was verified. | [required]
**is_reachable** | [**Reachable**](Reachable.md) |  | [required]
**misc** | [**CheckEmailOutputMisc**](CheckEmailOutputMisc.md) |  | [required]
**mx** | [**CheckEmailOutputMx**](CheckEmailOutputMx.md) |  | [required]
**score** | [**EmailScore**](EmailScore.md) |  | [required]
**smtp** | [**CheckEmailOutputSmtp**](CheckEmailOutputSmtp.md) |  | [required]
**syntax** | [**SyntaxDetails**](SyntaxDetails.md) |  | [required]

## Example

```typescript
import { CheckEmailOutput } from '@oppulence/reacher-sdk';

const instance: CheckEmailOutput = {
    debug: {} as any,
    input: 'example',
    is_reachable: {} as any,
    misc: {} as any,
    mx: {} as any,
    score: {} as any,
    smtp: {} as any,
    syntax: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
