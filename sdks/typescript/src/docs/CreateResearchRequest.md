# CreateResearchRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idempotency_key** | **string** |  | [required]
**import_content** | **string** |  | [required]
**import_format** | [**ResearchImportFormat**](ResearchImportFormat.md) |  | [required]
**max_candidates** | **number** |  | [required]
**max_verifications** | **number** |  | [required]
**name** | **string** |  | [required]
**target_role** | **string** |  | [required]

## Example

```typescript
import { CreateResearchRequest } from '@oppulence/reacher-sdk';

const instance: CreateResearchRequest = {
    idempotency_key: 'example',
    import_content: 'example',
    import_format: {} as any,
    max_candidates: 0,
    max_verifications: 0,
    name: 'Weekly Cleanup',
    target_role: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
