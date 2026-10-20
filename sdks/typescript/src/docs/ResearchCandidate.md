# ResearchCandidate


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **string** |  | [required]
**email_source** | **string** |  | [required]
**employment_status** | **string** |  | [required]
**evidence_url** | **string** |  | [optional]
**mailbox** | [**ResearchMailbox**](ResearchMailbox.md) |  | [optional]
**name** | **string** |  | [required]
**observed_at** | **string** |  | [optional]
**person_status** | **string** |  | [required]
**role_status** | **string** |  | [required]
**title** | **string** |  | [required]
**verification_error** | **string** |  | [optional]

## Example

```typescript
import { ResearchCandidate } from '@oppulence/reacher-sdk';

const instance: ResearchCandidate = {
    email: 'example',
    email_source: 'example',
    employment_status: 'example',
    evidence_url: 'example',
    mailbox: {} as any,
    name: 'Weekly Cleanup',
    observed_at: 'example',
    person_status: 'example',
    role_status: 'example',
    title: 'example',
    verification_error: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
