# ListsApi

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**v1ListQuality**](#v1listquality) | **GET** /v1/lists/{list_id}/quality | GET /v1/lists/{list_id}/quality|
|[**v1ListsListIdRemediationExportsExportIdDownloadGet**](#v1listslistidremediationexportsexportiddownloadget) | **GET** /v1/lists/{list_id}/remediation-exports/{export_id}/download | Download remediation export|
|[**v1ListsListIdRemediationExportsPost**](#v1listslistidremediationexportspost) | **POST** /v1/lists/{list_id}/remediation-exports | Create remediation export|
|[**v1ListsListIdRemediationPlanGet**](#v1listslistidremediationplanget) | **GET** /v1/lists/{list_id}/remediation-plan | Get remediation plan|
|[**v1ListsListIdRemediationPlanPost**](#v1listslistidremediationplanpost) | **POST** /v1/lists/{list_id}/remediation-plan | Create remediation plan|

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
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List quality benchmark report |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1ListsListIdRemediationExportsExportIdDownloadGet**
> File v1ListsListIdRemediationExportsExportIdDownloadGet()


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

const { status, data } = await apiInstance.v1ListsListIdRemediationExportsExportIdDownloadGet(
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
 - **Accept**: text/csv


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Remediation export CSV |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1ListsListIdRemediationExportsPost**
> { [key: string]: any; } v1ListsListIdRemediationExportsPost(requestBody)


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

const { status, data } = await apiInstance.v1ListsListIdRemediationExportsPost(
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

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1ListsListIdRemediationPlanGet**
> { [key: string]: any; } v1ListsListIdRemediationPlanGet()


### Example

```typescript
import {
    ListsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new ListsApi(configuration);

let listId: number; // (default to undefined)

const { status, data } = await apiInstance.v1ListsListIdRemediationPlanGet(
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

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1ListsListIdRemediationPlanPost**
> { [key: string]: any; } v1ListsListIdRemediationPlanPost(requestBody)


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

const { status, data } = await apiInstance.v1ListsListIdRemediationPlanPost(
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

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
