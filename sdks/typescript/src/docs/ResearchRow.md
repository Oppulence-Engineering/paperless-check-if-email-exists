# ResearchRow


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**candidates** | [**Array&lt;ResearchCandidate&gt;**](ResearchCandidate.md) |  | [required]
**checked_pages** | **Array&lt;string&gt;** |  | [required]
**company_domain** | **string** |  | [required]
**company_name** | **string** |  | [required]
**reason** | **string** |  | [required]
**row_key** | **string** |  | [required]
**status** | **string** |  | [required]

## Example

```typescript
import { ResearchRow } from '@oppulence/reacher-sdk';

const instance: ResearchRow = {
    candidates: [],
    checked_pages: [],
    company_domain: 'example',
    company_name: 'example',
    reason: 'example',
    row_key: 'example',
    status: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
