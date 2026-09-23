# AdminTenantQuota


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tenant_id** | **string** |  | [optional]
**name** | **string** |  | [optional]
**monthly_email_limit** | **number** |  | [optional]
**used_this_period** | **number** |  | [optional]
**period_reset_at** | **string** |  | [optional]
**quota_unlimited** | **boolean** |  | [optional]
**remaining_quota** | **number** |  | [optional]

## Example

```typescript
import { AdminTenantQuota } from '@oppulence/reacher-sdk';

const instance: AdminTenantQuota = {
    tenant_id: 'example',
    name: 'Weekly Cleanup',
    monthly_email_limit: 0,
    used_this_period: 0,
    period_reset_at: 'example',
    quota_unlimited: true,
    remaining_quota: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
