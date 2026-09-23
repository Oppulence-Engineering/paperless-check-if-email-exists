# ListsApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**v1CreateRemediationExport**](#v1createremediationexport) | **POST** /v1/lists/{list_id}/remediation-exports | Create remediation export|
|[**v1CreateRemediationPlan**](#v1createremediationplan) | **POST** /v1/lists/{list_id}/remediation-plan | Create remediation plan|
|[**v1DownloadRemediationExport**](#v1downloadremediationexport) | **GET** /v1/lists/{list_id}/remediation-exports/{export_id}/download | Download remediation export|
|[**v1GetRemediationPlan**](#v1getremediationplan) | **GET** /v1/lists/{list_id}/remediation-plan | Get remediation plan|
|[**v1ListQuality**](#v1listquality) | **GET** /v1/lists/{list_id}/quality | GET /v1/lists/{list_id}/quality|

# **v1CreateRemediationExport**
> { [key: string]: any; } v1CreateRemediationExport(requestBody)


### Example

```typescript
import {
    ListsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new ListsApi(configuration);

let listId: number; // (default to undefined)
let requestBody: { [key: string]: any; }; //

const { status, data } = await apiInstance.v1CreateRemediationExport(
    listId,
    requestBody
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **requestBody** | **{ [key: string]: any; }**|  | |
| **listId** | [**number**] |  | defaults to undefined|


### Return type

**{ [key: string]: any; }**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Remediation export |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1CreateRemediationPlan**
> { [key: string]: any; } v1CreateRemediationPlan(requestBody)


### Example

```typescript
import {
    ListsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new ListsApi(configuration);

let listId: number; // (default to undefined)
let requestBody: { [key: string]: any; }; //

const { status, data } = await apiInstance.v1CreateRemediationPlan(
    listId,
    requestBody
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **requestBody** | **{ [key: string]: any; }**|  | |
| **listId** | [**number**] |  | defaults to undefined|


### Return type

**{ [key: string]: any; }**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Remediation plan |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1DownloadRemediationExport**
> File v1DownloadRemediationExport()


### Example

```typescript
import {
    ListsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new ListsApi(configuration);

let listId: number; // (default to undefined)
let exportId: number; // (default to undefined)

const { status, data } = await apiInstance.v1DownloadRemediationExport(
    listId,
    exportId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **listId** | [**number**] |  | defaults to undefined|
| **exportId** | [**number**] |  | defaults to undefined|


### Return type

**File**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/csv, application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Remediation export CSV |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1GetRemediationPlan**
> { [key: string]: any; } v1GetRemediationPlan()


### Example

```typescript
import {
    ListsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new ListsApi(configuration);

let listId: number; // (default to undefined)

const { status, data } = await apiInstance.v1GetRemediationPlan(
    listId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **listId** | [**number**] |  | defaults to undefined|


### Return type

**{ [key: string]: any; }**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Remediation plan |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1ListQuality**
> v1ListQuality()

Returns a quality benchmark report for a list.

### Example

```typescript
import {
    ListsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new ListsApi(configuration);

let listId: number; //List identifier (default to undefined)

const { status, data } = await apiInstance.v1ListQuality(
    listId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **listId** | [**number**] | List identifier | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List quality benchmark report |  -  |
|**0** | Request error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
