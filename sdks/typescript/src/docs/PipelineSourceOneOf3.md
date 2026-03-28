# PipelineSourceOneOf3


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bucket** | **string** |  | [required]
**path_pattern** | **string** |  | [optional]
**prefix** | **string** |  | [optional]
**provider** | **string** |  | [required]
**region** | **string** |  | [optional]
**type** | **string** |  | [required]

## Example

```typescript
import { PipelineSourceOneOf3 } from '@oppulence/reacher-sdk';

const instance: PipelineSourceOneOf3 = {
    bucket: 'example-bucket',
    path_pattern: '*.csv',
    prefix: 'imports/',
    provider: 's3',
    region: 'us-east-1',
    type: 'bucket',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
