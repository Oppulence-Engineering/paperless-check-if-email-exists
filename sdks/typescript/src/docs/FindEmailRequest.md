# FindEmailRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**domain** | **string** |  | [required]
**first_name** | **string** |  | [required]
**last_name** | **string** |  | [required]
**strategy** | **string** | Search strategy: parallel (all at once) or waterfall (high-quality patterns first) | [optional]

## Example

```typescript
import { FindEmailRequest } from '@oppulence/reacher-sdk';

const instance: FindEmailRequest = {
    domain: 'example',
    first_name: 'example',
    last_name: 'example',
    strategy: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
