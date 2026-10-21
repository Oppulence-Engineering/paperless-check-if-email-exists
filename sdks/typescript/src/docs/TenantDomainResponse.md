# TenantDomainResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **string** |  | [required]
**domain** | **string** |  | [required]
**id** | **string** |  | [required]
**is_active** | **boolean** |  | [required]
**is_verified** | **boolean** |  | [required]
**notes** | **string** |  | [optional]
**tenant_id** | **string** |  | [required]
**updated_at** | **string** |  | [required]
**verification_record** | **string** |  | [required]
**verification_value** | **string** |  | [required]

## Example

```typescript
import { TenantDomainResponse } from '@oppulence/reacher-sdk';

const instance: TenantDomainResponse = {
    created_at: 'example',
    domain: 'example',
    id: 'example',
    is_active: true,
    is_verified: true,
    notes: 'example',
    tenant_id: 'example',
    updated_at: 'example',
    verification_record: 'example',
    verification_value: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
