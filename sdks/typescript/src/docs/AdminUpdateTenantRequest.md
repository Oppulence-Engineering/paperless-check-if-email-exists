# AdminUpdateTenantRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **string** |  | [optional]
**contact_email** | **string** |  | [optional]
**plan_tier** | **string** |  | [optional]
**status** | **string** |  | [optional]
**monthly_email_limit** | **number** |  | [optional]
**max_requests_per_second** | **number** |  | [optional]
**max_requests_per_minute** | **number** |  | [optional]
**max_requests_per_hour** | **number** |  | [optional]
**max_requests_per_day** | **number** |  | [optional]
**default_webhook_url** | **string** |  | [optional]
**webhook_signing_secret** | **string** |  | [optional]
**result_retention_days** | **number** |  | [optional]

## Example

```typescript
import { AdminUpdateTenantRequest } from '@oppulence/reacher-sdk';

const instance: AdminUpdateTenantRequest = {
    name: 'Weekly Cleanup',
    contact_email: 'example',
    plan_tier: 'example',
    status: 'example',
    monthly_email_limit: 0,
    max_requests_per_second: 0,
    max_requests_per_minute: 0,
    max_requests_per_hour: 0,
    max_requests_per_day: 0,
    default_webhook_url: 'example',
    webhook_signing_secret: 'example',
    result_retention_days: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
