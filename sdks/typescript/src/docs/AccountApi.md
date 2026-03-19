# AccountApi

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createTenantApiKey**](#createtenantapikey) | **POST** /v1/me/api-keys | POST /v1/me/api-keys|
|[**getTenantApiKey**](#gettenantapikey) | **GET** /v1/me/api-keys/{key_id} | GET /v1/me/api-keys/{key_id}|
|[**listTenantApiKeys**](#listtenantapikeys) | **GET** /v1/me/api-keys | GET /v1/me/api-keys|
|[**revokeTenantApiKey**](#revoketenantapikey) | **DELETE** /v1/me/api-keys/{key_id} | DELETE /v1/me/api-keys/{key_id}|
|[**updateTenantApiKey**](#updatetenantapikey) | **PATCH** /v1/me/api-keys/{key_id} | PATCH /v1/me/api-keys/{key_id}|
|[**v1Me**](#v1me) | **GET** /v1/me | GET /v1/me|

# **createTenantApiKey**
> createTenantApiKey()

Create a new API key for the authenticated tenant.

### Example

```typescript
import {
    AccountApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AccountApi(configuration);

const { status, data } = await apiInstance.createTenantApiKey();
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
|**201** | API key created |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTenantApiKey**
> getTenantApiKey()

Return a single API key for the authenticated tenant.

### Example

```typescript
import {
    AccountApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AccountApi(configuration);

let keyId: string; //API key identifier (default to undefined)

const { status, data } = await apiInstance.getTenantApiKey(
    keyId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
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

# **listTenantApiKeys**
> listTenantApiKeys()

List API keys for the authenticated tenant.

### Example

```typescript
import {
    AccountApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AccountApi(configuration);

const { status, data } = await apiInstance.listTenantApiKeys();
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
|**200** | API key list |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **revokeTenantApiKey**
> revokeTenantApiKey()

Revoke an API key.

### Example

```typescript
import {
    AccountApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AccountApi(configuration);

let keyId: string; //API key identifier (default to undefined)

const { status, data } = await apiInstance.revokeTenantApiKey(
    keyId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
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

# **updateTenantApiKey**
> updateTenantApiKey()

Update metadata for an existing API key.

### Example

```typescript
import {
    AccountApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AccountApi(configuration);

let keyId: string; //API key identifier (default to undefined)

const { status, data } = await apiInstance.updateTenantApiKey(
    keyId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
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

# **v1Me**
> v1Me()

Returns the authenticated tenant context and quota metadata.

### Example

```typescript
import {
    AccountApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AccountApi(configuration);

const { status, data } = await apiInstance.v1Me();
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
|**200** | Current tenant profile |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

