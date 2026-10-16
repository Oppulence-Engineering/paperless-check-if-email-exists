# TenantSettingsResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**approval_excellent_threshold** | **number** |  | [required]
**approval_good_threshold** | **number** |  | [required]
**default_policy_mode** | **string** |  | [required]
**default_webhook_url** | **string** |  | [optional]
**monthly_email_limit** | **number** |  | [optional]
**name** | **string** |  | [required]
**period_reset_at** | **string** |  | [required]
**result_retention_days** | **number** |  | [required]
**slug** | **string** |  | [required]
**tenant_id** | **string** |  | [required]
**used_this_period** | **number** |  | [required]

## Example

```typescript
import { TenantSettingsResponse } from '@oppulence/reacher-sdk';

const instance: TenantSettingsResponse = {
    approval_excellent_threshold: 0,
    approval_good_threshold: 0,
    default_policy_mode: 'example',
    default_webhook_url: 'example',
    monthly_email_limit: 0,
    name: 'Weekly Cleanup',
    period_reset_at: 'example',
    result_retention_days: 0,
    slug: 'example',
    tenant_id: 'example',
    used_this_period: 0,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
