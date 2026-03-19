# AdminApi

All URIs are relative to *https://api.reacher.email*

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
> createApiKey()

Create a new API key for a tenant.

### Example

```typescript
import {
    AdminApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let tenantId: string; //Tenant identifier (default to undefined)

const { status, data } = await apiInstance.createApiKey(
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

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | API key created |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createTenant**
> createTenant()

Create a new tenant.

### Example

```typescript
import {
    AdminApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

const { status, data } = await apiInstance.createTenant();
```

### Parameters
This endpoint does not have any parameters.


### Return type

void (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Tenant created |  -  |

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

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Tenant deleted |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getApiKey**
> getApiKey()

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

void (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | API key details |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTenant**
> getTenant()

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

void (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Tenant details |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTenantQuota**
> getTenantQuota()

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

void (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Tenant quota details |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listAllApiKeys**
> listAllApiKeys()

List all API keys across tenants with optional filtering.

### Example

```typescript
import {
    AdminApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

const { status, data } = await apiInstance.listAllApiKeys();
```

### Parameters
This endpoint does not have any parameters.


### Return type

void (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | All API keys |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listApiKeys**
> listApiKeys()

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

void (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Tenant API keys |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listTenants**
> listTenants()

List tenants with optional status and pagination filters.

### Example

```typescript
import {
    AdminApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

const { status, data } = await apiInstance.listTenants();
```

### Parameters
This endpoint does not have any parameters.


### Return type

void (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Tenant list |  -  |

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

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | API key reactivated |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **resetTenantQuota**
> resetTenantQuota()

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

void (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Quota reset |  -  |

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

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | API key revoked |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateApiKey**
> updateApiKey()

Update metadata for an API key.

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

const { status, data } = await apiInstance.updateApiKey(
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

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | API key updated |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateTenant**
> updateTenant()

Update tenant fields.

### Example

```typescript
import {
    AdminApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let tenantId: string; //Tenant identifier (default to undefined)

const { status, data } = await apiInstance.updateTenant(
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

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Tenant updated |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateTenantQuota**
> updateTenantQuota()

Update tenant quota limit.

### Example

```typescript
import {
    AdminApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminApi(configuration);

let tenantId: string; //Tenant identifier (default to undefined)

const { status, data } = await apiInstance.updateTenantQuota(
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

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Quota updated |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

