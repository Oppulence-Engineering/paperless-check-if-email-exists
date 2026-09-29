# IngestOutcomesResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepted** | **number** |  | [required]
**errors** | [**Array&lt;IngestRowError&gt;**](IngestRowError.md) |  | [required]
**policy_id** | **number** |  | [required]
**rejected** | **number** |  | [required]
**suppressed** | **number** |  | [required]

## Example

```typescript
import { IngestOutcomesResponse } from '@oppulence/reacher-sdk';

const instance: IngestOutcomesResponse = {
    accepted: 0,
    errors: [],
    policy_id: 0,
    rejected: 0,
    suppressed: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
