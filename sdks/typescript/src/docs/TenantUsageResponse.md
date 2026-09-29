# TenantUsageResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**monthly_email_limit** | **number** |  | [optional]
**period_reset_at** | **string** |  | [required]
**plan_tier** | **string** |  | [required]
**quota_remaining** | **number** |  | [optional]
**quota_unlimited** | **boolean** |  | [required]
**tenant_id** | **string** |  | [required]
**tenant_name** | **string** |  | [required]
**used_this_period** | **number** |  | [required]

## Example

```typescript
import { TenantUsageResponse } from '@oppulence/reacher-sdk';

const instance: TenantUsageResponse = {
    monthly_email_limit: 0,
    period_reset_at: 'example',
    plan_tier: 'example',
    quota_remaining: 0,
    quota_unlimited: true,
    tenant_id: 'example',
    tenant_name: 'example',
    used_this_period: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
