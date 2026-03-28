# PipelineSchedule


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cron** | **string** |  | [required]
**timezone** | **string** |  | [required]

## Example

```typescript
import { PipelineSchedule } from '@oppulence/reacher-sdk';

const instance: PipelineSchedule = {
    cron: '0 9 * * 1',
    timezone: 'UTC',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
