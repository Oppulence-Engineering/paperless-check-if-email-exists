# PartialConfidence


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**classification** | [**SmtpUncertaintyClass**](SmtpUncertaintyClass.md) |  | [required]
**confidence** | **number** |  | [required]
**factors** | **Array&lt;string&gt;** |  | [required]

## Example

```typescript
import { PartialConfidence } from '@oppulence/reacher-sdk';

const instance: PartialConfidence = {
    classification: {} as any,
    confidence: 0,
    factors: [],
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
