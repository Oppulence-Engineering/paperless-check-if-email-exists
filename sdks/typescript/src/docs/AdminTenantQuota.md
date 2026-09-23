# AdminTenantQuota


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**monthly_email_limit** | **number** |  | [optional]
**name** | **string** |  | [optional]
**period_reset_at** | **string** |  | [optional]
**quota_unlimited** | **boolean** |  | [optional]
**remaining_quota** | **number** |  | [optional]
**tenant_id** | **string** |  | [optional]
**used_this_period** | **number** |  | [optional]

## Example

```typescript
import { AdminTenantQuota } from '@oppulence/reacher-sdk';

const instance: AdminTenantQuota = {
    monthly_email_limit: 0,
    name: 'Weekly Cleanup',
    period_reset_at: 'example',
    quota_unlimited: true,
    remaining_quota: 0,
    tenant_id: 'example',
    used_this_period: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
