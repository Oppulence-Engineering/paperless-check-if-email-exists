# TenantApi

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**v1ClearTenantWebhook**](#v1cleartenantwebhook) | **DELETE** /v1/me/webhook | DELETE /v1/me/webhook|
|[**v1CreateTenantDomain**](#v1createtenantdomain) | **POST** /v1/me/domains | POST /v1/me/domains|
|[**v1DeleteTenantDomain**](#v1deletetenantdomain) | **DELETE** /v1/me/domains/{domain} | DELETE /v1/me/domains/{domain}|
|[**v1GetTenantDomain**](#v1gettenantdomain) | **GET** /v1/me/domains/{domain} | GET /v1/me/domains/{domain}|
|[**v1GetTenantSettings**](#v1gettenantsettings) | **GET** /v1/me/settings | GET /v1/me/settings|
|[**v1GetTenantUsage**](#v1gettenantusage) | **GET** /v1/me/usage | GET /v1/me/usage|
|[**v1GetTenantWebhook**](#v1gettenantwebhook) | **GET** /v1/me/webhook | GET /v1/me/webhook|
|[**v1ListTenantDomains**](#v1listtenantdomains) | **GET** /v1/me/domains | GET /v1/me/domains|
|[**v1UpdateTenantDomain**](#v1updatetenantdomain) | **PATCH** /v1/me/domains/{domain} | PATCH /v1/me/domains/{domain}|
|[**v1UpdateTenantSettings**](#v1updatetenantsettings) | **PATCH** /v1/me/settings | PATCH /v1/me/settings|
|[**v1UpdateTenantWebhook**](#v1updatetenantwebhook) | **PATCH** /v1/me/webhook | PATCH /v1/me/webhook|

# **v1ClearTenantWebhook**
> v1ClearTenantWebhook()

Clear tenant webhook URL and signing secret.

### Example

```typescript
import {
    TenantApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new TenantApi(configuration);

const { status, data } = await apiInstance.v1ClearTenantWebhook();
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
|**200** | Tenant webhook cleared |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1CreateTenantDomain**
> v1CreateTenantDomain()

Add a domain for the authenticated tenant.

### Example

```typescript
import {
    TenantApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new TenantApi(configuration);

const { status, data } = await apiInstance.v1CreateTenantDomain();
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
|**201** | Tenant domain created |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1DeleteTenantDomain**
> v1DeleteTenantDomain()

Remove a domain from the authenticated tenant.

### Example

```typescript
import {
    TenantApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new TenantApi(configuration);

let domain: string; //Domain identifier (default to undefined)

const { status, data } = await apiInstance.v1DeleteTenantDomain(
    domain
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **domain** | [**string**] | Domain identifier | defaults to undefined|


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
|**200** | Tenant domain deleted |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1GetTenantDomain**
> v1GetTenantDomain()

Get one domain entry for the authenticated tenant.

### Example

```typescript
import {
    TenantApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new TenantApi(configuration);

let domain: string; //Domain identifier (default to undefined)

const { status, data } = await apiInstance.v1GetTenantDomain(
    domain
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **domain** | [**string**] | Domain identifier | defaults to undefined|


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
|**200** | Tenant domain details |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1GetTenantSettings**
> v1GetTenantSettings()

Get tenant runtime settings and default operational behavior.

### Example

```typescript
import {
    TenantApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new TenantApi(configuration);

const { status, data } = await apiInstance.v1GetTenantSettings();
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
|**200** | Tenant settings |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1GetTenantUsage**
> v1GetTenantUsage()

Return current tenant usage and quota summary.

### Example

```typescript
import {
    TenantApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new TenantApi(configuration);

const { status, data } = await apiInstance.v1GetTenantUsage();
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
|**200** | Tenant usage summary |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1GetTenantWebhook**
> v1GetTenantWebhook()

Get masked tenant webhook integration state.

### Example

```typescript
import {
    TenantApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new TenantApi(configuration);

const { status, data } = await apiInstance.v1GetTenantWebhook();
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
|**200** | Tenant webhook state |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1ListTenantDomains**
> v1ListTenantDomains()

List all active and inactive domain entries for the authenticated tenant.

### Example

```typescript
import {
    TenantApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new TenantApi(configuration);

const { status, data } = await apiInstance.v1ListTenantDomains();
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
|**200** | Tenant domains list |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1UpdateTenantDomain**
> v1UpdateTenantDomain()

Update the domain value, status, verification state, or metadata notes.

### Example

```typescript
import {
    TenantApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new TenantApi(configuration);

let domain: string; //Domain identifier (default to undefined)

const { status, data } = await apiInstance.v1UpdateTenantDomain(
    domain
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **domain** | [**string**] | Domain identifier | defaults to undefined|


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
|**200** | Tenant domain updated |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1UpdateTenantSettings**
> v1UpdateTenantSettings()

Update tenant settings such as retention, default webhook URL, or secret.

### Example

```typescript
import {
    TenantApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new TenantApi(configuration);

const { status, data } = await apiInstance.v1UpdateTenantSettings();
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
|**200** | Tenant settings updated |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1UpdateTenantWebhook**
> v1UpdateTenantWebhook()

Update tenant webhook URL and signing secret.

### Example

```typescript
import {
    TenantApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new TenantApi(configuration);

const { status, data } = await apiInstance.v1UpdateTenantWebhook();
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
|**200** | Tenant webhook updated |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

