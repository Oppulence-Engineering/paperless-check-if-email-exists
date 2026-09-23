# AdminTenant


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **string** |  | [optional]
**name** | **string** |  | [optional]
**slug** | **string** |  | [optional]
**contact_email** | **string** |  | [optional]
**plan_tier** | **string** |  | [optional]
**status** | **string** |  | [optional]
**monthly_email_limit** | **number** |  | [optional]
**max_requests_per_second** | **number** |  | [optional]
**max_requests_per_minute** | **number** |  | [optional]
**max_requests_per_hour** | **number** |  | [optional]
**max_requests_per_day** | **number** |  | [optional]
**used_this_period** | **number** |  | [optional]
**default_webhook_url** | **string** |  | [optional]
**result_retention_days** | **number** |  | [optional]
**created_at** | **string** |  | [optional]
**updated_at** | **string** |  | [optional]

## Example

```typescript
import { AdminTenant } from '@oppulence/reacher-sdk';

const instance: AdminTenant = {
    id: 'example',
    name: 'Weekly Cleanup',
    slug: 'example',
    contact_email: 'example',
    plan_tier: 'example',
    status: 'example',
    monthly_email_limit: 0,
    max_requests_per_second: 0,
    max_requests_per_minute: 0,
    max_requests_per_hour: 0,
    max_requests_per_day: 0,
    used_this_period: 0,
    default_webhook_url: 'example',
    result_retention_days: 0,
    created_at: 'example',
    updated_at: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
