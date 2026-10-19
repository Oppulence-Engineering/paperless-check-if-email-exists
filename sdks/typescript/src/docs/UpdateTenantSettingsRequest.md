# UpdateTenantSettingsRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**approval_excellent_threshold** | **number** | Safe-to-send percentage for an excellent list. Set it with &#x60;approval_good_threshold&#x60;. | [optional]
**approval_good_threshold** | **number** | Safe-to-send percentage for a good list. Must be below the excellent threshold. | [optional]
**default_policy_mode** | **string** |  | [optional]
**default_webhook_url** | **string** |  | [optional]
**result_retention_days** | **number** |  | [optional]
**webhook_signing_secret** | **string** |  | [optional]

## Example

```typescript
import { UpdateTenantSettingsRequest } from '@oppulence/reacher-sdk';

const instance: UpdateTenantSettingsRequest = {
    approval_excellent_threshold: 0,
    approval_good_threshold: 0,
    default_policy_mode: 'example',
    default_webhook_url: 'example',
    result_retention_days: 0,
    webhook_signing_secret: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
