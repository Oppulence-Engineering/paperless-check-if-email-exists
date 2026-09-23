# OnboardRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contact_email** | **string** | Contact email for the tenant account (billing, alerts). | [required]
**email_to_verify** | **string** | Email address to verify. | [required]
**plan_tier** | **string** | Optional plan tier (defaults to \&quot;free\&quot;). | [optional]
**slug** | **string** | URL-safe slug (auto-generated from tenant_name if omitted). | [optional]
**tenant_name** | **string** | Display name for the new tenant. | [required]

## Example

```typescript
import { OnboardRequest } from '@oppulence/reacher-sdk';

const instance: OnboardRequest = {
    contact_email: 'example',
    email_to_verify: 'example',
    plan_tier: 'example',
    slug: 'example',
    tenant_name: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
