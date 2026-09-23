# UpdateTenantSettingsRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_policy_mode** | **string** |  | [optional]
**default_webhook_url** | **string** |  | [optional]
**result_retention_days** | **number** |  | [optional]
**webhook_signing_secret** | **string** |  | [optional]

## Example

```typescript
import { UpdateTenantSettingsRequest } from '@oppulence/reacher-sdk';

const instance: UpdateTenantSettingsRequest = {
    default_policy_mode: 'example',
    default_webhook_url: 'example',
    result_retention_days: 0,
    webhook_signing_secret: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
