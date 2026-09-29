# AdminApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createApiKey**](#createapikey) | **POST** /v1/admin/tenants/{tenant_id}/api-keys | POST /v1/admin/tenants/{tenant_id}/api-keys|
|[**createTenant**](#createtenant) | **POST** /v1/admin/tenants | POST /v1/admin/tenants|
|[**deleteTenant**](#deletetenant) | **DELETE** /v1/admin/tenants/{tenant_id} | DELETE /v1/admin/tenants/{tenant_id}|
|[**getApiKey**](#getapikey) | **GET** /v1/admin/tenants/{tenant_id}/api-keys/{key_id} | GET /v1/admin/tenants/{tenant_id}/api-keys/{key_id}|
|[**getTenant**](#gettenant) | **GET** /v1/admin/tenants/{tenant_id} | GET /v1/admin/tenants/{tenant_id}|
|[**getTenantQuota**](#gettenantquota) | **GET** /v1/admin/tenants/{tenant_id}/quota | GET /v1/admin/tenants/{tenant_id}/quota|
|[**listAllApiKeys**](#listallapikeys) | **GET** /v1/admin/api-keys | GET /v1/admin/api-keys|
|[**listApiKeys**](#listapikeys) | **GET** /v1/admin/tenants/{tenant_id}/api-keys | GET /v1/admin/tenants/{tenant_id}/api-keys|
|[**listTenants**](#listtenants) | **GET** /v1/admin/tenants | GET /v1/admin/tenants|
|[**reactivateApiKey**](#reactivateapikey) | **POST** /v1/admin/tenants/{tenant_id}/api-keys/{key_id}/reactivate | POST /v1/admin/tenants/{tenant_id}/api-keys/{key_id}/reactivate|
|[**resetTenantQuota**](#resettenantquota) | **POST** /v1/admin/tenants/{tenant_id}/quota/reset | POST /v1/admin/tenants/{tenant_id}/quota/reset|
|[**revokeApiKey**](#revokeapikey) | **DELETE** /v1/admin/tenants/{tenant_id}/api-keys/{key_id} | DELETE /v1/admin/tenants/{tenant_id}/api-keys/{key_id}|
|[**updateApiKey**](#updateapikey) | **PATCH** /v1/admin/tenants/{tenant_id}/api-keys/{key_id} | PATCH /v1/admin/tenants/{tenant_id}/api-keys/{key_id}|
|[**updateTenant**](#updatetenant) | **PUT** /v1/admin/tenants/{tenant_id} | PUT /v1/admin/tenants/{tenant_id}|
|[**updateTenantQuota**](#updatetenantquota) | **PATCH** /v1/admin/tenants/{tenant_id}/quota | PATCH /v1/admin/tenants/{tenant_id}/quota|

# **createApiKey**
> AdminCreatedApiKey createApiKey(adminApiKeyWriteRequest)

Create a new API key for a tenant.

### Example

```typescript
import {
    AdminApi,
    Configuration,
    AdminApiKeyWriteRequest
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let tenantId: string; //Tenant identifier (default to undefined)
let adminApiKeyWriteRequest: AdminApiKeyWriteRequest; //

const { status, data } = await apiInstance.createApiKey(
    tenantId,
    adminApiKeyWriteRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **adminApiKeyWriteRequest** | **AdminApiKeyWriteRequest**|  | |
| **tenantId** | [**string**] | Tenant identifier | defaults to undefined|


### Return type

**AdminCreatedApiKey**

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | API key created |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createTenant**
> AdminTenant createTenant(adminCreateTenantRequest)

Create a new tenant.

### Example

```typescript
import {
    AdminApi,
    Configuration,
    AdminCreateTenantRequest
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let adminCreateTenantRequest: AdminCreateTenantRequest; //

const { status, data } = await apiInstance.createTenant(
    adminCreateTenantRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **adminCreateTenantRequest** | **AdminCreateTenantRequest**|  | |


### Return type

**AdminTenant**

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Tenant created |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteTenant**
> deleteTenant()

Delete a tenant.

### Example

```typescript
import {
    AdminApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let tenantId: string; //Tenant identifier (default to undefined)

const { status, data } = await apiInstance.deleteTenant(
    tenantId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **tenantId** | [**string**] | Tenant identifier | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Tenant deleted |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getApiKey**
> AdminApiKey getApiKey()

Fetch one API key by tenant and key ID.

### Example

```typescript
import {
    AdminApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let tenantId: string; //Tenant identifier (default to undefined)
let keyId: string; //API key identifier (default to undefined)

const { status, data } = await apiInstance.getApiKey(
    tenantId,
    keyId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **tenantId** | [**string**] | Tenant identifier | defaults to undefined|
| **keyId** | [**string**] | API key identifier | defaults to undefined|


### Return type

**AdminApiKey**

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | API key details |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTenant**
> AdminTenant getTenant()

Fetch tenant details by tenant ID.

### Example

```typescript
import {
    AdminApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let tenantId: string; //Tenant identifier (default to undefined)

const { status, data } = await apiInstance.getTenant(
    tenantId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **tenantId** | [**string**] | Tenant identifier | defaults to undefined|


### Return type

**AdminTenant**

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Tenant details |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTenantQuota**
> AdminTenantQuota getTenantQuota()

Fetch current tenant quota usage and remaining allowance.

### Example

```typescript
import {
    AdminApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let tenantId: string; //Tenant identifier (default to undefined)

const { status, data } = await apiInstance.getTenantQuota(
    tenantId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **tenantId** | [**string**] | Tenant identifier | defaults to undefined|


### Return type

**AdminTenantQuota**

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Tenant quota details |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listAllApiKeys**
> AdminAllApiKeys listAllApiKeys()

List all API keys across tenants with optional filtering.

### Example

```typescript
import {
    AdminApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let tenantId: string; // (optional) (default to undefined)
let status: string; // (optional) (default to undefined)
let limit: number; // (optional) (default to undefined)
let offset: number; // (optional) (default to undefined)

const { status, data } = await apiInstance.listAllApiKeys(
    tenantId,
    status,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **tenantId** | [**string**] |  | (optional) defaults to undefined|
| **status** | [**string**] |  | (optional) defaults to undefined|
| **limit** | [**number**] |  | (optional) defaults to undefined|
| **offset** | [**number**] |  | (optional) defaults to undefined|


### Return type

**AdminAllApiKeys**

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | All API keys |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listApiKeys**
> AdminApiKeyList listApiKeys()

List API keys for a tenant.

### Example

```typescript
import {
    AdminApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let tenantId: string; //Tenant identifier (default to undefined)

const { status, data } = await apiInstance.listApiKeys(
    tenantId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **tenantId** | [**string**] | Tenant identifier | defaults to undefined|


### Return type

**AdminApiKeyList**

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Tenant API keys |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listTenants**
> AdminTenantList listTenants()

List tenants with optional status and pagination filters.

### Example

```typescript
import {
    AdminApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let status: string; // (optional) (default to undefined)
let limit: number; // (optional) (default to undefined)
let offset: number; // (optional) (default to undefined)

const { status, data } = await apiInstance.listTenants(
    status,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **status** | [**string**] |  | (optional) defaults to undefined|
| **limit** | [**number**] |  | (optional) defaults to undefined|
| **offset** | [**number**] |  | (optional) defaults to undefined|


### Return type

**AdminTenantList**

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Tenant list |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **reactivateApiKey**
> reactivateApiKey()

Reactivate a previously revoked API key.

### Example

```typescript
import {
    AdminApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let tenantId: string; //Tenant identifier (default to undefined)
let keyId: string; //API key identifier (default to undefined)

const { status, data } = await apiInstance.reactivateApiKey(
    tenantId,
    keyId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **tenantId** | [**string**] | Tenant identifier | defaults to undefined|
| **keyId** | [**string**] | API key identifier | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | API key reactivated |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **resetTenantQuota**
> AdminTenantQuota resetTenantQuota()

Reset tenant quota usage counters.

### Example

```typescript
import {
    AdminApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let tenantId: string; //Tenant identifier (default to undefined)

const { status, data } = await apiInstance.resetTenantQuota(
    tenantId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **tenantId** | [**string**] | Tenant identifier | defaults to undefined|


### Return type

**AdminTenantQuota**

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Quota reset |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **revokeApiKey**
> revokeApiKey()

Revoke an API key.

### Example

```typescript
import {
    AdminApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let tenantId: string; //Tenant identifier (default to undefined)
let keyId: string; //API key identifier (default to undefined)

const { status, data } = await apiInstance.revokeApiKey(
    tenantId,
    keyId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **tenantId** | [**string**] | Tenant identifier | defaults to undefined|
| **keyId** | [**string**] | API key identifier | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | API key revoked |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateApiKey**
> AdminApiKey updateApiKey(adminApiKeyWriteRequest)

Update metadata for an API key.

### Example

```typescript
import {
    AdminApi,
    Configuration,
    AdminApiKeyWriteRequest
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let tenantId: string; //Tenant identifier (default to undefined)
let keyId: string; //API key identifier (default to undefined)
let adminApiKeyWriteRequest: AdminApiKeyWriteRequest; //

const { status, data } = await apiInstance.updateApiKey(
    tenantId,
    keyId,
    adminApiKeyWriteRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **adminApiKeyWriteRequest** | **AdminApiKeyWriteRequest**|  | |
| **tenantId** | [**string**] | Tenant identifier | defaults to undefined|
| **keyId** | [**string**] | API key identifier | defaults to undefined|


### Return type

**AdminApiKey**

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | API key updated |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateTenant**
> AdminTenant updateTenant(adminUpdateTenantRequest)

Update tenant fields.

### Example

```typescript
import {
    AdminApi,
    Configuration,
    AdminUpdateTenantRequest
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let tenantId: string; //Tenant identifier (default to undefined)
let adminUpdateTenantRequest: AdminUpdateTenantRequest; //

const { status, data } = await apiInstance.updateTenant(
    tenantId,
    adminUpdateTenantRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **adminUpdateTenantRequest** | **AdminUpdateTenantRequest**|  | |
| **tenantId** | [**string**] | Tenant identifier | defaults to undefined|


### Return type

**AdminTenant**

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Tenant updated |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateTenantQuota**
> AdminTenantQuota updateTenantQuota(adminUpdateQuotaRequest)

Update tenant quota limit.

### Example

```typescript
import {
    AdminApi,
    Configuration,
    AdminUpdateQuotaRequest
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let tenantId: string; //Tenant identifier (default to undefined)
let adminUpdateQuotaRequest: AdminUpdateQuotaRequest; //

const { status, data } = await apiInstance.updateTenantQuota(
    tenantId,
    adminUpdateQuotaRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **adminUpdateQuotaRequest** | **AdminUpdateQuotaRequest**|  | |
| **tenantId** | [**string**] | Tenant identifier | defaults to undefined|


### Return type

**AdminTenantQuota**

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Quota updated |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
