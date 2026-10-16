# UpdateReverificationRequest

Turns scheduled re-checks on or off for the workspace and sets how old a result may get before it is checked again.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**batch_size** | **number** | Addresses re-checked in each run (1 to 1,000). | [optional]
**enabled** | **boolean** |  | [required]
**staleness_days** | **number** | Re-check results older than this many days (1 to 365). | [required]

## Example

```typescript
import { UpdateReverificationRequest } from '@oppulence/reacher-sdk';

const instance: UpdateReverificationRequest = {
    batch_size: 0,
    enabled: true,
    staleness_days: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
