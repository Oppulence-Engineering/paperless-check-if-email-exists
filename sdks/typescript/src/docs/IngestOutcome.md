# IngestOutcome

One outcome event ingested via API or CSV.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**campaign_id** | **string** |  | [optional]
**email** | **string** |  | [required]
**metadata** | **any** |  | [optional]
**occurred_at** | **string** |  | [required]
**source** | **string** |  | [optional]
**type** | [**OutcomeType**](OutcomeType.md) |  | [required]

## Example

```typescript
import { IngestOutcome } from '@oppulence/reacher-sdk';

const instance: IngestOutcome = {
    campaign_id: 'example',
    email: 'example',
    metadata: {} as any,
    occurred_at: 'example',
    source: 'example',
    type: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
