# AdminCreateTenantRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contact_email** | **string** |  | [required]
**default_webhook_url** | **string** |  | [optional]
**max_requests_per_day** | **number** |  | [optional]
**max_requests_per_hour** | **number** |  | [optional]
**max_requests_per_minute** | **number** |  | [optional]
**max_requests_per_second** | **number** |  | [optional]
**monthly_email_limit** | **number** |  | [optional]
**name** | **string** |  | [required]
**plan_tier** | **string** |  | [optional]
**result_retention_days** | **number** |  | [optional]
**slug** | **string** |  | [required]
**webhook_signing_secret** | **string** |  | [optional]

## Example

```typescript
import { AdminCreateTenantRequest } from '@oppulence/reacher-sdk';

const instance: AdminCreateTenantRequest = {
    contact_email: 'example',
    default_webhook_url: 'example',
    max_requests_per_day: 0,
    max_requests_per_hour: 0,
    max_requests_per_minute: 0,
    max_requests_per_second: 0,
    monthly_email_limit: 0,
    name: 'Weekly Cleanup',
    plan_tier: 'example',
    result_retention_days: 0,
    slug: 'example',
    webhook_signing_secret: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
